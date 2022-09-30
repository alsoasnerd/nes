use super::cartridge::Rom;
use super::joypads::Joypad;
use super::ppu::PPU;

//  _______________ $10000  _______________
// | PRG-ROM       |       |               |
// | Upper Bank    |       |               |
// |_ _ _ _ _ _ _ _| $C000 | PRG-ROM       |
// | PRG-ROM       |       |               |
// | Lower Bank    |       |               |
// |_______________| $8000 |_______________|
// | SRAM          |       | SRAM          |
// |_______________| $6000 |_______________|
// | Expansion ROM |       | Expansion ROM |
// |_______________| $4020 |_______________|
// | I/O Registers |       |               |
// |_ _ _ _ _ _ _ _| $4000 |               |
// | Mirrors       |       | I/O Registers |
// | $2000-$2007   |       |               |
// |_ _ _ _ _ _ _ _| $2008 |               |
// | I/O Registers |       |               |
// |_______________| $2000 |_______________|
// | Mirrors       |       |               |
// | $0000-$07FF   |       |               |
// |_ _ _ _ _ _ _ _| $0800 |               |
// | RAM           |       | RAM           |
// |_ _ _ _ _ _ _ _| $0200 |               |
// | Stack         |       |               |
// |_ _ _ _ _ _ _ _| $0100 |               |
// | Zero Page     |       |               |
// |_______________| $0000 |_______________|
const RAM: u16 = 0x0000;
const RAM_MIRRORS_END: u16 = 0x1FFF;
const PPU_REGISTERS_MIRRORS_END: u16 = 0x3FFF;

pub struct BUS<'call> {
    cpu_vram: [u8; 2048],
    prg_rom: Vec<u8>,
    ppu: PPU,

    cycles: usize,
    gameloop_callback: Box<dyn FnMut(&PPU, &mut Joypad) + 'call>,
    joypad1: Joypad,
}

impl<'a> BUS<'a> {
    pub fn new<'call, F>(rom: Rom, gameloop_callback: F) -> BUS<'call>
    where
        F: FnMut(&PPU, &mut Joypad) + 'call,
    {
        let ppu = PPU::new(rom.chr_rom, rom.screen_mirroring);

        BUS {
            cpu_vram: [0; 2048],
            prg_rom: rom.prg_rom,
            ppu: ppu,
            cycles: 0,
            gameloop_callback: Box::from(gameloop_callback),
            joypad1: Joypad::new(),
        }
    }

    pub fn memory_read(&mut self, address: u16) -> u8 {
        match address {
            RAM..=RAM_MIRRORS_END => {
                let mirror_down_address = address & 0b00000111_11111111;
                self.cpu_vram[mirror_down_address as usize]
            }
            0x2000 | 0x2001 | 0x2003 | 0x2005 | 0x2006 | 0x4014 => {
                // panic!("Attempt to read from write-only PPU address {:x}", address);
                0
            }
            0x2002 => self.ppu.read_status(),
            0x2004 => self.ppu.read_oam_data(),
            0x2007 => self.ppu.read_data(),

            0x4000..=0x4015 => {
                //ignore APU
                0
            }

            0x4016 => self.joypad1.read(),

            0x4017 => {
                // ignore joypad 2
                0
            }
            0x2008..=PPU_REGISTERS_MIRRORS_END => {
                let mirror_down_address = address & 0b00100000_00000111;
                self.memory_read(mirror_down_address)
            }
            0x8000..=0xFFFF => self.read_prg_rom(address),

            _ => {
                println!("Ignoring memory access at {:x}", address);
                0
            }
        }
    }

    pub fn memory_write(&mut self, address: u16, data: u8) {
        match address {
            RAM..=RAM_MIRRORS_END => {
                let mirror_down_address = address & 0b11111111111;
                self.cpu_vram[mirror_down_address as usize] = data;
            }
            0x2000 => {
                self.ppu.write_to_control(data);
            }
            0x2001 => {
                self.ppu.write_to_mask(data);
            }

            0x2002 => panic!("attempt to write to PPU status register"),

            0x2003 => {
                self.ppu.write_to_oam_address(data);
            }
            0x2004 => {
                self.ppu.write_to_oam_data(data);
            }
            0x2005 => {
                self.ppu.write_to_scroll(data);
            }

            0x2006 => {
                self.ppu.write_to_ppu_address(data);
            }
            0x2007 => {
                self.ppu.write_to_data(data);
            }
            0x4000..=0x4013 | 0x4015 => {
                //ignore APU
            }

            0x4016 => self.joypad1.write(data),

            0x4017 => {
                // ignore joypad 2
            }

            0x4014 => {
                let mut buffer: [u8; 256] = [0; 256];
                let hi: u16 = (data as u16) << 8;
                for i in 0..256u16 {
                    buffer[i as usize] = self.memory_read(hi + i);
                }

                self.ppu.write_oam_dma(&buffer);

                // todo: handle this eventually
                // let add_cycles: u16 = if self.cycles % 2 == 1 { 514 } else { 513 };
                // self.tick(add_cycles); //todo this will cause weird effects as PPU will have 513/514 * 3 ticks
            }

            0x2008..=PPU_REGISTERS_MIRRORS_END => {
                let mirror_down_address = address & 0b00100000_00000111;
                self.memory_write(mirror_down_address, data);
                // todo!("PPU is not supported yet");
            }
            0x8000..=0xFFFF => panic!("Attempt to write to Cartridge ROM space: {:x}", address),

            _ => {
                println!("Ignoring memory write-access at {:x}", address);
            }
        }
    }

    pub fn memory_read_u16(&mut self, address: u16) -> u16 {
        let low = self.memory_read(address) as u16;
        let high = self.memory_read(address + 1) as u16;

        (high << 8) | (low as u16)
    }

    pub fn memory_write_u16(&mut self, pos: u16, data: u16) {
        let high = (data >> 8) as u8;
        let low = (data & 0xff) as u8;

        self.memory_write(pos, low);
        self.memory_write(pos + 1, high);
    }

    fn read_prg_rom(&self, mut address: u16) -> u8 {
        address -= 0x8000;
        if self.prg_rom.len() == 0x4000 && address >= 0x4000 {
            //mirror if needed
            address = address % 0x4000;
        }
        self.prg_rom[address as usize]
    }

    pub fn tick(&mut self, cycles: u8) {
        self.cycles += cycles as usize;
        let new_frame = self.ppu.tick(cycles * 3);
        if new_frame {
            (self.gameloop_callback)(&self.ppu, &mut self.joypad1);
        }
    }

    pub fn poll_nmi_status(&mut self) -> Option<u8> {
        self.ppu.poll_nmi_interrupt()
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::components::cartridge::test;

    #[test]
    fn test_memory_read_write_to_ram() {
        let mut bus = BUS::new(test::test_rom(), |_ppu: &PPU, _joypad: &mut Joypad| {});
        bus.memory_write(0x01, 0x55);
        assert_eq!(bus.memory_read(0x01), 0x55);
    }
}
