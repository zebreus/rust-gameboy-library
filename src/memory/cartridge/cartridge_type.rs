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
