use crate::memory::{serial::serial_connection::SerialConnection, Memory};

use super::{display_connection::DisplayConnection, lcd_control::BackgroundTilemapArea};

/// A tile map represents a 32x32 grid of tiles
pub struct TileMap {
    /// The array with the tile ids.
    pub tiles: [u8; 1024],
}

impl<T: SerialConnection, D: DisplayConnection> Memory<T, D> {
    /// Get the [TileMap] from a memory area.
    pub fn get_tile_map(&self, area: BackgroundTilemapArea) -> TileMap {
        let memory_area = area.get_memory_area();
        let tiles: [u8; 1024] = self.memory[memory_area]
            .try_into()
            .expect("Incorrect length. Should not happen.");
        TileMap { tiles }
    }
}
