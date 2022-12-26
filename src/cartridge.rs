use arr_macro::arr;
use std::fs;

use crate::memory_device::MemoryDevice;

/// Represents a gameboy cartridge. Currently for debugging only
pub struct Cartridge {
    memory: [u8; 65536],
    /// The title of the ROm
    pub title: String,
}

impl Cartridge {
    /// Loads a new test cartridge with a test ROM
    pub fn new() -> Cartridge {
        let content = fs::read("ld_test.gb").expect("Should exists");
        let mut memory: [u8; 65536] = arr![0; 65536];
        // TODO: This code is ugly, learn better rust
        for (index, byte) in content.iter().enumerate() {
            memory[index] = *byte;
        }

        let title_memory: &[u8] = &memory[0x0134..0x0143];
        let title_result = String::from_utf8(title_memory.into());
        let title = title_result.expect("The title should not contain invalid characters");
        Cartridge { memory, title }
    }
}

impl MemoryDevice for Cartridge {
    fn read(&self, address: u16) -> u8 {
        let value = self.memory[address as usize];
        println!("Read {}({:#04x}) from {:#06x}", value, value, address);
        return value;
    }
    fn write(&mut self, address: u16, value: u8) -> () {
        println!(
            "Write value {}({:#04x}) from {:#06x}",
            value, value, address
        );
        self.memory[address as usize] = value;
    }
}

#[cfg(test)]
mod tests {
    use crate::memory_device::MemoryDevice;

    use super::Cartridge;

    #[test]
    fn loads_correctly() {
        let cartridge = Cartridge::new();
        assert_eq!(cartridge.read(0x0100), 0);
        assert_eq!(cartridge.read(0x0101), 195);
    }
}
