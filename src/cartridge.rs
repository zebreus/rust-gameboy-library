use arr_macro::arr;
use std::{cmp::min, fs, mem::take, ops::RangeInclusive};

use crate::memory::{
    memory_addresses::{
        CARTRIDGE_CHECKSUM_LSB_ADDRESS, CARTRIDGE_CHECKSUM_MSB_ADDRESS, CARTRIDGE_HEADER_RANGE,
        CARTRIDGE_TYPE_ADDRESS, DESTINATION_COUNTRY_ADDRESS, FIRST_ROM_BANK,
        HEADER_CHECKSUM_ADDRESS, RAM_SIZE_ADDRESS, ROM_BANK_SIZE, ROM_SIZE_ADDRESS,
        ROM_VERSION_ADDRESS, SECOND_ROM_BANK, TITLE_RANGE,
    },
    Memory, MemoryDevice,
};

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
    rom: Vec<u8>,
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

    /// The current ram bank
    pub current_ram_bank: usize,
    /// The current second rom bank
    pub current_second_rom_bank: u8,
    /// If advanced banking is enabled
    pub advanced_banking_enabled: bool,
}

/// Decode the RAM size byte from the cartridge header into the number of RAM bytes.
pub fn decode_ram_size(byte: u8) -> usize {
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
pub fn decode_rom_size(byte: u8) -> usize {
    (1 << 15) * (1 << byte)
}

impl Cartridge {
    /// Loads a new test cartridge with a test ROM
    pub fn new() -> Cartridge {
        Self::load("test_roms/blargg/cpu_instrs/individual/06-ld r,r.gb")
    }
    /// Loads a new test cartridge with a ROM from a file
    pub fn load(path_to_rom: &str) -> Cartridge {
        let mut content = fs::read(path_to_rom).expect("Should exists");
        let memory = take(&mut content);

        let title_memory: &[u8] = &memory[TITLE_RANGE];
        let title_result = String::from_utf8(title_memory.into());
        let title = title_result.expect("The title should not contain invalid characters");
        let cartridge_type: CartridgeType = memory[CARTRIDGE_TYPE_ADDRESS].into();
        let rom_size = decode_rom_size(memory[ROM_SIZE_ADDRESS]);
        let ram_size = decode_ram_size(memory[RAM_SIZE_ADDRESS]);
        let destination: Destination = memory[DESTINATION_COUNTRY_ADDRESS].into();
        let mask_rom_version_number = memory[ROM_VERSION_ADDRESS];
        let header_checksum = memory[HEADER_CHECKSUM_ADDRESS];
        let cartridge_checksum = u16::from_be_bytes([
            memory[CARTRIDGE_CHECKSUM_MSB_ADDRESS],
            memory[CARTRIDGE_CHECKSUM_LSB_ADDRESS],
        ]);

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
            current_ram_bank: 0,
            current_second_rom_bank: 1,
            advanced_banking_enabled: false,
        }
    }
    /// Check if the cartridge header is valid
    pub fn check_header_checksum(&self) -> Result<(), ()> {
        let checksum_bytes = &self.rom[CARTRIDGE_HEADER_RANGE];
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
    pub fn place_into_memory(&self, memory: &mut Memory) {
        memory.memory[FIRST_ROM_BANK].copy_from_slice(&self.rom[FIRST_ROM_BANK]);
        memory.memory[SECOND_ROM_BANK].copy_from_slice(&self.rom[SECOND_ROM_BANK]);
    }
    fn load_second_rom_bank(&self, memory: &mut Memory) {
        let selected_rom_bank = if self.advanced_banking_enabled {
            self.current_second_rom_bank
        } else {
            self.current_second_rom_bank & 0b1111
        };
        let rom_bank_chunk = self
            .rom
            .chunks_exact(ROM_BANK_SIZE)
            .nth(selected_rom_bank as usize)
            .expect("Tried to load a nonexisting ROM bank");
        memory.memory[SECOND_ROM_BANK].copy_from_slice(rom_bank_chunk)
    }
    /// Process writes to the memory
    pub fn process_writes(&mut self, memory: &mut Memory) {
        let  Some(writes) = memory.mbc_registers.get_writes() else {return;};

        match self.cartridge_type {
            CartridgeType::RomRam | CartridgeType::RomRamBattery | CartridgeType::RomOnly => {}
            CartridgeType::Mbc1 | CartridgeType::Mbc1Ram | CartridgeType::Mbc1RamBattery => {
                for write in writes.iter() {
                    let (address, value) = write;
                    // const RAM_ENABLE: RangeInclusive<u16> = 0x0000..=0x1FFF;
                    // const ROM_SELECT: RangeInclusive<u16> = 0x2000..=0x3FFF;
                    // const RAM_SELECT: RangeInclusive<u16> = 0x4000..=0x5FFF;
                    // const BANKING_MODE_SELECT: RangeInclusive<u16> = 0x4000..=0x5FFF;
                    match *address {
                        0x0000..=0x1FFF => {
                            let enable_external_ram = (value & 0b1111) == 0xA;
                            memory.enable_external_ram = enable_external_ram
                        }
                        0x2000..=0x3FFF => {
                            let new_rom_bank = min(value & 0b11111, 1)
                                | (self.current_second_rom_bank as u8 & 0b1100000);
                            self.current_second_rom_bank = new_rom_bank;
                            self.load_second_rom_bank(memory);
                        }
                        0x4000..=0x5FFF => {
                            let new_rom_bank =
                                (value & 0b01100000) | (self.current_second_rom_bank & 0b1111);
                            self.current_second_rom_bank = new_rom_bank;
                            self.load_second_rom_bank(memory);
                        }
                        0x6000..=0x7FFF => {
                            self.advanced_banking_enabled = value % 2 != 0;
                            self.load_second_rom_bank(memory);
                        }
                        _ => {
                            panic!("Should not happen")
                        }
                    }
                }
            }
            CartridgeType::Mbc2 | CartridgeType::Mbc2Battery => {}
            CartridgeType::Mmm01 | CartridgeType::Mmm01Ram | CartridgeType::Mmm01RamBattery => {}
            CartridgeType::Mbc3TimerBattery
            | CartridgeType::Mbc3TimerRamBattery
            | CartridgeType::Mbc3
            | CartridgeType::Mbc3Ram
            | CartridgeType::Mbc3RamBattery => {}
            CartridgeType::Mbc5
            | CartridgeType::Mbc5Ram
            | CartridgeType::Mbc5RamBattery
            | CartridgeType::Mbc5Rumble
            | CartridgeType::Mbc5RumbleRam
            | CartridgeType::Mbc5RumbleRamBattery => {}
            CartridgeType::Mbc6 => {}
            CartridgeType::Mbc7SensorRumbleRamBattery => {}
            CartridgeType::PocketCamera => {}
            CartridgeType::BandaiTama5 => {}
            CartridgeType::Huc3 => {}
            CartridgeType::Huc1RamBattery => {}
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
        let mut memory = Memory::new_for_tests();
        cartridge.place_into_memory(&mut memory);
        assert_eq!(memory.read(0x0100), 0);
        assert_eq!(memory.read(0x0101), 195);
    }
}
