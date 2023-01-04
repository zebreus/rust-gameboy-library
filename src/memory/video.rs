use crate::memory::{serial::serial_connection::SerialConnection, Memory};

use self::{palette::Palette, tile::Tile};

/// Logic related to tiles
pub mod tile;

mod display_connection;

/// Contains a struct for color palettes.
pub mod palette;

// struct TileMap {}

/// A collection of functions for video stuff
pub trait VideoFeatures {
    /// Parse all tiles into a vec
    fn get_tile_data(&self) -> Vec<Tile>;
}

impl<T: SerialConnection> VideoFeatures for Memory<T> {
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
pub struct Video {
    /// The current background color palette
    pub background_palette: Palette,
    /// The current first object color palette
    pub first_object_palette: Palette,
    /// The current second object color palette
    pub second_object_palette: Palette,
}

impl Video {
    /// Create a new grapics struct
    pub fn new() -> Video {
        Video {
            background_palette: Palette::from_background_register(0),
            first_object_palette: Palette::from_object_register(0),
            second_object_palette: Palette::from_object_register(0),
        }
    }
}
