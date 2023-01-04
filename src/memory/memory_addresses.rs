use std::ops::RangeInclusive;

/// The size of a ROM bank
pub const ROM_BANK_SIZE: usize = 0x4000;
/// The first ROM bank is usually mounted to this memory address
pub const FIRST_ROM_BANK: RangeInclusive<usize> = 0x0000..=0x3FFF;
/// The second ROM bank is usually mounted to this memory address
pub const SECOND_ROM_BANK: RangeInclusive<usize> = 0x4000..=0x7FFF;
/// The cartridge RAM is accessible here.
pub const EXTERNAL_RAM_BANK: RangeInclusive<usize> = 0xA000..=0xBFFF;

/// The first area that can be used as tile data for the window and background layer
pub const FIRST_BG_TILE_DATA_AREA: RangeInclusive<usize> = 0x8800..=0x97FF;
/// The second area that can be used as tile data for the window and background layer
pub const SECOND_BG_TILE_DATA_AREA: RangeInclusive<usize> = 0x8000..=0x8FFF;

/// The area containing the tile data for the objects layer
pub const OBJECT_TILE_DATA_AREA: RangeInclusive<usize> = 0x8000..=0x8FFF;

/// The area containing the object attribute memory
#[doc(alias = "OAM")]
pub const OBJECT_ATTRIBUTE_MEMORY_AREA: RangeInclusive<usize> = 0xFE00..=0xFE9F;

/// The first area that can be used as a tilemap for the window or background
pub const FIRST_BG_TILE_MAP_AREA: RangeInclusive<usize> = 0x9800..=0x9BFF;
/// The second area that can be used as a tilemap for the window or background
pub const SECOND_BG_TILE_MAP_AREA: RangeInclusive<usize> = 0x9C00..=0x9FFF;

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

/// Contains the serial data.
///
/// Before a transfer it contains the data being send, which is replaced with the received data during/after transfer.
#[doc(alias = "SB")]
pub const SERIAL_DATA_ADDRESS: usize = 0xFF01;
/// Control the serial connection.
///
/// See <https://gbdev.io/pandocs/Serial_Data_Transfer_(Link_Cable).html#ff02--sc-serial-transfer-control> for more details.
#[doc(alias = "SC")]
pub const SERIAL_CONTROL_ADDRESS: usize = 0xFF02;
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
pub const TIMER_CONTROL_ADDRESS: usize = 0xFF07;

/// The main lcd control register
#[doc(alias = "LCDC")]
pub const LCD_CONTROL_ADDRESS: usize = 0xFF40;
/// The current LCD status is stored here
#[doc(alias = "STAT")]
pub const LCD_STATUS_ADDRESS: usize = 0xFF41;
/// Write a value 0xNN here to start copying the area `0xNN00..=0xNN9F` to `0xFE00..=0xFE9F` ([OBJECT_ATTRIBUTE_MEMORY_AREA])
///
/// The transfer takes 160 cycles. While the transfer is running the CPU can only access HRAM.
#[doc(alias = "DMA")]
pub const INITIATE_OBJECT_ATTRIBUTE_MEMORY_TRANSFER_ADDRESS: usize = 0xFF46;
/// Write here to set the [Palette] for the background and the window layer
#[doc(alias = "BGP")]
pub const BACKGROUND_PALETTE_ADDRESS: usize = 0xFF47;
/// Write here to set the first [Palette] for the object layer
#[doc(alias = "OBP1")]
pub const FIRST_OBJECT_PALETTE_ADDRESS: usize = 0xFF48;
/// Write here to set the second [Palette] for the object layer
#[doc(alias = "OBP2")]
pub const SECOND_OBJECT_PALETTE_ADDRESS: usize = 0xFF49;

/// This address should always read `0xff`.
///
/// I got that info from https://www.reddit.com/r/EmuDev/comments/ipap0w/comment/g76m04i
///
/// Apparently there are tests in the mooneye test suite that verify the correct values for all IO registers.
pub const ALWAYS_RETURNS_FF_ADDRESS: usize = 0xFF4D;
