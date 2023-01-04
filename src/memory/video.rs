use crate::memory::{serial::serial_connection::SerialConnection, Memory};

use self::{display_connection::DisplayConnection, palette::Palette, tile::Tile};

use super::memory_addresses::{
    BACKGROUND_PALETTE_ADDRESS, FIRST_OBJECT_PALETTE_ADDRESS, SECOND_OBJECT_PALETTE_ADDRESS,
};

/// Logic related to tiles
pub mod tile;

/// Contains a trait for the connection to an actual display
pub mod display_connection;

/// Contains a struct for color palettes.
pub mod palette;

// struct TileMap {}

/// A collection of functions for video stuff
pub trait VideoFeatures {
    /// Parse all tiles into a vec
    fn get_tile_data(&self) -> Vec<Tile>;
}

impl<T: SerialConnection, D: DisplayConnection> VideoFeatures for Memory<T, D> {
    fn get_tile_data(&self) -> Vec<Tile> {
        let video_ram = &self.memory[0x8000..=0x8FFF];
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
}

impl<T: DisplayConnection> Video<T> {
    /// Create a new grapics struct
    pub fn new(display_connection: T) -> Self {
        Self {
            display_connection,
            background_palette: Palette::from_background_register(0),
            first_object_palette: Palette::from_object_register(0),
            second_object_palette: Palette::from_object_register(0),
        }
    }
}

impl<T: SerialConnection, D: DisplayConnection> Memory<T, D> {
    /// Process writes to the memory
    pub fn write_video(&mut self, address: u16, value: u8) -> Option<()> {
        match address as usize {
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
            _ => None,
        }
    }
    /// Will be called on every cycle
    pub fn cycle_video(&mut self) {}
}
