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

   pub struct ControlRegister: u8 {
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
    pub fn new() -> Self {
        ControlRegister::from_bits_truncate(0b00000000)
    }

    pub fn vram_address_increment(&self) -> u8 {
        if self.contains(ControlRegister::VRAM_ADD_INCREMENT) {
            32
        } else {
            1
        }
    }

    pub fn update(&mut self, value: u8) {
        self.bits = value;
    }
}

pub struct AddressRegister {
    low: u8,
    high: u8,
    high_pointer: bool,
}

impl AddressRegister {
    pub fn new() -> Self {
        Self {
            low: 0,
            high: 0,
            high_pointer: true,
        }
    }

    pub fn get(&self) -> u16 {
        let high_u16 = self.high as u16;
        let low_u16 = self.low as u16;

        (high_u16 << 8) | low_u16
    }

    pub fn set(&mut self, value: u16) {
        self.high = (value >> 8) as u8;
        self.low = (value & 0xff) as u8;
    }

    pub fn update(&mut self, value: u8) {
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

    pub fn increment(&mut self, value: u8) {
        let old_low = self.low;
        self.low = self.low.wrapping_add(value);

        if old_low > self.low {
            self.high = self.high.wrapping_add(1);
        }

        if self.get() > 0x3fff {
            self.set(self.get() & 0b11111111111111);
        }
    }

    pub fn reset_latch(&mut self) {
        self.high_pointer = true;
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
    internal_data_buffer: u8,
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
            internal_data_buffer: 0,
        }
    }

    fn write_in_ppu_address(&mut self, value: u8) {
        self.address.update(value);
    }

    fn write_in_control(&mut self, value: u8) {
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

    fn read_data(&mut self) -> u8 {
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
}
