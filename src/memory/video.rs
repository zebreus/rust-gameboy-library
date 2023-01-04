use crate::memory::{serial::serial_connection::SerialConnection, Memory};

use self::tile::Tile;

/// Logic related to tiles
pub mod tile;

mod display_connection;

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
