use num_enum::{IntoPrimitive, TryFromPrimitive};

/// A mode that the ppu can be in
#[derive(TryFromPrimitive, Debug, IntoPrimitive, PartialEq, Clone, Copy)]
#[repr(u8)]
pub enum PpuMode {
    /// Pause after every line
    ///
    /// Duration: 85-208 dots. 456 - Duration of [Oam].  (22-52 cycles)
    HBlank = 0,
    /// Pause after every frame
    ///
    /// Duration: 4560 dots (1140 cycles)
    VBlank = 1,
    /// Searching OAM for OBJs whose Y coordinate overlap this line
    ///
    /// Duration: 80 dots (20 cycles)
    ///
    /// OAM is inaccessible in this mode
    Oam = 2,
    /// Reading OAM and VRAM to generate the picture
    ///
    /// Duration: 168-291 dots depending on the sprite count (42-73 cycles)
    ///
    /// VRAM and OAM are inaccessible in this mode
    TransferringData = 3,
}

/// Represents the LCD status register
pub struct LcdStatus {
    /// Whether [Interrupt::Stat] is triggered by the [line_y_equal_flag]
    pub line_y_stat_interrupt_enable: bool,
    /// Whether [Interrupt::Stat] is triggered by the [PpuMode::Oam]
    pub oam_stat_interrupt_enable: bool,
    /// Whether [Interrupt::Stat] is triggered by the [PpuMode::VBlank]
    pub vblank_stat_interrupt_enable: bool,
    /// Whether [Interrupt::Stat] is triggered by the [PpuMode::HBlank]
    pub hblank_stat_interrupt_enable: bool,
    // TODO: Find proper names for LYC=LY
    /// Is set to true if LYC=LY
    pub line_y_equal_flag: bool,
    /// The current ppu mode
    pub ppu_mode: PpuMode,
}

impl Into<LcdStatus> for u8 {
    fn into(self) -> LcdStatus {
        let line_y_stat_interrupt_enable = (self & 0b01000000) != 0;
        let oam_stat_interrupt_enable = (self & 0b00100000) != 0;
        let vblank_stat_interrupt_enable = (self & 0b00010000) != 0;
        let hblank_stat_interrupt_enable = (self & 0b00001000) != 0;
        let line_y_equal_flag = (self & 0b00000100) != 0;
        let ppu_mode = (self & 0b00000011)
            .try_into()
            .expect("Every value should be valid");

        LcdStatus {
            line_y_stat_interrupt_enable,
            oam_stat_interrupt_enable,
            vblank_stat_interrupt_enable,
            hblank_stat_interrupt_enable,
            line_y_equal_flag,
            ppu_mode,
        }
    }
}

impl Into<u8> for &LcdStatus {
    fn into(self) -> u8 {
        let line_y_stat_interrupt_enable: u8 = if self.line_y_stat_interrupt_enable {
            0b01000000
        } else {
            0
        };
        let oam_stat_interrupt_enable: u8 = if self.oam_stat_interrupt_enable {
            0b00100000
        } else {
            0
        };
        let vblank_stat_interrupt_enable: u8 = if self.vblank_stat_interrupt_enable {
            0b00010000
        } else {
            0
        };
        let hblank_stat_interrupt_enable: u8 = if self.hblank_stat_interrupt_enable {
            0b00001000
        } else {
            0
        };
        let line_y_equal_flag: u8 = if self.line_y_equal_flag {
            0b00000100
        } else {
            0
        };
        let ppu_mode: u8 = (self.ppu_mode).into();

        return ppu_mode
            | line_y_equal_flag
            | hblank_stat_interrupt_enable
            | vblank_stat_interrupt_enable
            | oam_stat_interrupt_enable
            | line_y_stat_interrupt_enable;
    }
}

#[cfg(test)]
mod tests {
    use crate::memory::video::lcd_status::LcdStatus;

    #[test]
    fn lcd_status_converts_back_to_the_same_value() {
        let original = 0b01010101u8;
        let lcd_status: LcdStatus = original.into();
        let reencoded: u8 = (&lcd_status).into();
        assert_eq!(original, reencoded);
    }

    #[test]
    fn lcd_status_always_encodes_with_bit_7_as_zero() {
        let original = 0b11111111u8;
        let lcd_status: LcdStatus = original.into();
        let reencoded: u8 = (&lcd_status).into();
        assert_eq!(0b01111111, reencoded);
    }
}
