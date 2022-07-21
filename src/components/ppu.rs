use super::cartridges::Mirroring;
use bitflags::bitflags;

bitflags! {

   // 7  bit  0
   // ---- ----
   // VPHB SINN
   // |||| ||||
   // |||| ||++- Base nametable address
   // |||| ||    (0 = $2000; 1 = $2400; 2 = $2800; 3 = $2C00)
   // |||| |+--- VRAM address increment per CPU read/write of PPUDATA
   // |||| |     (0: add 1, going across; 1: add 32, going down)
   // |||| +---- Sprite pattern table address for 8x8 sprites
   // ||||       (0: $0000; 1: $1000; ignored in 8x16 mode)
   // |||+------ Background pattern table address (0: $0000; 1: $1000)
   // ||+------- Sprite size (0: 8x8 pixels; 1: 8x16 pixels)
   // |+-------- PPU master/slave select
   // |          (0: read backdrop from EXT pins; 1: output color on EXT pins)
   // +--------- Generate an NMI at the start of the
   //            vertical blanking interval (0: off; 1: on)

   struct ControlRegister: u8 {
       const NAMETABLE1                = 0b00000001;
       const NAMETABLE2                = 0b00000010;
       const VRAM_ADD_INCREMENT        = 0b00000100;
       const SPRITE_PATTERN_ADDRESS    = 0b00001000;
       const BACKROUND_PATTERN_ADDRESS = 0b00010000;
       const SPRITE_SIZE               = 0b00100000;
       const MASTER_SLAVE_SELECT       = 0b01000000;
       const GENERATE_NMI              = 0b10000000;
   }
}

impl ControlRegister {
    fn new() -> Self {
        ControlRegister::from_bits_truncate(0b00000000)
    }

    fn vram_address_increment(&self) -> u8 {
        if self.contains(ControlRegister::VRAM_ADD_INCREMENT) {
            32
        } else {
            1
        }
    }

    fn update(&mut self, value: u8) {
        self.bits = value;
    }

    fn generate_vblank_nmi(&self) -> bool {
        self.contains(ControlRegister::GENERATE_NMI)
    }
}

struct AddressRegister {
    low: u8,
    high: u8,
    high_pointer: bool,
}

impl AddressRegister {
    fn new() -> Self {
        Self {
            low: 0,
            high: 0,
            high_pointer: true,
        }
    }

    fn get(&self) -> u16 {
        let high_u16 = self.high as u16;
        let low_u16 = self.low as u16;

        (high_u16 << 8) | low_u16
    }

    fn set(&mut self, value: u16) {
        self.high = (value >> 8) as u8;
        self.low = (value & 0xff) as u8;
    }

    fn update(&mut self, value: u8) {
        if self.high_pointer {
            self.high = value;
        } else {
            self.low = value;
        }

        if self.get() > 0x3fff {
            self.set(self.get() & 0b11111111111111);
        }

        self.high_pointer = self.high_pointer;
    }

    fn increment(&mut self, value: u8) {
        let old_low = self.low;
        self.low = self.low.wrapping_add(value);

        if old_low > self.low {
            self.high = self.high.wrapping_add(1);
        }

        if self.get() > 0x3fff {
            self.set(self.get() & 0b11111111111111);
        }
    }

    fn reset_latch(&mut self) {
        self.high_pointer = true;
    }
}

bitflags! {

    // 7  bit  0
    // ---- ----
    // VSO. ....
    // |||| ||||
    // |||+-++++- Least significant bits previously written into a PPU register
    // |||        (due to register not being updated for this address)
    // ||+------- Sprite overflow. The intent was for this flag to be set
    // ||         whenever more than eight sprites appear on a scanline, but a
    // ||         hardware bug causes the actual behavior to be more complicated
    // ||         and generate false positives as well as false negatives; see
    // ||         PPU sprite evaluation. This flag is set during sprite
    // ||         evaluation and cleared at dot 1 (the second dot) of the
    // ||         pre-render line.
    // |+-------- Sprite 0 Hit.  Set when a nonzero pixel of sprite 0 overlaps
    // |          a nonzero background pixel; cleared at dot 1 of the pre-render
    // |          line.  Used for raster timing.
    // +--------- Vertical blank has started (0: not in vblank; 1: in vblank).
    //            Set at dot 1 of line 241 (the line *after* the post-render
    //            line); cleared after reading $2002 and at dot 1 of the
    //            pre-render line.

    struct StatusRegister: u8 {
        const NOT_USED          = 0b0000_0001;
        const NOT_USED2         = 0b0000_0010;
        const NOT_USED3         = 0b0000_0100;
        const NOT_USED4         = 0b0000_1000;
        const NOT_USED5         = 0b0001_0000;
        const SPRITE_OVERFLOW   = 0b0010_0000;
        const SPRITE_ZERO_HIT   = 0b0100_0000;
        const VBLANK_STARTED    = 0b1000_0000;
    }
}

