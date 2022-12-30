use std::ops::RangeInclusive;

/// The size of a ROM bank
pub const ROM_BANK_SIZE: usize = 0x4000;
/// The first ROM bank is usually mounted to this memory address
pub const FIRST_ROM_BANK: RangeInclusive<usize> = 0x0000..=0x3FFF;
/// The second ROM bank is usually mounted to this memory address
pub const SECOND_ROM_BANK: RangeInclusive<usize> = 0x4000..=0x7FFF;
/// The cartridge RAM is accessible here.
pub const EXTERNAL_RAM_BANK: RangeInclusive<usize> = 0xA000..=0xBFFF;

/// Each cartridge contains a header, located here. The cartridge header provides the following information about the game itself and the hardware it expects to run on.
pub const CARTRIDGE_HEADER_RANGE: RangeInclusive<usize> = 0x0134..=0x014C;
/// The memory at this range contains the title of the game.
/// In older games the next byte is also part of the title.
/// The cartridge title is stored here
pub const TITLE_RANGE: RangeInclusive<usize> = 0x0134..=0x0142;
/// This byte indicates what kind of hardware is present on the cartridge. See [CartridgeType] for the possible values
pub const CARTRIDGE_TYPE_ADDRESS: usize = 0x0147;
/// This byte indicates how much ROM is present on the cartridge. See [decode_rom_size] for the possible values.
pub const ROM_SIZE_ADDRESS: usize = 0x0148;
/// This byte indicates how much RAM is present on the cartridge. See [decode_ram_size] for the possible values.
pub const RAM_SIZE_ADDRESS: usize = 0x0149;
/// This byte specifies whether this version of the game is intended to be sold in Japan or elsewhere. See [Destination] for the possible values.
pub const DESTINATION_COUNTRY_ADDRESS: usize = 0x014A;
/// This byte indicates the version of the ROM. It is usually set to 0.
pub const ROM_VERSION_ADDRESS: usize = 0x014C;
/// This byte contains an 8-bit checksum computed from the cartridge header bytes. You can check how the checksum is calculated in the implementation of [Cartridge::check_header_checksum]
pub const HEADER_CHECKSUM_ADDRESS: usize = 0x014D;
/// The most significant byte of the cartridge checksum.
///
/// The checksum is computed as the sum of all the bytes of the cartridge ROM (except these two checksum bytes). Our implementation of that is at [Cartridge::check_cartridge_checksum]
pub const CARTRIDGE_CHECKSUM_MSB_ADDRESS: usize = 0x014E;
/// The least significant byte of the cartridge checksum.
///
/// The checksum is computed as the sum of all the bytes of the cartridge ROM (except these two checksum bytes). Our implementation of that is at [Cartridge::check_cartridge_checksum]
pub const CARTRIDGE_CHECKSUM_LSB_ADDRESS: usize = 0x014F;

/// The timer divider register is stored at this address.
///
/// See [Timer](super::Timer) for details
#[doc(alias = "DIV")]
pub const TIMER_DIVIDER_ADDRESS: usize = 0xFF04;
/// See [Timer](super::Timer) for details
#[doc(alias = "TIMA")]
pub const TIMER_COUNTER_ADDRESS: usize = 0xFF05;
/// See [Timer](super::Timer) for details
#[doc(alias = "TMA")]
pub const TIMER_MODULO_ADDRESS: usize = 0xFF06;
/// See [Timer](super::Timer) for details
#[doc(alias = "TCA")]
pub const TIMER_CONTROL_ADDRESS: usize = 0xFF06;
