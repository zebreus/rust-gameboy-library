use arr_macro::arr;
use std::fs;

use crate::memory::MemoryDevice;

/// Whether a version of the game is intended to be sold in Japan or elsewhere.
pub enum Destination {
    /// Japan (and possibly overseas)
    Japan,
    /// Overseas only
    OverseasOnly,
}

impl Into<Destination> for u8 {
    fn into(self) -> Destination {
        match self {
            0 => Destination::Japan,
            1 => Destination::OverseasOnly,
            _ => panic!("Invalid value for the cartridge destination"),
        }
    }
}

/// Indicates what kind of hardware is present on the cartridge
pub enum CartridgeType {
    /// RomOnly
    RomOnly,
    /// Mbc1
    Mbc1,
    /// Mbc1 Ram
    Mbc1Ram,
    /// Mbc1 Ram Battery
    Mbc1RamBattery,
    /// Mbc2
    Mbc2,
    /// Mbc2 Battery
    Mbc2Battery,
    /// Rom Ram
    RomRam,
    /// RomRamBattery
    RomRamBattery,
    /// Mmm01
    Mmm01,
    /// Mmm01 Ram
    Mmm01Ram,
    /// Mmm01 Ram Battery
    Mmm01RamBattery,
    /// Mbc3 Timer Battery
    Mbc3TimerBattery,
    /// Mbc3 Timer Ram Battery
    Mbc3TimerRamBattery,
    /// Mbc3
    Mbc3,
    /// Mbc3 Ram
    Mbc3Ram,
    /// Mbc3 Ram Battery
    Mbc3RamBattery,
    /// Mbc5
    Mbc5,
    /// Mbc5 Ram
    Mbc5Ram,
    /// Mbc5 Ram Battery
    Mbc5RamBattery,
    /// Mbc5 Rumble
    Mbc5Rumble,
    /// Mbc5 Rumble Ram
    Mbc5RumbleRam,
    /// Mbc5 Rumble Ram Battery
    Mbc5RumbleRamBattery,
    /// Mbc6
    Mbc6,
    /// Mbc7 Sensor Rumble Ram Battery
    Mbc7SensorRumbleRamBattery,
    /// Pocket Camera
    PocketCamera,
    /// Bandai Tama5
    BandaiTama5,
    /// Huc3
    Huc3,
    /// Huc1 Ram Battery
    Huc1RamBattery,
}

impl Into<CartridgeType> for u8 {
    fn into(self) -> CartridgeType {
        match self {
            0x00 => CartridgeType::RomOnly,
            0x01 => CartridgeType::Mbc1,
            0x02 => CartridgeType::Mbc1Ram,
            0x03 => CartridgeType::Mbc1RamBattery,
            0x05 => CartridgeType::Mbc2,
            0x06 => CartridgeType::Mbc2Battery,
            0x08 => CartridgeType::RomRam,
            0x09 => CartridgeType::RomRamBattery,
            0x0B => CartridgeType::Mmm01,
            0x0C => CartridgeType::Mmm01Ram,
            0x0D => CartridgeType::Mmm01RamBattery,
            0x0F => CartridgeType::Mbc3TimerBattery,
            0x10 => CartridgeType::Mbc3TimerRamBattery,
            0x11 => CartridgeType::Mbc3,
            0x12 => CartridgeType::Mbc3Ram,
            0x13 => CartridgeType::Mbc3RamBattery,
            0x19 => CartridgeType::Mbc5,
            0x1A => CartridgeType::Mbc5Ram,
            0x1B => CartridgeType::Mbc5RamBattery,
            0x1C => CartridgeType::Mbc5Rumble,
            0x1D => CartridgeType::Mbc5RumbleRam,
            0x1E => CartridgeType::Mbc5RumbleRamBattery,
            0x20 => CartridgeType::Mbc6,
            0x22 => CartridgeType::Mbc7SensorRumbleRamBattery,
            0xFC => CartridgeType::PocketCamera,
            0xFD => CartridgeType::BandaiTama5,
            0xFE => CartridgeType::Huc3,
            0xFF => CartridgeType::Huc1RamBattery,
            _ => panic!("Invalid value for the cartridge type"),
        }
    }
}

