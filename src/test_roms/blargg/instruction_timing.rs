#[cfg(test)]
mod tests {
    use super::super::test_blargg_rom;

    #[test]
    fn all_instructions_test() {
        test_blargg_rom("test_roms/blargg/instr_timing/instr_timing.gb", 1000000);
    }
}
