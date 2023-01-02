#[cfg(test)]
mod tests {
    use super::super::test_blargg_rom;

    #[test]
    fn memory_timing_full_test() {
        test_blargg_rom("test_roms/blargg/mem_timing/mem_timing.gb", 10000000);
    }

    #[test]
    fn memory_read_timing_test() {
        test_blargg_rom(
            "test_roms/blargg/mem_timing/individual/01-read_timing.gb",
            10000000,
        );
    }

    #[test]
    fn memory_write_timing_test() {
        test_blargg_rom(
            "test_roms/blargg/mem_timing/individual/02-write_timing.gb",
            10000000,
        );
    }
    #[test]
    fn memory_modify_timing_test() {
        test_blargg_rom(
            "test_roms/blargg/mem_timing/individual/03-modify_timing.gb",
            10000000,
        );
    }
}
