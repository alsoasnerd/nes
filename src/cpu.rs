use std::collections::HashMap;

use crate::opcodes;

pub const ROM_FIRST_BYTE: u16 = 0x8000;
pub const ROM_FIRST_ADDRESS: u16 = 0xFFFC;

#[derive(Debug)]
pub enum AddressingMode {
    Accumulator,
    Immediate,
    ZeroPage,
    ZeroPageX,
    ZeroPageY,
    Relative,
    Absolute,
    AbsoluteX,
    AbsoluteY,
    Indirect,
    IndirectX,
    IndirectY,
    NoneAddressing,
}

#[derive(Debug)]
struct Memmory {
    pub array: [u8; 0xFFFF]
}

impl Memmory {
    fn new() -> Self {
        Memmory {
            array: [0; 0xFFFF]
        }
    }
    fn read(&self, address: u16) -> u8 {
        self.array[address as usize]
    }

    fn write(&mut self, address: u16, data: u8) {
        self.array[address as usize] = data;
    }

    fn read_u16(&self, address: u16) -> u16 {
        let low = self.read(address);
        let high = self.read(address + 1);

        u16::from_le_bytes([high, low])
    }

    fn write_u16(&mut self, address: u16, data: u16) {
        let low = (data >> 8) as u8;
        let high = (data & 0xFF) as u8;

        self.write(address, low);
        self.write(address + 1, high)
    }
}

#[derive(Debug)]
pub struct CPU {
    pub register_a: u8,
    pub register_x: u8,
    pub register_y: u8,
    pub status: u8,
    pub pc: u16,
    memmory: Memmory,
    stack: Vec<u8>,
}

impl Default for CPU {
    fn default() -> Self {
        Self::new()
    }
}

impl CPU {
    pub fn new() -> Self {
        CPU {
            register_a: 0,
            register_x: 0,
            register_y: 0,
            status: 0,
            pc: 0,
            memmory: Memmory::new(),
            stack: Vec::new(),
        }
    }

    fn get_operand_address(&self, mode: &AddressingMode) -> u16 {
        match mode {
            AddressingMode::Accumulator => self.register_a as u16,
            AddressingMode::Immediate => self.pc, // Get the address into register, not the value.
            AddressingMode::ZeroPage => self.memmory.read(self.pc) as u16, // Get any value less then 256 bytes.
            AddressingMode::Absolute => self.memmory.read_u16(self.pc), // Loads any value.

            // Gets any value less then 256 bytes and add value of X register.
            AddressingMode::ZeroPageX => {
                let position = self.memmory.read(self.pc);
                position.wrapping_add(self.register_x) as u16
            },

            // Gets any value less then 256 bytes and add value of Y register.
            AddressingMode::ZeroPageY => {
                let position = self.memmory.read(self.pc);
                position.wrapping_add(self.register_y) as u16
            },

            // Gets any address and add in PC.
            AddressingMode::Relative => {
                let position = self.memmory.read(self.pc);
                self.pc.wrapping_add(position as u16)
            }

            // Gets any value and add value of X register.
            AddressingMode::AbsoluteX => {
                let base = self.memmory.read_u16(self.pc);
                base.wrapping_add(self.register_x as u16)
            },

            // Gets any value and add value of Y register.
            AddressingMode::AbsoluteY => {
                let base = self.memmory.read_u16(self.pc);
                base.wrapping_add(self.register_y as u16)
            },

            // Gets any value of any address.
            AddressingMode::Indirect => {
                let base = self.memmory.read_u16(self.pc);
                self.memmory.read_u16(base)
            }

            // Add value of X register in a zero page address, gets the value in this address, and ordenate him using Little Endian
            AddressingMode::IndirectX => {
                let base = self.memmory.read(self.pc);
                let pointer = base.wrapping_add(self.register_x);
                let low = self.memmory.read(pointer as u16);
                let high = self.memmory.read(pointer.wrapping_add(1) as u16);

                u16::from_le_bytes([high, low])
            },

            // Dereference an zero page address using Little Endian and add the Y register in result.
            AddressingMode::IndirectY => {
                let base = self.memmory.read(self.pc);
                let low = self.memmory.read(base as u16);
                let high = self.memmory.read(base.wrapping_add(1) as u16);
                let deref_base = u16::from_le_bytes([high, low]);

                deref_base.wrapping_add(self.register_y as u16)
            },

            AddressingMode::NoneAddressing => {
                panic!("mode {:?} is not supported", mode);
            }

        }
    }

