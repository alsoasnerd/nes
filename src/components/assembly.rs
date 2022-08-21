use super::cpu::{ AddressingMode, CPU };
use std::collections::HashMap;

pub struct OpCode {
    pub code: u8,
    pub mnemonic: &'static str,
    pub len: u8,
    pub cycles: u8,
    pub mode: AddressingMode,
}

impl OpCode {
    fn new(code: u8, mnemonic: &'static str, len: u8, cycles: u8, mode: AddressingMode) -> Self {
        OpCode {
            code,
            mnemonic,
            len,
            cycles,
            mode,
        }
    }
}

lazy_static! {
    pub static ref CPUOPSCODES: Vec<OpCode> = vec![
        OpCode::new(0x00, "BRK", 1, 7, AddressingMode::NoneAddressing),
        OpCode::new(0xea, "NOP", 1, 2, AddressingMode::NoneAddressing),

        /* Arithmetic */
        OpCode::new(0x69, "ADC", 2, 2, AddressingMode::Immediate),
        OpCode::new(0x65, "ADC", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0x75, "ADC", 2, 4, AddressingMode::ZeroPageX),
        OpCode::new(0x6d, "ADC", 3, 4, AddressingMode::Absolute),
        OpCode::new(0x7d, "ADC", 3, 4/*+1 if page crossed*/, AddressingMode::AbsoluteX),
        OpCode::new(0x79, "ADC", 3, 4/*+1 if page crossed*/, AddressingMode::AbsoluteY),
        OpCode::new(0x61, "ADC", 2, 6, AddressingMode::IndirectX),
        OpCode::new(0x71, "ADC", 2, 5/*+1 if page crossed*/, AddressingMode::IndirectY),

        OpCode::new(0xe9, "SBC", 2, 2, AddressingMode::Immediate),
        OpCode::new(0xe5, "SBC", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0xf5, "SBC", 2, 4, AddressingMode::ZeroPageX),
        OpCode::new(0xed, "SBC", 3, 4, AddressingMode::Absolute),
        OpCode::new(0xfd, "SBC", 3, 4/*+1 if page crossed*/, AddressingMode::AbsoluteX),
        OpCode::new(0xf9, "SBC", 3, 4/*+1 if page crossed*/, AddressingMode::AbsoluteY),
        OpCode::new(0xe1, "SBC", 2, 6, AddressingMode::IndirectX),
        OpCode::new(0xf1, "SBC", 2, 5/*+1 if page crossed*/, AddressingMode::IndirectY),

        OpCode::new(0x29, "AND", 2, 2, AddressingMode::Immediate),
        OpCode::new(0x25, "AND", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0x35, "AND", 2, 4, AddressingMode::ZeroPageX),
        OpCode::new(0x2d, "AND", 3, 4, AddressingMode::Absolute),
        OpCode::new(0x3d, "AND", 3, 4/*+1 if page crossed*/, AddressingMode::AbsoluteX),
        OpCode::new(0x39, "AND", 3, 4/*+1 if page crossed*/, AddressingMode::AbsoluteY),
        OpCode::new(0x21, "AND", 2, 6, AddressingMode::IndirectX),
        OpCode::new(0x31, "AND", 2, 5/*+1 if page crossed*/, AddressingMode::IndirectY),

        OpCode::new(0x49, "EOR", 2, 2, AddressingMode::Immediate),
        OpCode::new(0x45, "EOR", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0x55, "EOR", 2, 4, AddressingMode::ZeroPageX),
        OpCode::new(0x4d, "EOR", 3, 4, AddressingMode::Absolute),
        OpCode::new(0x5d, "EOR", 3, 4/*+1 if page crossed*/, AddressingMode::AbsoluteX),
        OpCode::new(0x59, "EOR", 3, 4/*+1 if page crossed*/, AddressingMode::AbsoluteY),
        OpCode::new(0x41, "EOR", 2, 6, AddressingMode::IndirectX),
        OpCode::new(0x51, "EOR", 2, 5/*+1 if page crossed*/, AddressingMode::IndirectY),

        OpCode::new(0x09, "ORA", 2, 2, AddressingMode::Immediate),
        OpCode::new(0x05, "ORA", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0x15, "ORA", 2, 4, AddressingMode::ZeroPageX),
        OpCode::new(0x0d, "ORA", 3, 4, AddressingMode::Absolute),
        OpCode::new(0x1d, "ORA", 3, 4/*+1 if page crossed*/, AddressingMode::AbsoluteX),
        OpCode::new(0x19, "ORA", 3, 4/*+1 if page crossed*/, AddressingMode::AbsoluteY),
        OpCode::new(0x01, "ORA", 2, 6, AddressingMode::IndirectX),
        OpCode::new(0x11, "ORA", 2, 5/*+1 if page crossed*/, AddressingMode::IndirectY),

        /* Shifts */
        OpCode::new(0x0a, "ASL", 1, 2, AddressingMode::NoneAddressing),
        OpCode::new(0x06, "ASL", 2, 5, AddressingMode::ZeroPage),
        OpCode::new(0x16, "ASL", 2, 6, AddressingMode::ZeroPageX),
        OpCode::new(0x0e, "ASL", 3, 6, AddressingMode::Absolute),
        OpCode::new(0x1e, "ASL", 3, 7, AddressingMode::AbsoluteX),

        OpCode::new(0x4a, "LSR", 1, 2, AddressingMode::NoneAddressing),
        OpCode::new(0x46, "LSR", 2, 5, AddressingMode::ZeroPage),
        OpCode::new(0x56, "LSR", 2, 6, AddressingMode::ZeroPageX),
        OpCode::new(0x4e, "LSR", 3, 6, AddressingMode::Absolute),
        OpCode::new(0x5e, "LSR", 3, 7, AddressingMode::AbsoluteX),

        OpCode::new(0x2a, "ROL", 1, 2, AddressingMode::NoneAddressing),
        OpCode::new(0x26, "ROL", 2, 5, AddressingMode::ZeroPage),
        OpCode::new(0x36, "ROL", 2, 6, AddressingMode::ZeroPageX),
        OpCode::new(0x2e, "ROL", 3, 6, AddressingMode::Absolute),
        OpCode::new(0x3e, "ROL", 3, 7, AddressingMode::AbsoluteX),

        OpCode::new(0x6a, "ROR", 1, 2, AddressingMode::NoneAddressing),
        OpCode::new(0x66, "ROR", 2, 5, AddressingMode::ZeroPage),
        OpCode::new(0x76, "ROR", 2, 6, AddressingMode::ZeroPageX),
        OpCode::new(0x6e, "ROR", 3, 6, AddressingMode::Absolute),
        OpCode::new(0x7e, "ROR", 3, 7, AddressingMode::AbsoluteX),

        OpCode::new(0xe6, "INC", 2, 5, AddressingMode::ZeroPage),
        OpCode::new(0xf6, "INC", 2, 6, AddressingMode::ZeroPageX),
        OpCode::new(0xee, "INC", 3, 6, AddressingMode::Absolute),
        OpCode::new(0xfe, "INC", 3, 7, AddressingMode::AbsoluteX),

        OpCode::new(0xe8, "INX", 1, 2, AddressingMode::NoneAddressing),
        OpCode::new(0xc8, "INY", 1, 2, AddressingMode::NoneAddressing),

        OpCode::new(0xc6, "DEC", 2, 5, AddressingMode::ZeroPage),
        OpCode::new(0xd6, "DEC", 2, 6, AddressingMode::ZeroPageX),
        OpCode::new(0xce, "DEC", 3, 6, AddressingMode::Absolute),
        OpCode::new(0xde, "DEC", 3, 7, AddressingMode::AbsoluteX),

        OpCode::new(0xca, "DEX", 1, 2, AddressingMode::NoneAddressing),
        OpCode::new(0x88, "DEY", 1, 2, AddressingMode::NoneAddressing),

        OpCode::new(0xc9, "CMP", 2, 2, AddressingMode::Immediate),
        OpCode::new(0xc5, "CMP", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0xd5, "CMP", 2, 4, AddressingMode::ZeroPageX),
        OpCode::new(0xcd, "CMP", 3, 4, AddressingMode::Absolute),
        OpCode::new(0xdd, "CMP", 3, 4/*+1 if page crossed*/, AddressingMode::AbsoluteX),
        OpCode::new(0xd9, "CMP", 3, 4/*+1 if page crossed*/, AddressingMode::AbsoluteY),
        OpCode::new(0xc1, "CMP", 2, 6, AddressingMode::IndirectX),
        OpCode::new(0xd1, "CMP", 2, 5/*+1 if page crossed*/, AddressingMode::IndirectY),

        OpCode::new(0xc0, "CPY", 2, 2, AddressingMode::Immediate),
        OpCode::new(0xc4, "CPY", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0xcc, "CPY", 3, 4, AddressingMode::Absolute),

        OpCode::new(0xe0, "CPX", 2, 2, AddressingMode::Immediate),
        OpCode::new(0xe4, "CPX", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0xec, "CPX", 3, 4, AddressingMode::Absolute),


        /* Branching */

        OpCode::new(0x4c, "JMP", 3, 3, AddressingMode::NoneAddressing), //AddressingMode that acts as Immidiate
        OpCode::new(0x6c, "JMP", 3, 5, AddressingMode::NoneAddressing), //AddressingMode:Indirect with 6502 bug

        OpCode::new(0x20, "JSR", 3, 6, AddressingMode::NoneAddressing),
        OpCode::new(0x60, "RTS", 1, 6, AddressingMode::NoneAddressing),

        OpCode::new(0x40, "RTI", 1, 6, AddressingMode::NoneAddressing),

        OpCode::new(0xd0, "BNE", 2, 2 /*(+1 if branch succeeds +2 if to a new page)*/, AddressingMode::NoneAddressing),
        OpCode::new(0x70, "BVS", 2, 2 /*(+1 if branch succeeds +2 if to a new page)*/, AddressingMode::NoneAddressing),
        OpCode::new(0x50, "BVC", 2, 2 /*(+1 if branch succeeds +2 if to a new page)*/, AddressingMode::NoneAddressing),
        OpCode::new(0x30, "BMI", 2, 2 /*(+1 if branch succeeds +2 if to a new page)*/, AddressingMode::NoneAddressing),
        OpCode::new(0xf0, "BEQ", 2, 2 /*(+1 if branch succeeds +2 if to a new page)*/, AddressingMode::NoneAddressing),
        OpCode::new(0xb0, "BCS", 2, 2 /*(+1 if branch succeeds +2 if to a new page)*/, AddressingMode::NoneAddressing),
        OpCode::new(0x90, "BCC", 2, 2 /*(+1 if branch succeeds +2 if to a new page)*/, AddressingMode::NoneAddressing),
        OpCode::new(0x10, "BPL", 2, 2 /*(+1 if branch succeeds +2 if to a new page)*/, AddressingMode::NoneAddressing),

        OpCode::new(0x24, "BIT", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0x2c, "BIT", 3, 4, AddressingMode::Absolute),


        /* Stores, Loads */
        OpCode::new(0xa9, "LDA", 2, 2, AddressingMode::Immediate),
        OpCode::new(0xa5, "LDA", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0xb5, "LDA", 2, 4, AddressingMode::ZeroPageX),
        OpCode::new(0xad, "LDA", 3, 4, AddressingMode::Absolute),
        OpCode::new(0xbd, "LDA", 3, 4/*+1 if page crossed*/, AddressingMode::AbsoluteX),
        OpCode::new(0xb9, "LDA", 3, 4/*+1 if page crossed*/, AddressingMode::AbsoluteY),
        OpCode::new(0xa1, "LDA", 2, 6, AddressingMode::IndirectX),
        OpCode::new(0xb1, "LDA", 2, 5/*+1 if page crossed*/, AddressingMode::IndirectY),

        OpCode::new(0xa2, "LDX", 2, 2, AddressingMode::Immediate),
        OpCode::new(0xa6, "LDX", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0xb6, "LDX", 2, 4, AddressingMode::ZeroPageY),
        OpCode::new(0xae, "LDX", 3, 4, AddressingMode::Absolute),
        OpCode::new(0xbe, "LDX", 3, 4/*+1 if page crossed*/, AddressingMode::AbsoluteY),

        OpCode::new(0xa0, "LDY", 2, 2, AddressingMode::Immediate),
        OpCode::new(0xa4, "LDY", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0xb4, "LDY", 2, 4, AddressingMode::ZeroPageX),
        OpCode::new(0xac, "LDY", 3, 4, AddressingMode::Absolute),
        OpCode::new(0xbc, "LDY", 3, 4/*+1 if page crossed*/, AddressingMode::AbsoluteX),


        OpCode::new(0x85, "STA", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0x95, "STA", 2, 4, AddressingMode::ZeroPageX),
        OpCode::new(0x8d, "STA", 3, 4, AddressingMode::Absolute),
        OpCode::new(0x9d, "STA", 3, 5, AddressingMode::AbsoluteX),
        OpCode::new(0x99, "STA", 3, 5, AddressingMode::AbsoluteY),
        OpCode::new(0x81, "STA", 2, 6, AddressingMode::IndirectX),
        OpCode::new(0x91, "STA", 2, 6, AddressingMode::IndirectY),

        OpCode::new(0x86, "STX", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0x96, "STX", 2, 4, AddressingMode::ZeroPageY),
        OpCode::new(0x8e, "STX", 3, 4, AddressingMode::Absolute),

        OpCode::new(0x84, "STY", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0x94, "STY", 2, 4, AddressingMode::ZeroPageX),
        OpCode::new(0x8c, "STY", 3, 4, AddressingMode::Absolute),


        /* Flags clear */

        OpCode::new(0xD8, "CLD", 1, 2, AddressingMode::NoneAddressing),
        OpCode::new(0x58, "CLI", 1, 2, AddressingMode::NoneAddressing),
        OpCode::new(0xb8, "CLV", 1, 2, AddressingMode::NoneAddressing),
        OpCode::new(0x18, "CLC", 1, 2, AddressingMode::NoneAddressing),
        OpCode::new(0x38, "SEC", 1, 2, AddressingMode::NoneAddressing),
        OpCode::new(0x78, "SEI", 1, 2, AddressingMode::NoneAddressing),
        OpCode::new(0xf8, "SED", 1, 2, AddressingMode::NoneAddressing),

        OpCode::new(0xaa, "TAX", 1, 2, AddressingMode::NoneAddressing),
        OpCode::new(0xa8, "TAY", 1, 2, AddressingMode::NoneAddressing),
        OpCode::new(0xba, "TSX", 1, 2, AddressingMode::NoneAddressing),
        OpCode::new(0x8a, "TXA", 1, 2, AddressingMode::NoneAddressing),
        OpCode::new(0x9a, "TXS", 1, 2, AddressingMode::NoneAddressing),
        OpCode::new(0x98, "TYA", 1, 2, AddressingMode::NoneAddressing),

        /* Stack */
        OpCode::new(0x48, "PHA", 1, 3, AddressingMode::NoneAddressing),
        OpCode::new(0x68, "PLA", 1, 4, AddressingMode::NoneAddressing),
        OpCode::new(0x08, "PHP", 1, 3, AddressingMode::NoneAddressing),
        OpCode::new(0x28, "PLP", 1, 4, AddressingMode::NoneAddressing),

        /* unofficial */

        OpCode::new(0xc7, "*DCP", 2, 5, AddressingMode::ZeroPage),
        OpCode::new(0xd7, "*DCP", 2, 6, AddressingMode::ZeroPageX),
        OpCode::new(0xCF, "*DCP", 3, 6, AddressingMode::Absolute),
        OpCode::new(0xdF, "*DCP", 3, 7, AddressingMode::AbsoluteX),
        OpCode::new(0xdb, "*DCP", 3, 7, AddressingMode::AbsoluteY),
        OpCode::new(0xd3, "*DCP", 2, 8, AddressingMode::IndirectY),
        OpCode::new(0xc3, "*DCP", 2, 8, AddressingMode::IndirectX),


        OpCode::new(0x27, "*RLA", 2, 5, AddressingMode::ZeroPage),
        OpCode::new(0x37, "*RLA", 2, 6, AddressingMode::ZeroPageX),
        OpCode::new(0x2F, "*RLA", 3, 6, AddressingMode::Absolute),
        OpCode::new(0x3F, "*RLA", 3, 7, AddressingMode::AbsoluteX),
        OpCode::new(0x3b, "*RLA", 3, 7, AddressingMode::AbsoluteY),
        OpCode::new(0x33, "*RLA", 2, 8, AddressingMode::IndirectY),
        OpCode::new(0x23, "*RLA", 2, 8, AddressingMode::IndirectX),

        OpCode::new(0x07, "*SLO", 2, 5, AddressingMode::ZeroPage),
        OpCode::new(0x17, "*SLO", 2, 6, AddressingMode::ZeroPageX),
        OpCode::new(0x0F, "*SLO", 3, 6, AddressingMode::Absolute),
        OpCode::new(0x1f, "*SLO", 3, 7, AddressingMode::AbsoluteX),
        OpCode::new(0x1b, "*SLO", 3, 7, AddressingMode::AbsoluteY),
        OpCode::new(0x03, "*SLO", 2, 8, AddressingMode::IndirectX),
        OpCode::new(0x13, "*SLO", 2, 8, AddressingMode::IndirectY),

        OpCode::new(0x47, "*SRE", 2, 5, AddressingMode::ZeroPage),
        OpCode::new(0x57, "*SRE", 2, 6, AddressingMode::ZeroPageX),
        OpCode::new(0x4F, "*SRE", 3, 6, AddressingMode::Absolute),
        OpCode::new(0x5f, "*SRE", 3, 7, AddressingMode::AbsoluteX),
        OpCode::new(0x5b, "*SRE", 3, 7, AddressingMode::AbsoluteY),
        OpCode::new(0x43, "*SRE", 2, 8, AddressingMode::IndirectX),
        OpCode::new(0x53, "*SRE", 2, 8, AddressingMode::IndirectY),


        OpCode::new(0x80, "*NOP", 2,2, AddressingMode::Immediate),
        OpCode::new(0x82, "*NOP", 2,2, AddressingMode::Immediate),
        OpCode::new(0x89, "*NOP", 2,2, AddressingMode::Immediate),
        OpCode::new(0xc2, "*NOP", 2,2, AddressingMode::Immediate),
        OpCode::new(0xe2, "*NOP", 2,2, AddressingMode::Immediate),


        OpCode::new(0xCB, "*AXS", 2,2, AddressingMode::Immediate),

        OpCode::new(0x6B, "*ARR", 2,2, AddressingMode::Immediate),

        OpCode::new(0xeb, "*SBC", 2,2, AddressingMode::Immediate),

        OpCode::new(0x0b, "*ANC", 2,2, AddressingMode::Immediate),
        OpCode::new(0x2b, "*ANC", 2,2, AddressingMode::Immediate),

        OpCode::new(0x4b, "*ALR", 2,2, AddressingMode::Immediate),
        // OpCode::new(0xCB, "IGN", 3,4 /* or 5*/, AddressingMode::AbsoluteX),

        OpCode::new(0x04, "*NOP", 2,3, AddressingMode::ZeroPage),
        OpCode::new(0x44, "*NOP", 2,3, AddressingMode::ZeroPage),
        OpCode::new(0x64, "*NOP", 2,3, AddressingMode::ZeroPage),
        OpCode::new(0x14, "*NOP", 2, 4, AddressingMode::ZeroPageX),
        OpCode::new(0x34, "*NOP", 2, 4, AddressingMode::ZeroPageX),
        OpCode::new(0x54, "*NOP", 2, 4, AddressingMode::ZeroPageX),
        OpCode::new(0x74, "*NOP", 2, 4, AddressingMode::ZeroPageX),
        OpCode::new(0xd4, "*NOP", 2, 4, AddressingMode::ZeroPageX),
        OpCode::new(0xf4, "*NOP", 2, 4, AddressingMode::ZeroPageX),
        OpCode::new(0x0c, "*NOP", 3, 4, AddressingMode::Absolute),
        OpCode::new(0x1c, "*NOP", 3, 4 /*or 5*/, AddressingMode::AbsoluteX),
        OpCode::new(0x3c, "*NOP", 3, 4 /*or 5*/, AddressingMode::AbsoluteX),
        OpCode::new(0x5c, "*NOP", 3, 4 /*or 5*/, AddressingMode::AbsoluteX),
        OpCode::new(0x7c, "*NOP", 3, 4 /*or 5*/, AddressingMode::AbsoluteX),
        OpCode::new(0xdc, "*NOP", 3, 4 /* or 5*/, AddressingMode::AbsoluteX),
        OpCode::new(0xfc, "*NOP", 3, 4 /* or 5*/, AddressingMode::AbsoluteX),

        OpCode::new(0x67, "*RRA", 2, 5, AddressingMode::ZeroPage),
        OpCode::new(0x77, "*RRA", 2, 6, AddressingMode::ZeroPageX),
        OpCode::new(0x6f, "*RRA", 3, 6, AddressingMode::Absolute),
        OpCode::new(0x7f, "*RRA", 3, 7, AddressingMode::AbsoluteX),
        OpCode::new(0x7b, "*RRA", 3, 7, AddressingMode::AbsoluteY),
        OpCode::new(0x63, "*RRA", 2, 8, AddressingMode::IndirectX),
        OpCode::new(0x73, "*RRA", 2, 8, AddressingMode::IndirectY),


        OpCode::new(0xe7, "*ISB", 2,5, AddressingMode::ZeroPage),
        OpCode::new(0xf7, "*ISB", 2,6, AddressingMode::ZeroPageX),
        OpCode::new(0xef, "*ISB", 3,6, AddressingMode::Absolute),
        OpCode::new(0xff, "*ISB", 3,7, AddressingMode::AbsoluteX),
        OpCode::new(0xfb, "*ISB", 3,7, AddressingMode::AbsoluteY),
        OpCode::new(0xe3, "*ISB", 2,8, AddressingMode::IndirectX),
        OpCode::new(0xf3, "*ISB", 2,8, AddressingMode::IndirectY),

        OpCode::new(0x02, "*NOP", 1,2, AddressingMode::NoneAddressing),
        OpCode::new(0x12, "*NOP", 1,2, AddressingMode::NoneAddressing),
        OpCode::new(0x22, "*NOP", 1,2, AddressingMode::NoneAddressing),
        OpCode::new(0x32, "*NOP", 1,2, AddressingMode::NoneAddressing),
        OpCode::new(0x42, "*NOP", 1,2, AddressingMode::NoneAddressing),
        OpCode::new(0x52, "*NOP", 1,2, AddressingMode::NoneAddressing),
        OpCode::new(0x62, "*NOP", 1,2, AddressingMode::NoneAddressing),
        OpCode::new(0x72, "*NOP", 1,2, AddressingMode::NoneAddressing),
        OpCode::new(0x92, "*NOP", 1,2, AddressingMode::NoneAddressing),
        OpCode::new(0xb2, "*NOP", 1,2, AddressingMode::NoneAddressing),
        OpCode::new(0xd2, "*NOP", 1,2, AddressingMode::NoneAddressing),
        OpCode::new(0xf2, "*NOP", 1,2, AddressingMode::NoneAddressing),

        OpCode::new(0x1a, "*NOP", 1,2, AddressingMode::NoneAddressing),
        OpCode::new(0x3a, "*NOP", 1,2, AddressingMode::NoneAddressing),
        OpCode::new(0x5a, "*NOP", 1,2, AddressingMode::NoneAddressing),
        OpCode::new(0x7a, "*NOP", 1,2, AddressingMode::NoneAddressing),
        OpCode::new(0xda, "*NOP", 1,2, AddressingMode::NoneAddressing),
        // OpCode::new(0xea, "NOP", 1,2, AddressingMode::NoneAddressing),
        OpCode::new(0xfa, "*NOP", 1,2, AddressingMode::NoneAddressing),

        OpCode::new(0xab, "*LXA", 2, 3, AddressingMode::Immediate), //todo: highly unstable and not used
        //http://visual6502.org/wiki/index.php?title=6502_Opcode_8B_%28XAA,_ANE%29
        OpCode::new(0x8b, "*XAA", 2, 3, AddressingMode::Immediate), //todo: highly unstable and not used
        OpCode::new(0xbb, "*LAS", 3, 2, AddressingMode::AbsoluteY), //todo: highly unstable and not used
        OpCode::new(0x9b, "*TAS", 3, 2, AddressingMode::AbsoluteY), //todo: highly unstable and not used
        OpCode::new(0x93, "*AHX", 2, /* guess */ 8, AddressingMode::IndirectY), //todo: highly unstable and not used
        OpCode::new(0x9f, "*AHX", 3, /* guess */ 4/* or 5*/, AddressingMode::AbsoluteY), //todo: highly unstable and not used
        OpCode::new(0x9e, "*SHX", 3, /* guess */ 4/* or 5*/, AddressingMode::AbsoluteY), //todo: highly unstable and not used
        OpCode::new(0x9c, "*SHY", 3, /* guess */ 4/* or 5*/, AddressingMode::AbsoluteX), //todo: highly unstable and not used

        OpCode::new(0xa7, "*LAX", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0xb7, "*LAX", 2, 4, AddressingMode::ZeroPageY),
        OpCode::new(0xaf, "*LAX", 3, 4, AddressingMode::Absolute),
        OpCode::new(0xbf, "*LAX", 3, 4, AddressingMode::AbsoluteY),
        OpCode::new(0xa3, "*LAX", 2, 6, AddressingMode::IndirectX),
        OpCode::new(0xb3, "*LAX", 2, 5, AddressingMode::IndirectY),

        OpCode::new(0x87, "*SAX", 2, 3, AddressingMode::ZeroPage),
        OpCode::new(0x97, "*SAX", 2, 4, AddressingMode::ZeroPageY),
        OpCode::new(0x8f, "*SAX", 3, 4, AddressingMode::Absolute),
        OpCode::new(0x83, "*SAX", 2, 6, AddressingMode::IndirectX),
    ];


    pub static ref OPCODES_MAP: HashMap<u8, &'static OpCode> = {
        let mut map = HashMap::new();
        for cpuop in &*CPUOPSCODES {
            map.insert(cpuop.code, cpuop);
        }
        map
    };
}

