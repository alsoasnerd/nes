use super::cartridges::ROM;
use super::ppu::PPU;
use super::ram::RAM;

const RAM_START: u16 = 0x0000;
const RAM_END: u16 = 0x1FFF;
const PPU_REGISTERS_START: u16 = 0x2000;
const PPU_REGISTERS_END: u16 = 0x3FFF;

pub struct BUS {
    ram: RAM,
    prg_rom: Vec<u8>,
    ppu: PPU,
    cycles: usize,
}

impl BUS {
    pub fn new(rom: ROM) -> Self {
        let ppu = PPU::new(rom.chr_rom, rom.screen_mirroring);

        Self {
            ram: RAM::new(),
            prg_rom: rom.prg_rom,
            ppu,
            cycles: 0,
        }
    }

    pub fn memory_read(&mut self, address: u16) -> u8 {
        match address {
            RAM_START..=RAM_END => {
                let adjusted_address = address & 0b00000111_11111111;
                self.ram.read(adjusted_address)
            }

            PPU_REGISTERS_START | 0x2001 | 0x2003 | 0x2005 | 0x2006 | 0x4014 => {
                panic!("Attempt to read from write-only PPU address {:x}", address);
            }

            0x2007 => self.ppu.read_data(),

            0x2008..=PPU_REGISTERS_END => {
                let adjusted_address = address & 0b00100000_00000111;
                self.memory_read(adjusted_address)
            }

            0x8000..=0xFFFF => self.read_prg_rom(address),

            _ => {
                println!("Ignoring memory access at {}", address);
                0
            }
        }
    }

    pub fn memory_write(&mut self, address: u16, data: u8) {
        match address {
            RAM_START..=RAM_END => {
                let adjusted_address = address & 0b11111111111;
                self.ram.write(adjusted_address, data);
            }

            PPU_REGISTERS_START => self.ppu.write_in_control(data),

            0x2006 => self.ppu.write_in_ppu_address(data),

            0x2007 => self.ppu.write_in_data(data),

            0x2008..=PPU_REGISTERS_END => {
                let adjusted_address = address & 0b00100000_00000111;
                self.memory_write(adjusted_address, data);
            }

            0x8000..=0xFFFF => {
                panic!("Attempt to write to Cartridge ROM space")
            }

            _ => {
                println!("Ignoring memory write-access at {}", address);
            }
        }
    }

    pub fn memory_read_u16(&mut self, address: u16) -> u16 {
        let low = self.memory_read(address) as u16;
        let high = self.memory_read(address + 1) as u16;

        (high << 8) | (low as u16)
    }

    pub fn memory_write_u16(&mut self, address: u16, value: u16) {
        let high = (value >> 8) as u8;
        let low = (value & 0xff) as u8;

        self.memory_write(address, low);
        self.memory_write(address + 1, high);
    }

    pub fn read_prg_rom(&self, mut address: u16) -> u8 {
        address -= 0x8000;

        if self.prg_rom.len() == 0x4000 && address >= 0x4000 {
            address %= 0x4000;
        }

        self.prg_rom[address as usize]
    }

    pub fn tick(&mut self, cycles: u8) {
        self.cycles += cycles as usize;
        self.ppu.tick(cycles * 3);
    }

    pub fn pool_nmi_status(&mut self) -> Option<u8> {
        self.ppu.pool_nmi_status()
    }
}