    pub fn load_and_run(&mut self, program: Vec<u8>) {
        self.load(program);
        self.reset();
        self.run();
    }

    pub fn load(&mut self, program: Vec<u8>) {
        let program_space = ROM_FIRST_BYTE as usize ..(ROM_FIRST_BYTE as usize + program.len());

        self.memmory.array[program_space].copy_from_slice(&program[..]);
        self.memmory.write_u16(ROM_FIRST_ADDRESS, ROM_FIRST_BYTE);
    }

    pub fn run(&mut self) {
        let opcodes: &HashMap<u8, &'static opcodes::OpCode> = &(*opcodes::OPCODES_HASHMAP);
        loop {
            let code = self.memmory.read(self.pc);
            self.pc += 1;
            let pc_state = self.pc;

            let opcode = opcodes.get(&code).unwrap_or_else(|| {
                panic!("OpCode {:x} is not recognized", code);
            });

            match code {
                0x69 | 0x65 | 0x75 | 0x6d | 0x7d | 0x79 | 0x61 | 0x71 => {
                    self.adc(&opcode.mode);
                }

                0x29 | 0x25 | 0x35 | 0x2d | 0x3d | 0x39 | 0x21 | 0x31 => {
                    self.and(&opcode.mode);
                }

                0x0a | 0x1a => {
                    self.asl(&opcode.mode);
                }

                0xA9 | 0xA5 | 0xB5 | 0xAD | 0xBD | 0xB9 | 0xA1 | 0xB1 => {
                    self.lda(&opcode.mode);
                }

                0x85 | 0x95 | 0x8d | 0x9D | 0x99 | 0x81 | 0x91 => {
                    self.sta(&opcode.mode);
                }


                0xE9 | 0xE5 | 0xF5 | 0xED | 0xFD | 0xF9 | 0xE1 | 0xF1 => {
                    self.sbc(&opcode.mode);
                }

                0xAA => self.tax(),
                0xE8 => self.inx(),
                0x48 => self.pha(),
                0x08 => self.php(),
                0x68 => self.pla(),
                0x28 => self.plp(),
                0x00 => return,
                _ => todo!(),
            }

            if pc_state == self.pc {
                self.pc += (opcode.len - 1) as u16
            }
        }
    }

    fn adc(&mut self, mode: &AddressingMode) {
        let address = self.get_operand_address(mode);
        let value = self.memmory.read(address);

        let result = self.register_a.wrapping_add(value).wrapping_add(self.status & 0b1000_0000);

        self.register_a = result;
    }

    fn and(&mut self, mode: &AddressingMode) {
        let address = self.get_operand_address(mode);
        let value = self.memmory.read(address);

        self.register_a &= value;
    }

    fn asl(&mut self, mode: &AddressingMode) {
        let address = self.get_operand_address(mode);
        let value = self.memmory.read(address);

        let result = value.wrapping_add(value << 1);
        self.memmory.write(address, result);
    }

    fn lda(&mut self, mode: &AddressingMode) {
        let address = self.get_operand_address(mode);
        let value = self.memmory.read(address);

        self.register_a = value;
        self.update_zero_and_negative_flags(self.register_a);
    }

    fn sta(&mut self, mode: &AddressingMode){
        let address = self.get_operand_address(mode);
        self.memmory.write(address, self.register_a);
    }

