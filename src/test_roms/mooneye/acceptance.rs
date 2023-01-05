#[cfg(test)]
mod tests {
    use super::super::test_mooneye_rom;

    #[test]
    fn daa_test() {
        test_mooneye_rom("test_roms/mooneye/acceptance/instr/daa.gb", 1000000);
    }
    // #[test]
    // fn acceptance_tests() {
    //     for entry in glob("test_roms/mooneye/acceptance/*.gb").expect("Failed to read glob pattern")
    //     {
    //         match entry {
    //             Ok(path) => {
    //                 println!("Testing: {}", path.display());
    //                 let path = path.display().to_string();
    //                 test_mooneye_rom(path.as_str(), 1000000);
    //             }
    //             _ => {}
    //         }
    //     }
    // }
}
