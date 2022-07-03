use crate::ram::RAM;

const RAM: u16 = 0x0000;
const RAM_END: u16 = 0x1FFF;
const PPU_REGISTERS: u16 = 0x2000;
const PPU_REGISTERS_END: u16 = 0x3FFF;

pub struct BUS {
    ram: RAM
}

impl BUS {
    pub fn new() -> Self {
        Self {
            ram: RAM::new()
        }
    }

    pub fn memory_read(&self, address: u16) -> u8 {
        match address {
            RAM ..= RAM_END => {
                let adjusted_address = address & 0b00000111_11111111;
                self.ram.read(adjusted_address)
            }

            PPU_REGISTERS ..= PPU_REGISTERS_END => {
                let _adjusted_address = address & 0b00100000_00000111;
                todo!("PPU is not supported yet")
            }
            _ => {
                println!("Ignoring memory access at {}", address);
                0
            }
        }
    }

    pub fn memory_write(&mut self, address: u16, data: u8) {
        match address {
            RAM ..= RAM_END => {
                let adjusted_address = address & 0b11111111111;
                self.ram.write(adjusted_address, data);
            }
            PPU_REGISTERS ..= PPU_REGISTERS_END => {
                let _adjusted_address = address & 0b00100000_00000111;
                todo!("PPU is not supported yet");
            }
            _ => {
                println!("Ignoring memory write-access at {}", address);
            }
        }
    }

    pub fn memory_read_u16(&self, address: u16) -> u16 {
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
}