pub struct Assembler {
    opcodes: HashMap<u8, &'static OpCode>
}

impl Assembler {
    pub fn new() -> Self {
        Assembler {
            opcodes: OPCODES_MAP.clone()
        }
    }

    pub fn interpret(&self, cpu: &mut CPU, code: u8) -> bool {
        let pc_state = cpu.register_pc;
        let opcode = self
            .opcodes
            .get(&code)
            .expect(&format!("OpCode {:x} is not recognized", code));

        match code {
            /* ADC */
            0x69 | 0x65 | 0x75 | 0x6d | 0x7d | 0x79 | 0x61 | 0x71 => {
                cpu.adc(&opcode.mode);
            }

            /* AND */
            0x29 | 0x25 | 0x35 | 0x2d | 0x3d | 0x39 | 0x21 | 0x31 => {
                cpu.and(&opcode.mode);
            }

            /* ASL */ 0x0a => cpu.asl_accumulator(),

            /* ASL */
            0x06 | 0x16 | 0x0e | 0x1e => {
                cpu.asl(&opcode.mode);
            }

            /* BCC */ 0x90 => cpu.bcc(),

            /* BCS */ 0xb0 => cpu.bcs(),

            /* BEQ */ 0xf0 => cpu.beq(),

            /* BIT */
            0x24 | 0x2c => {
                cpu.bit(&opcode.mode);
            }

            /* BMI */ 0x30 => cpu.bmi(),

            /* BNE */ 0xd0 => cpu.bne(),

            /* BPL */ 0x10 => cpu.bpl(),

            /* BRK */ 0x00 => return true,

            /* BVC */ 0x50 => cpu.bvc(),

            /* BVS */ 0x70 => cpu.bvs(),

            /* CLC */ 0x18 => cpu.clc(),

            /* CLD */ 0xd8 => cpu.cld(),

            /* CLI */ 0x58 => cpu.cli(),

            /* CLV */ 0xb8 => cpu.clv(),

            /* CMP */
            0xc9 | 0xc5 | 0xd5 | 0xcd | 0xdd | 0xd9 | 0xc1 | 0xd1 => {
                cpu.cmp(&opcode.mode);
            }

            /* CPX */
            0xe0 | 0xe4 | 0xec => {
                cpu.cpx(&opcode.mode);
            }

            /* CPY */
            0xc0 | 0xc4 | 0xcc => {
                cpu.cpy(&opcode.mode);
            }

            /* DEC */
            0xc6 | 0xd6 | 0xce | 0xde => {
                cpu.dec(&opcode.mode);
            }

            /* DEX */ 0xca => cpu.dex(),

            /* DEY */ 0x88 => cpu.dey(),

            /* EOR */
            0x49 | 0x45 | 0x55 | 0x4d | 0x5d | 0x59 | 0x41 | 0x51 => {
                cpu.eor(&opcode.mode);
            }

            /* INC */
            0xe6 | 0xf6 | 0xee | 0xfe => {
                cpu.inc(&opcode.mode);
            }

            /* INX */ 0xe8 => cpu.inx(),

            /* INY */ 0xc8 => cpu.iny(),

            /* JMP Absolute */ 0x4c => cpu.jmp_absolute(),

            /* JMP Indirect */ 0x6c => cpu.jmp_indirect(),

            /* JSR */ 0x20 => cpu.jsr(),

            /* LDA */
            0xa9 | 0xa5 | 0xb5 | 0xad | 0xbd | 0xb9 | 0xa1 | 0xb1 => {
                cpu.lda(&opcode.mode);
            }

            /* LDX */
            0xa2 | 0xa6 | 0xb6 | 0xae | 0xbe => {
                cpu.ldx(&opcode.mode);
            }

            /* LDY */
            0xa0 | 0xa4 | 0xb4 | 0xac | 0xbc => {
                cpu.ldy(&opcode.mode);
            }

            /* LSR */ 0x4a => cpu.lsr_accumulator(),

            /* LSR */
            0x46 | 0x56 | 0x4e | 0x5e => {
                cpu.lsr(&opcode.mode);
            }

            /* NOP */ 0xea => {}

            /* ORA */
            0x09 | 0x05 | 0x15 | 0x0d | 0x1d | 0x19 | 0x01 | 0x11 => {
                cpu.ora(&opcode.mode);
            }

            /* PHA */ 0x48 => cpu.pha(),

            /* PHP */ 0x08 => cpu.php(),

            /* PLA */ 0x68 => cpu.pla(),

            /* PLP */ 0x28 => cpu.plp(),

            /* ROL */ 0x2a => cpu.rol_accumulator(),

            /* ROL */
            0x26 | 0x36 | 0x2e | 0x3e => {
                cpu.rol(&opcode.mode);
            }

            /* ROR */ 0x6a => cpu.ror_accumulator(),

            /* ROR */
            0x66 | 0x76 | 0x6e | 0x7e => {
                cpu.ror(&opcode.mode);
            }

            /* RTI */ 0x40 => cpu.rti(),

            /* RTS */ 0x60 => cpu.rts(),

            /* SBC */
            0xe9 | 0xe5 | 0xf5 | 0xed | 0xfd | 0xf9 | 0xe1 | 0xf1 => {
                cpu.sbc(&opcode.mode);
            }

            /* SEC */ 0x38 => cpu.sec(),

            /* SED */ 0xf8 => cpu.sed(),

            /* SEI */ 0x78 => cpu.sei(),

            /* STA */
            0x85 | 0x95 | 0x8d | 0x9d | 0x99 | 0x81 | 0x91 => {
                cpu.sta(&opcode.mode);
            }

            /* STX */
            0x86 | 0x96 | 0x8e => {
                cpu.stx(&opcode.mode);
            }

            /* STY */
            0x84 | 0x94 | 0x8c => {
                cpu.sty(&opcode.mode);
            }

            /* TAX */ 0xAA => cpu.tax(),

            /* TAY */ 0xa8 => cpu.tay(),

            /* TSX */ 0xba => cpu.tsx(),

            /* TXA */ 0x8a => cpu.txa(),

            /* TXS */ 0x9a => cpu.txs(),

            /* TYA */ 0x98 => cpu.tya(),

            /* unofficial */

            /* DCP */
            0xc7 | 0xd7 | 0xCF | 0xdF | 0xdb | 0xd3 | 0xc3 => {
                cpu.dcp(&opcode.mode);
            }

            /* RLA */
            0x27 | 0x37 | 0x2F | 0x3F | 0x3b | 0x33 | 0x23 => {
                cpu.rla(&opcode.mode);
            }

            /* SLO */
            0x07 | 0x17 | 0x0F | 0x1f | 0x1b | 0x03 | 0x13 => {
                cpu.slo(&opcode.mode);
            }

            /* SRE */
            0x47 | 0x57 | 0x4F | 0x5f | 0x5b | 0x43 | 0x53 => {
                cpu.sre(&opcode.mode);
            }

            /* SKB */
            0x80 | 0x82 | 0x89 | 0xc2 | 0xe2 => {
                // do nothing
            }

            /* AXS */
            0xCB => cpu.axs(&opcode.mode),

            /* ARR */
            0x6B => cpu.arr(&opcode.mode),

            /* unofficial SBC */
            0xeb => cpu.unofficial_sbc(&opcode.mode),

            /* ANC */
            0x0b | 0x2b => {
                cpu.anc(&opcode.mode);
            }

            /* ALR */
            0x4b => cpu.alr(&opcode.mode),

            /* NOP read */
            0x04 | 0x44 | 0x64 | 0x14 | 0x34 | 0x54 | 0x74 | 0xd4 | 0xf4 | 0x0c | 0x1c | 0x3c
            | 0x5c | 0x7c | 0xdc | 0xfc => {
                cpu.nop_read(&opcode.mode);
            }

            /* RRA */
            0x67 | 0x77 | 0x6f | 0x7f | 0x7b | 0x63 | 0x73 => {
                cpu.rra(&opcode.mode);
            }

            /* ISB */
            0xe7 | 0xf7 | 0xef | 0xff | 0xfb | 0xe3 | 0xf3 => {
                cpu.isb(&opcode.mode);
            }

            /* NOPs */
            0x02 | 0x12 | 0x22 | 0x32 | 0x42 | 0x52 | 0x62 | 0x72 | 0x92 | 0xb2 | 0xd2 | 0xf2
            | 0x1a | 0x3a | 0x5a | 0x7a | 0xda | 0xfa => {}

            /* LAX */
            0xa7 | 0xb7 | 0xaf | 0xbf | 0xa3 | 0xb3 => {
                cpu.lax(&opcode.mode);
            }

            /* SAX */
            0x87 | 0x97 | 0x8f | 0x83 => {
                cpu.sax(&opcode.mode);
            }

            /* LXA */
            0xab => cpu.lxa(&opcode.mode),

            /* XAA */
            0x8b => cpu.xaa(&opcode.mode),

            /* LAS */
            0xbb => cpu.las(&opcode.mode),

            /* TAS */
            0x9b => cpu.tas(),

            /* AXA Indirect Y */
            0x93 => cpu.axa_indirect(),

            /* AXA Absolute Y*/
            0x9f => cpu.axa_absolute(),

            /* SXA */
            0x9e => cpu.sxa(),

            /* SYA */
            0x9c => cpu.sya(),
        }

        cpu.update_pc(&opcode, pc_state);
        false
    }
}