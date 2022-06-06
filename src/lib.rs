const LDA: u8 = 0xA9;
const BRK: u8 = 0x00;
const TAX: u8 = 0xAA;
const INX: u8 = 0xE8;
struct CPU {
    register_a: u8,
    register_x: u8,
    status: u8,
    pc: u16,
}

impl CPU {
    fn new() -> Self {
        CPU {
            register_a: 0,
            register_x: 0,
            status: 0,
            pc: 0,
        }
    }

    fn interpret(&mut self, program: Vec<u8>) {
        self.pc = 0;

        loop {
            let opcodes = program[self.pc as usize];
            self.pc += 1;

            match opcodes {
                LDA => {
                    let flag = program[self.pc as usize];
                    self.pc += 1;

                    self.lda(flag);
                }

                BRK => return,

                TAX => self.tax(),

                INX => self.inx(),

                _ => todo!(),
            }
        }
    }

    fn update_zero_and_negative_flags(&mut self, register: u8) {
        if register == 0 {
            self.status = self.status | 0x02;
        } else {
            self.status = self.status & 0xfd;
        }

        if register & 0x80 != 0 {
            self.status = self.status | 0x80;
        } else {
            self.status = self.status & 0x7f;
        }
    }

    fn lda(&mut self, flag: u8) {
        self.register_a = flag;

        self.update_zero_and_negative_flags(self.register_a);
    }

    fn tax(&mut self) {
        self.register_x = self.register_a;

        self.update_zero_and_negative_flags(self.register_x);
    }

    fn inx(&mut self) {
        self.register_x = self.register_x.wrapping_add(1);

        self.update_zero_and_negative_flags(self.register_x);
    }
}

pub fn run() {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    //Test LDA with non-zero flag
    fn test_lda_immidiate_load_data() {
        let mut cpu = CPU::new();
        let program = vec![LDA, 0xc0, BRK];

        cpu.interpret(program);
        assert_eq!(cpu.register_a, 0xc0);
        assert!(cpu.status & 0x02 == 0x00);
        assert!(cpu.status & 0x80 == 0x80);
    }

    #[test]
    //Test LDA with zero flag
    fn test_lda_zero_flag() {
        let mut cpu = CPU::new();
        let program = vec![LDA, BRK, BRK];

        cpu.interpret(program);
        assert!(cpu.status & 0x02 == 0x02)
    }

    #[test]
    //Tests if register_a can be moved to register_x using TAX
    fn test_tax_move_a_to_x() {
        let mut cpu = CPU::new();
        let program = vec![TAX, BRK];

        cpu.register_a = 10;
        cpu.interpret(program);
        assert_eq!(cpu.register_x, 10);
    }

    //Tests the LDA, TAX, INX and BRK opcodes
   #[test]
   fn test_5_ops_working_together() {
       let mut cpu = CPU::new();
       cpu.interpret(vec![LDA, 0xc0, TAX, INX, BRK]);

       assert_eq!(cpu.register_x, 0xc1)
   }

   //Tests if INX cannot be overflowed
    #[test]
    fn test_inx_overflow() {
        let mut cpu = CPU::new();
        cpu.register_x = 0xFF;
        cpu.interpret(vec![INX, INX, BRK]);

        assert_eq!(cpu.register_x, 1)
    }
}
