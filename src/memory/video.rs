use crate::memory::{serial::serial_connection::SerialConnection, Memory};

use self::{
    display_connection::DisplayConnection, lcd_control::LcdControl, palette::Palette, tile::Tile,
};

use super::memory_addresses::{
    BACKGROUND_PALETTE_ADDRESS, FIRST_OBJECT_PALETTE_ADDRESS,
    INITIATE_OBJECT_ATTRIBUTE_MEMORY_TRANSFER_ADDRESS, LCD_CONTROL_ADDRESS, LCD_STATUS_ADDRESS,
    SECOND_OBJECT_PALETTE_ADDRESS,
};

/// Logic related to tiles
pub mod tile;

/// Contains logic related to object attributes
pub mod object_attributes;

/// Logic related to tilemaps
pub mod tile_map;

/// Contains a trait for the connection to an actual display
pub mod display_connection;

/// Contains a struct for color palettes.
pub mod palette;

/// Contains logic for decoding the lcd control register.
pub mod lcd_control;

/// Contains logic for decoding the lcd status register.
pub mod lcd_status;

// struct TileMap {}

/// A collection of functions for video stuff
pub trait VideoFeatures {
    /// Parse all tiles into a vec
    fn get_tile_data(&self) -> Vec<Tile>;
}

impl<T: SerialConnection, D: DisplayConnection> VideoFeatures for Memory<T, D> {
    fn get_tile_data(&self) -> Vec<Tile> {
        let video_ram = &self.memory[0x8000..=0x97FF];
        let chunks = video_ram
            .chunks_exact(16)
            .map(|chunk| Tile::from(chunk.try_into().unwrap()))
            .collect::<Vec<Tile>>();
        return chunks;
    }
}

#[cfg(test)]
mod tests {

    use crate::memory::{video::VideoFeatures, Memory};

    #[test]
    fn the_number_of_returned_tiles_looks_correct() {
        let memory = Memory::new_for_tests();
        let tiles = memory.get_tile_data();
        assert_eq!(tiles.len(), 256);
    }
}

/// A running object attribute memory transfer
pub struct ObjectAttributeMemoryTransfer {
    /// The current source address
    pub current_source_address: usize,
    /// The current target address in the object attribute memory
    pub current_target_address: usize,
}

/// Represents the gpu
pub struct Video<T: DisplayConnection> {
    /// Pixels get drawn onto this display
    pub display_connection: T,
    /// The current background color palette
    pub background_palette: Palette,
    /// The current first object color palette
    pub first_object_palette: Palette,
    /// The current second object color palette
    pub second_object_palette: Palette,
    /// The current state of the LCD control register
    pub current_lcd_control: LcdControl,
    /// The current state of the LCD status register
    pub current_lcd_status: LcdControl,
    /// Set to a None if no transfer is in progress.
    pub current_transfer: Option<ObjectAttributeMemoryTransfer>,
}

impl<T: DisplayConnection> Video<T> {
    /// Create a new grapics struct
    pub fn new(display_connection: T) -> Self {
        Self {
            display_connection,
            background_palette: Palette::from_background_register(0),
            first_object_palette: Palette::from_object_register(0),
            second_object_palette: Palette::from_object_register(0),
            current_lcd_control: 0.into(),
            current_lcd_status: 0.into(),
            current_transfer: None,
        }
    }
}

impl<T: SerialConnection, D: DisplayConnection> Memory<T, D> {
    /// Process writes to the memory
    pub fn write_video(&mut self, address: u16, value: u8) -> Option<()> {
        match address as usize {
            LCD_CONTROL_ADDRESS => {
                self.graphics.current_lcd_control = value.into();
                self.memory[LCD_CONTROL_ADDRESS] = value;
                return Some(());
            }
            LCD_STATUS_ADDRESS => {
                let old_value = self.memory[LCD_STATUS_ADDRESS];
                let new_value = (value & 0b11111000) | (old_value & 0b00000111);
                self.graphics.current_lcd_status = new_value.into();
                self.memory[LCD_STATUS_ADDRESS] = new_value;
                return Some(());
            }
            BACKGROUND_PALETTE_ADDRESS => {
                self.graphics.background_palette = Palette::from_background_register(value);
                self.memory[BACKGROUND_PALETTE_ADDRESS] = value;
                return Some(());
            }
            FIRST_OBJECT_PALETTE_ADDRESS => {
                self.graphics.first_object_palette = Palette::from_object_register(value);
                self.memory[FIRST_OBJECT_PALETTE_ADDRESS] = value;
                return Some(());
            }
            SECOND_OBJECT_PALETTE_ADDRESS => {
                self.graphics.second_object_palette = Palette::from_object_register(value);
                self.memory[SECOND_OBJECT_PALETTE_ADDRESS] = value;
                return Some(());
            }
            INITIATE_OBJECT_ATTRIBUTE_MEMORY_TRANSFER_ADDRESS => {
                self.graphics.current_transfer = Some(ObjectAttributeMemoryTransfer {
                    current_source_address: u16::from_be_bytes([value, 0]) as usize,
                    current_target_address: 0xFF00,
                });
                self.memory[SECOND_OBJECT_PALETTE_ADDRESS] = value;
                return Some(());
            }
            _ => None,
        }
    }
    /// Will be called on every cycle
    pub fn cycle_video(&mut self) {
        match &mut self.graphics.current_transfer {
            Some(transfer) => {
                self.memory[transfer.current_target_address] =
                    self.memory[transfer.current_source_address];
                transfer.current_source_address += 1;
                transfer.current_target_address += 1;
                if transfer.current_target_address > 0xFF9F {
                    self.graphics.current_transfer = None;
                }
            }
            None => {}
        }
    }
}
