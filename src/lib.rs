pub mod cpu;
pub mod opcodes;
pub mod memmory;

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
    fn test_0xe9_sbc_without_overflow() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x01, 0xe9, 0x01]);

        assert_eq!(cpu.register_a, 0x00);
        assert!(cpu.register_sr & 0b0000_0001 == 0b1);
    }

    #[test]
    fn test_0xe9_sbc_overflow() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x01, 0xe9, 0x02]);

        assert_eq!(cpu.register_a, 0xFF);
        assert_eq!(cpu.register_sr & 0b0000_0001, 0b0);
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

    #[test]
    fn test_bvc_branch_if_overflow_clear() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x00, 0x50, 0x00]);

        assert_eq!(cpu.register_pc, 0x01);
    }

    #[test]
    fn test_bvs_branch_if_overflow_set() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x00, 0x70, 0x00]);

        assert_eq!(cpu.register_pc, 0x01);
    }

    #[test]
    fn test_0xc9_cpm_compare_memory_with_accumulator() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa9, 0x10, 0xC9, 0x10]);

        assert_eq!(cpu.register_sr & 0b0000_0001, 0b1);
    }

    #[test]
    fn test_0xe0_cpx_compare_x_with_memory() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa2, 0x10, 0xE0, 0x10]);

        assert_eq!(cpu.register_sr & 0b0000_0001, 0b1);
    }

    #[test]
    fn test_0xc0_cpy_compare_y_with_memmory() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa0, 0x10, 0xC0, 0x10]);

        assert_eq!(cpu.register_sr & 0b0000_0001, 0b1);
    }

    #[test]
    fn test_dec_0xc6_decrement_memory() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa2, 0x00, 0xC6, 0x00, 0xC6, 0x00]);

        assert_eq!(cpu.memmory.array[0x00], 0xFE);
    }

    #[test]
    fn test_dex_decrement_x() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xca, 0xca]);

        assert_eq!(cpu.register_x, 0xFE);
    }

    #[test]
    fn test_dey_decrement_y() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0x88, 0x88]);

        assert_eq!(cpu.register_y, 0xFE);
    }

    #[test]
    fn test_eor_0x41_eor_accumulator_with_memory() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa2, 0x05, 0x41, 0x05]);

        assert_eq!(cpu.register_a, 0x00);
    }

    #[test]
    fn test_0xe6_inc_increment_memory() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xa2, 0x00, 0xE6, 0x00, 0xE6, 0x00]);

        assert_eq!(cpu.memmory.array[0x00], 0x02);
    }

    #[test]
    fn test_inx_increment_x() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xe8, 0xe8]);

        assert_eq!(cpu.register_x, 0x02);
    }

    #[test]
    fn test_iny_increment_y() {
        let mut cpu = CPU::new();
        cpu.load_and_run(vec![0xc8, 0xc8]);

        assert_eq!(cpu.register_y, 0x02);
    }
}
