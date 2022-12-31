#[cfg(test)]
mod tests {
    use super::super::test_blargg_rom;

    #[test]
    fn special_test() {
        test_blargg_rom(
            "test_roms/blargg/cpu_instrs/individual/01-special.gb",
            10000000,
        );
    }

    #[test]
    fn interrupt_test() {
        test_blargg_rom(
            "test_roms/blargg/cpu_instrs/individual/02-interrupts.gb",
            100000000,
        );
    }

    #[test]
    fn stackpointer_operations_test() {
        test_blargg_rom(
            "test_roms/blargg/cpu_instrs/individual/03-op sp,hl.gb",
            10000000,
        );
    }

    #[test]
    fn load_immediate_to_register_test() {
        test_blargg_rom(
            "test_roms/blargg/cpu_instrs/individual/04-op r,imm.gb",
            10000000,
        );
    }

    #[test]
    fn fifth_instruction_test() {
        test_blargg_rom(
            "test_roms/blargg/cpu_instrs/individual/05-op rp.gb",
            10000000,
        );
    }

    #[test]
    fn ld_test() {
        test_blargg_rom(
            "test_roms/blargg/cpu_instrs/individual/06-ld r,r.gb",
            1000000,
        );
    }

    #[test]
    fn branching_test() {
        test_blargg_rom(
            "test_roms/blargg/cpu_instrs/individual/07-jr,jp,call,ret,rst.gb",
            100000000,
        );
    }

    #[test]
    fn misc_instructions_test() {
        test_blargg_rom(
            "test_roms/blargg/cpu_instrs/individual/08-misc instrs.gb",
            1000000,
        );
    }

    #[test]
    fn register_operations_test() {
        test_blargg_rom(
            "test_roms/blargg/cpu_instrs/individual/09-op r,r.gb",
            10000000,
        );
    }

    #[test]
    fn bitwise_operations_test() {
        test_blargg_rom(
            "test_roms/blargg/cpu_instrs/individual/10-bit ops.gb",
            20000000,
        );
    }

    #[test]
    fn accumulator_with_hl_test() {
        test_blargg_rom(
            "test_roms/blargg/cpu_instrs/individual/11-op a,(hl).gb",
            50000000,
        );
    }
}
