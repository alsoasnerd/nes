use crate::cpu::CPU;
use crate::opcodes;
use std::collections::HashMap;

pub struct Assembler {
    opcodes: HashMap<u8, &'static opcodes::OpCode>,
}

impl Assembler {
    pub fn new() -> Self {
        Self {
            opcodes: opcodes::OPCODES_MAP.clone(),
        }
    }

    pub fn interpret(&self, cpu: &mut CPU, code: u8) {
        let pc_state = cpu.register_pc;
        let opcode = self.opcodes.get(&code).unwrap();

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

            /* BRK */ 0x00 => return,

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

            _ => todo!(),
        }

        cpu.update_pc(opcode, pc_state);
    }
}
