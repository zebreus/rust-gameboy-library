use std::{cmp::max, fs, mem::take};

use crate::memory::{
    memory_addresses::{
        CARTRIDGE_CHECKSUM_LSB_ADDRESS, CARTRIDGE_CHECKSUM_MSB_ADDRESS, CARTRIDGE_HEADER_RANGE,
        CARTRIDGE_TYPE_ADDRESS, DESTINATION_COUNTRY_ADDRESS, FIRST_ROM_BANK,
        HEADER_CHECKSUM_ADDRESS, RAM_SIZE_ADDRESS, ROM_BANK_SIZE, ROM_SIZE_ADDRESS,
        ROM_VERSION_ADDRESS, SECOND_ROM_BANK, TITLE_RANGE,
    },
    Memory,
};

use self::{cartridge_type::CartridgeType, destination::Destination};

use super::serial::serial_connection::SerialConnection;

/// Contains information about cartridge types
pub mod cartridge_type;
/// Contains information about destination regions
pub mod destination;

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
    pub fn place_into_memory(&self, memory: &mut [u8; 65536]) {
        memory[FIRST_ROM_BANK].copy_from_slice(&self.rom[FIRST_ROM_BANK]);
        memory[SECOND_ROM_BANK].copy_from_slice(&self.rom[SECOND_ROM_BANK]);
    }
    fn load_second_rom_bank(&self, memory: &mut [u8; 65536]) {
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
        memory[SECOND_ROM_BANK].copy_from_slice(rom_bank_chunk)
    }
}

impl<T: SerialConnection> Memory<T> {
    /// Process writes to the memory
    pub fn write_cartridge(&mut self, address: u16, value: u8) -> Option<()> {
        match self.cartridge.cartridge_type {
            CartridgeType::RomRam | CartridgeType::RomRamBattery | CartridgeType::RomOnly => {}
            CartridgeType::Mbc1 | CartridgeType::Mbc1Ram | CartridgeType::Mbc1RamBattery => {
                // const RAM_ENABLE: RangeInclusive<u16> = 0x0000..=0x1FFF;
                // const ROM_SELECT: RangeInclusive<u16> = 0x2000..=0x3FFF;
                // const RAM_SELECT: RangeInclusive<u16> = 0x4000..=0x5FFF;
                // const BANKING_MODE_SELECT: RangeInclusive<u16> = 0x4000..=0x5FFF;
                match address {
                    0x0000..=0x1FFF => {
                        let enable_external_ram = (value & 0b1111) == 0xA;
                        self.enable_external_ram = enable_external_ram
                    }
                    0x2000..=0x3FFF => {
                        let new_rom_bank = max(value & 0b11111, 1)
                            | (self.cartridge.current_second_rom_bank as u8 & 0b1100000);
                        self.cartridge.current_second_rom_bank = new_rom_bank;
                        self.cartridge.load_second_rom_bank(&mut self.memory);
                    }
                    0x4000..=0x5FFF => {
                        let new_rom_bank = (value & 0b01100000)
                            | (self.cartridge.current_second_rom_bank & 0b1111);
                        self.cartridge.current_second_rom_bank = new_rom_bank;
                        self.cartridge.load_second_rom_bank(&mut self.memory);
                    }
                    0x6000..=0x7FFF => {
                        self.cartridge.advanced_banking_enabled = value % 2 != 0;
                        self.cartridge.load_second_rom_bank(&mut self.memory);
                    }
                    _ => {}
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
        };
        match address {
            0x0000..=0x7FFF => Some(()),
            0x8000..=0xFFFF => None,
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
        cartridge.place_into_memory(&mut memory.memory);
        assert_eq!(memory.read(0x0100), 0);
        assert_eq!(memory.read(0x0101), 195);
    }
}