impl StatusRegister {
    fn new() -> Self {
        StatusRegister::from_bits_truncate(0b0000_0000)
    }

    fn set_vblank_status(&mut self, status: bool) {
        self.set(StatusRegister::VBLANK_STARTED, status);
    }

    fn reset_vblank_status(&mut self) {
        self.remove(StatusRegister::VBLANK_STARTED);
    }
}

pub struct PPU {
    pub chr_rom: Vec<u8>,
    pub pallete_table: [u8; 32],
    pub vram: [u8; 2048],
    pub oam_data: [u8; 256],
    pub mirroring: Mirroring,
    address: AddressRegister,
    control: ControlRegister,
    status: StatusRegister,
    internal_data_buffer: u8,
    scanline: u16,
    cycles: usize,
    nmi_interrupt: Option<u8>
}

impl PPU {
    pub fn new(chr_rom: Vec<u8>, mirroring: Mirroring) -> Self {
        Self {
            chr_rom,
            pallete_table: [0; 32],
            vram: [0; 2048],
            oam_data: [0; 64 * 4],
            mirroring,
            address: AddressRegister::new(),
            control: ControlRegister::new(),
            status: StatusRegister::new(),
            internal_data_buffer: 0,
            scanline: 0,
            cycles: 0,
            nmi_interrupt: None
        }
    }

    pub fn write_in_ppu_address(&mut self, value: u8) {
        self.address.update(value);
    }

    pub fn write_in_control(&mut self, value: u8) {
        self.control.update(value);
    }

    fn increment_vram_address(&mut self) {
        self.address
            .increment(self.control.vram_address_increment());
    }

    fn mirror_vram_address(&self, address: u16) -> u16 {
        let mirrored_address = address & 0b10111111111111;
        let vram_index = mirrored_address - 0x2000;
        let name_table = vram_index / 0x400;

        match (&self.mirroring, name_table) {
            (Mirroring::Vertical, 2) | (Mirroring::Vertical, 3) => vram_index - 0x800,
            (Mirroring::Horizontal, 2) => vram_index - 0x400,
            (Mirroring::Horizontal, 1) => vram_index - 0x400,
            (Mirroring::Horizontal, 3) => vram_index - 0x800,
            _ => vram_index,
        }
    }

    pub fn read_data(&mut self) -> u8 {
        let address = self.address.get();
        self.increment_vram_address();

        match address {
            0..=0x1fff => {
                let result = self.internal_data_buffer;
                self.internal_data_buffer = self.chr_rom[address as usize];
                result
            }

            0x2000..=0x2fff => {
                let result = self.internal_data_buffer;
                self.internal_data_buffer = self.vram[self.mirror_vram_address(address) as usize];
                result
            }

            0x3000..=0x3eff => panic!("Read unexpected address: 0x{:02x}", address),
            0x3f00..=0x3fff => self.pallete_table[(address - 0x3f00) as usize],
            _ => panic!("Read unexpected address: 0x{:02x}", address),
        }
    }

    pub fn write_in_data(&mut self, data: u8) {
        let address = self.address.get();

        match address {
            0..=0x1FFF => println!("Attempt to write to chr_rom space: 0x{:02x}", address),
            0x2000..=0x2FFF => self.vram[self.mirror_vram_address(address) as usize] = data,
            0x3000..=0x3EFF => unimplemented!("Address {} should't be used in reallity", address),

            0x3F10 | 0x3F14 | 0x3F18 | 0x3F1C => {
                let add_mirror = address - 0x10;
                self.pallete_table[(add_mirror - 0x3F00) as usize] = data;
            }

            0x3f00..=0x3fff => {
                self.pallete_table[(address - 0x3f00) as usize] = data;
            }

            _ => panic!("Unexpected access to mirrored space {}", address),
        }
        self.increment_vram_address();
    }

    pub fn tick(&mut self, cycles: u8) -> bool {
        self.cycles += cycles as usize;

        if !self.cycles >= 341 {
            return false;
        }

        self.cycles -= 341;
        self.scanline += 1;

        if self.scanline == 241 {
            self.status.set_vblank_status(true);
            if self.control.generate_vblank_nmi() {
                self.nmi_interrupt = Some(1);
            }
        }

        if self.scanline >= 262 {
            self.scanline = 0;
            self.nmi_interrupt = None;
            self.status.reset_vblank_status();
            return true;
        }

        return false;
    }

    pub fn pool_nmi_status(&mut self) -> Option<u8> {
        self.nmi_interrupt.take()
    }
}
