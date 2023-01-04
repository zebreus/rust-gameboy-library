use std::ops::RangeInclusive;

use crate::memory::memory_addresses::{
    FIRST_BG_TILE_DATA_AREA, FIRST_BG_TILE_MAP_AREA, SECOND_BG_TILE_DATA_AREA,
    SECOND_BG_TILE_MAP_AREA,
};

/// Which background map is used for rendering.
#[derive(Debug, PartialEq)]
pub enum BackgroundTilemapArea {
    /// Use [FIRST_BG_TILE_MAP_AREA] as the source for the tilemap
    First,
    /// Use [SECOND_BG_TILE_MAP_AREA] as the source for the tilemap
    Second,
}

impl BackgroundTilemapArea {
    /// Get the corresponding memory area
    pub fn get_memory_area(&self) -> RangeInclusive<usize> {
        match self {
            BackgroundTilemapArea::First => FIRST_BG_TILE_MAP_AREA,
            BackgroundTilemapArea::Second => SECOND_BG_TILE_MAP_AREA,
        }
    }
}

/// Which tile data is used for rendering the background and window.
#[derive(Debug, PartialEq)]
pub enum BackgroundTileDataArea {
    /// Use [FIRST_BG_TILE_DATA_AREA] as the source for the tilemap
    First,
    /// Use [SECOND_BG_TILE_DATA_AREA] as the source for the tilemap
    Second,
}

impl BackgroundTileDataArea {
    /// Get the corresponding memory area
    pub fn get_memory_area(&self) -> RangeInclusive<usize> {
        match self {
            BackgroundTileDataArea::First => FIRST_BG_TILE_DATA_AREA,
            BackgroundTileDataArea::Second => SECOND_BG_TILE_DATA_AREA,
        }
    }
}

/// The size of the objects
#[derive(Debug, PartialEq)]
pub enum ObjectSize {
    /// A object is one tile big
    EightByEight,
    /// A object consists of two vertically stacked tiles.
    EightBySixteen,
}

impl ObjectSize {
    /// Get the current object height
    pub fn get_height(&self) -> u8 {
        match self {
            ObjectSize::EightByEight => 8,
            ObjectSize::EightBySixteen => 16,
        }
    }
    /// Get the current object width
    pub fn get_width(&self) -> u8 {
        8
    }
}

/// Represents the LCD control register
pub struct LcdControl {
    /// Controls whether the LCD is on and the PPU is active
    ///
    /// When disabled the display is blank. It takes one frame after enabling for the screen to draw again.
    pub lcd_ppu_enable: bool,
    /// Controls which memory area is used as the tilemap for rendering the window layer
    pub window_tilemap: BackgroundTilemapArea,
    /// Controls whether the window shall be displayed or not.
    ///
    /// Changing the value mid-frame triggers a more complex behaviour: See <https://gbdev.io/pandocs/Scrolling.html#ff4aff4b--wy-wx-window-y-position-x-position-plus-7>
    pub window_enable: bool,
    /// Controls which addressing mode the window and background use for their tiles.
    pub window_bg_tile_data: BackgroundTileDataArea,
    /// Controls which memory area is used as the tilemap for rendering the background layer
    pub background_tilemap: BackgroundTilemapArea,
    /// Controls whether sprites consist of one tile or two tiles stacked vertically.
    pub object_size: ObjectSize,
    /// Controls whether sprites are displayed or not.
    pub object_enable: bool,
    /// Controls whether the background and window are drawn or not.
    pub only_sprites: bool,
}

impl Into<LcdControl> for u8 {
    fn into(self) -> LcdControl {
        let lcd_ppu_enable = (self & 0b10000000) != 0;
        let window_tilemap = if (self & 0b01000000) != 0 {
            BackgroundTilemapArea::Second
        } else {
            BackgroundTilemapArea::First
        };
        let window_enable = (self & 0b00100000) != 0;
        let window_bg_tile_data = if (self & 0b00010000) != 0 {
            BackgroundTileDataArea::Second
        } else {
            BackgroundTileDataArea::First
        };
        let background_tilemap = if (self & 0b00001000) != 0 {
            BackgroundTilemapArea::Second
        } else {
            BackgroundTilemapArea::First
        };
        let object_size = if (self & 0b00000100) != 0 {
            ObjectSize::EightBySixteen
        } else {
            ObjectSize::EightByEight
        };
        let object_enable = (self & 0b00000010) != 0;
        let only_sprites = (self & 0b00000001) != 0;

        LcdControl {
            lcd_ppu_enable,
            window_tilemap,
            window_enable,
            window_bg_tile_data,
            background_tilemap,
            object_size,
            object_enable,
            only_sprites,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::memory::video::lcd_control::{
        BackgroundTileDataArea, BackgroundTilemapArea, LcdControl, ObjectSize,
    };

    #[test]
    fn lcd_control_parses_testvalue_correctly() {
        let lcd_control: LcdControl = 0b10101010.into();
        assert_eq!(lcd_control.lcd_ppu_enable, true);
        assert_eq!(lcd_control.window_tilemap, BackgroundTilemapArea::First);
        assert_eq!(lcd_control.window_enable, true);
        assert_eq!(
            lcd_control.window_bg_tile_data,
            BackgroundTileDataArea::First
        );
        assert_eq!(
            lcd_control.background_tilemap,
            BackgroundTilemapArea::Second
        );
        assert_eq!(lcd_control.object_size, ObjectSize::EightByEight);
        assert_eq!(lcd_control.object_enable, true);
        assert_eq!(lcd_control.only_sprites, false);
    }

    #[test]
    fn lcd_control_parses_inverted_testvalue_correctly() {
        let lcd_control: LcdControl = 0b01010101.into();
        assert_eq!(lcd_control.lcd_ppu_enable, false);
        assert_eq!(lcd_control.window_tilemap, BackgroundTilemapArea::Second);
        assert_eq!(lcd_control.window_enable, false);
        assert_eq!(
            lcd_control.window_bg_tile_data,
            BackgroundTileDataArea::Second
        );
        assert_eq!(lcd_control.background_tilemap, BackgroundTilemapArea::First);
        assert_eq!(lcd_control.object_size, ObjectSize::EightBySixteen);
        assert_eq!(lcd_control.object_enable, false);
        assert_eq!(lcd_control.only_sprites, true);
    }
}
