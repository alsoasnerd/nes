pub mod cpu;
pub mod opcodes;

#[cfg(test)]
mod test {
    use crate::cpu::CPU;

    #[test]
    fn test_0xa9_lda_immidiate_load_data() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x05, 0x00]);
        assert_eq!(cpu.register_a, 5);
        assert!(cpu.register_sr & 0b0000_0010 == 0);
        assert!(cpu.register_sr & 0b1000_0000 == 0);
    }

    #[test]
    fn test_0xa9_lda_zero_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x00, 0x00]);
        assert!(cpu.register_sr & 0b0000_0010 == 0b10);
    }

    #[test]
    fn test_0xaa_tax_move_a_to_x() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x0A, 0xaa, 0x00]);

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
        assert_eq!(cpu.register_sr, 0x10);
        assert_eq!(cpu.register_a, 0x10);
    }

    #[test]
    fn test_0x90_bcc_branch_if_carry_clear() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x10, 0x90, 0x00]);

        assert_eq!(cpu.register_pc, 0x01);
    }

    #[test]
    fn test_bcs_branch_if_carry_set() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x10, 0x90, 0x01]);

        assert_eq!(cpu.register_pc, 0x01);
    }

    #[test]
    fn test_beq_branch_if_equal() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x10, 0x90, 0x02]);

        assert_eq!(cpu.register_pc, 0x01);
    }

    #[test]
    fn test_bit_test_zero_flag() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x00, 0x29, 0x00]);

        assert!(cpu.register_sr & 0b0000_0010 == 0b10);
    }

    #[test]
    fn test_bmi_branch_if_minus_flag_set() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x00, 0x30, 0x00]);

        assert_eq!(cpu.register_pc, 0x01);
    }

    #[test]
    fn test_bne_branch_if_not_equal() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x10, 0xD0, 0x02]);

        assert_eq!(cpu.register_pc, 0x01);
    }

    #[test]
    fn test_bpl_branch_if_plus_flag_clear() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x00, 0x10, 0x00]);

        assert_eq!(cpu.register_pc, 0x01);
    }
}