    fn tax(&mut self) {
        self.register_x = self.register_a;

        self.update_zero_and_negative_flags(self.register_x);
    }

    fn inx(&mut self) {
        self.register_x = self.register_x.wrapping_add(1);

        self.update_zero_and_negative_flags(self.register_x);
    }


    fn sbc(&mut self, mode: &AddressingMode) {
        let address = self.get_operand_address(mode);
        let value = self.memmory.read(address);

        let result = self.register_a.wrapping_sub(value).wrapping_sub(self.status & 0b1000_0000);

        self.register_a = result;
    }

    fn pha(&mut self) {
        self.stack.push(self.register_a);
    }

    fn php(&mut self) {
        self.stack.push(self.status);
    }

    fn pla(&mut self) {
        let value = self.stack.pop().unwrap();
        self.register_a = value;

        self.update_zero_and_negative_flags(self.register_a);
    }

    fn plp(&mut self) {
        let value = self.stack.pop().unwrap();
        self.status = value;
    }

    fn update_zero_and_negative_flags(&mut self, result: u8) {
        if result == 0 {
            self.status |= 0b0000_0010;
        } else {
            self.status &= 0b1111_1101;
        }

        if result & 0b1000_0000 != 0 {
            self.status |= 0b1000_0000;
        } else {
            self.status &= 0b0111_1111;
        }
    }

    pub fn reset(&mut self) {
        self.register_a = 0;
        self.register_x = 0;
        self.register_y = 0;
        self.status = 0;

        self.pc = self.memmory.read_u16(ROM_FIRST_ADDRESS);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_0xa9_lda_immidiate_load_data() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x05, 0x00]);
        assert_eq!(cpu.register_a, 5);
        assert!(cpu.status & 0b0000_0010 == 0);
        assert!(cpu.status & 0b1000_0000 == 0);
    }

    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x00, 0x00]);
        assert!(cpu.status & 0b0000_0010 == 0b10);
    }

    #[test]
    fn test_0xaa_tax_move_a_to_x() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x0A,0xaa, 0x00]);

        assert_eq!(cpu.register_x, 10)
    }

    #[test]
    fn test_5_ops_working_together() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0xc0, 0xaa, 0xe8, 0x00]);

        assert_eq!(cpu.register_x, 0xc1)
    }

    #[test]
    fn test_inx_overflow() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0xff, 0xaa, 0xe8, 0xe8, 0x00]);

        assert_eq!(cpu.register_x, 1)
    }

    #[test]
    fn test_lda_from_memory() {
        let mut cpu = CPU::new();
        cpu.memmory.write(0x10, 0x55);

        cpu.load_and_run(vec![0xa5, 0x10, 0x00]);

        assert_eq!(cpu.register_a, 0x55);
    }

    #[test]
    fn test_sta_0x85_and_lda() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa5, 0xc0, 0x85, 0x00]);

        assert_eq!(cpu.memmory.array[0x8001], 0xc0);
    }

    #[test]
    fn test_adc_without_carry() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x01, 0x69, 0x02]);

        assert_eq!(cpu.register_a, 0x03);
    }

    #[test]
    fn test_adc_with_carry() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x50, 0x69, 0x90]);

        assert_eq!(cpu.register_a, 0xe0);
    }

    #[test]
    fn test_sbc_without_carry() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x01, 0xE9, 0x02]);

        assert_eq!(cpu.register_a, 0xFF);
    }

    #[test]
    fn test_sbc_with_carry() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x50, 0xE9, 0xf0]);

        assert_eq!(cpu.register_a, 0x60);
    }

    #[test]
    fn test_pha_plp_php_pla() {
        let mut cpu = CPU::new();

        cpu.load_and_run(vec![0xa9, 0x10, 0x48, 0x28, 0xa9, 0x05, 0x08, 0x68]);
        assert_eq!(cpu.status, 0x10);
        assert_eq!(cpu.register_a, 0x10);
    }
}