/// Represents a gameboy cartridge. Currently for debugging only
pub struct Cartridge {
    rom: [u8; 65536],
    /// The title of the ROm
    pub title: String,
    /// Indicates what kind of hardware is present on the cartridge
    pub cartridge_type: CartridgeType,
    /// Rom size in bytes
    pub rom_size: usize,
    /// Ram size in bytes
    pub ram_size: usize,
    /// Destination code
    pub destination: Destination,
    /// Version number of the game
    pub mask_rom_version_number: u8,
    /// An 8-bit checksum computed from the cartridge header bytes 0x0134 0x014C
    pub header_checksum: u8,
    /// A 16-bit checksum computed from the sum of all bytes in the cartridge
    pub cartridge_checksum: u16,
}

/// Decode the RAM size byte from the cartridge header into the number of RAM bytes.
fn decode_ram_size(byte: u8) -> usize {
    match byte {
        0 => 0,
        1 => 0,
        2 => 1 << 13,
        3 => 1 << 15,
        4 => 1 << 17,
        5 => 1 << 16,
        _ => panic!("Invalid value for the cartridge RAM size"),
    }
}

/// Decode the ROM size byte from the cartridge header into the number of ROM bytes.
fn decode_rom_size(byte: u8) -> usize {
    (1 << 15) * (1 << byte)
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
        let cartridge_type: CartridgeType = memory[0x0147].into();
        let rom_size = decode_rom_size(memory[0x0148]);
        let ram_size = decode_ram_size(memory[0x0149]);
        let destination: Destination = memory[0x014A].into();
        let mask_rom_version_number = memory[0x014C];
        let header_checksum = memory[0x014D];
        let cartridge_checksum = u16::from_be_bytes([memory[0x014E], memory[0x014F]]);

        Cartridge {
            rom: memory,
            title,
            cartridge_type,
            rom_size,
            ram_size,
            destination,
            mask_rom_version_number,
            header_checksum,
            cartridge_checksum,
        }
    }
    /// Check if the cartridge header is valid
    pub fn check_header_checksum(&self) -> Result<(), ()> {
        let checksum_bytes = &self.rom[0x0134..0x014C + 1];
        let checksum = checksum_bytes.iter().fold(0u8, |accumulator, byte| {
            accumulator.wrapping_sub(*byte).wrapping_sub(1)
        });
        if checksum != self.header_checksum {
            return Err(());
        }
        Ok(())
    }
    /// Check if the cartridge ROM is valid
    pub fn check_cartridge_checksum(&self) -> Result<(), ()> {
        let checksum_with_checksum_bytes = self.rom.iter().fold(0u16, |accumulator, byte| {
            accumulator.wrapping_add(*byte as u16)
        });
        let checksum = checksum_with_checksum_bytes
            .wrapping_sub(self.cartridge_checksum.to_ne_bytes()[0] as u16)
            .wrapping_sub(self.cartridge_checksum.to_ne_bytes()[1] as u16);
        if checksum != self.cartridge_checksum {
            return Err(());
        }
        Ok(())
    }
    /// Put the cartridge ROM into memory
    pub fn place_into_memory<M: MemoryDevice>(&self, memory: &mut M) {
        for (index, byte) in self.rom[0..=0x7FFF].iter().enumerate() {
            memory.write(index as u16, *byte);
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{memory::Memory, memory::MemoryDevice};

    use super::Cartridge;

    #[test]
    fn loads_correctly() {
        let cartridge = Cartridge::new();
        assert_eq!(cartridge.rom[0x0100], 0);
        assert_eq!(cartridge.rom[0x0101], 195);
    }

    #[test]
    fn test_cartridge_has_correct_header() {
        let cartridge = Cartridge::new();
        let check_result = cartridge.check_header_checksum();
        assert!(check_result.is_ok());
    }

    #[test]
    fn test_cartridge_has_correct_checksum() {
        let cartridge = Cartridge::new();
        let check_result = cartridge.check_cartridge_checksum();
        assert!(check_result.is_ok());
    }

    #[test]
    fn test_cartridge_can_be_placed_in_memory() {
        let cartridge = Cartridge::new();
        let mut memory = Memory::new();
        cartridge.place_into_memory(&mut memory);
        assert_eq!(memory.read(0x0100), 0);
        assert_eq!(memory.read(0x0101), 195);
    }
}
