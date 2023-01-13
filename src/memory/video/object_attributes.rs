use crate::memory::{
    memory_addresses::OBJECT_ATTRIBUTE_MEMORY_AREA, serial::serial_connection::SerialConnection,
    MemoryController,
};

use super::display_connection::DisplayConnection;

/// Which color palette should be used for an object
pub enum ObjectPalette {
    /// The one from [FIRST_OBJECT_PALETTE_ADDRESS]
    First,
    /// The one from [SECOND_OBJECT_PALETTE_ADDRESS]
    Second,
}

/// Represents an entry in the object attribute memory
pub struct ObjectAttributes {
    /// The x position on screen + 8
    pub x_position: u8,
    /// The y position on screen + 16
    pub y_position: u8,
    /// The index of the tile in the tile data from [OBJECT_TILE_DATA_AREA]
    pub tile: u8,
    /// Draw the object below background and window if this is set to true.
    pub draw_under_bg_and_window: bool,
    /// Flip horizontally
    pub x_flip: bool,
    /// Flip vertically
    pub y_flip: bool,
    /// Select the color palette for this object
    pub palette: ObjectPalette,
}

impl Into<ObjectAttributes> for &[u8] {
    fn into(self) -> ObjectAttributes {
        let array: [u8; 4] = self.try_into().unwrap();
        let object_attributes = array.into();
        object_attributes
    }
}

impl Into<ObjectAttributes> for [u8; 4] {
    fn into(self) -> ObjectAttributes {
        let x_position = self[0];
        let y_position = self[1];
        let tile = self[2];
        let draw_under_bg_and_window = (self[3] & 0b10000000) != 0;
        let x_flip = (self[3] & 0b01000000) != 0;
        let y_flip = (self[3] & 0b00100000) != 0;
        let palette = if (self[3] & 0b00010000) != 0 {
            ObjectPalette::Second
        } else {
            ObjectPalette::First
        };

        ObjectAttributes {
            x_position,
            y_position,
            tile,
            draw_under_bg_and_window,
            x_flip,
            y_flip,
            palette,
        }
    }
}

impl<T: SerialConnection, D: DisplayConnection> MemoryController<T, D> {
    /// Get the object attributes from object attribute memory
    pub fn get_object_attributes(&self) -> Vec<ObjectAttributes> {
        let object_attribute_memory = &self.memory[OBJECT_ATTRIBUTE_MEMORY_AREA];
        let chunks = object_attribute_memory
            .chunks_exact(4)
            .map(|chunk| chunk.into())
            .collect::<Vec<ObjectAttributes>>();
        return chunks;
    }

    // TODO: Add tests
    /// Get the [ObjectAttributes] for all objects that are visible on a given line.
    pub fn get_relevant_object_attributes(&self, line: u8) -> Vec<ObjectAttributes> {
        let object_attributes = self.get_object_attributes();
        let object_height = self.graphics.current_lcd_control.object_size.get_height();
        let filtered_object_attributes = object_attributes
            .into_iter()
            .filter(|attributes| {
                let first_line_visible = attributes.y_position <= (line + 16);
                let last_line_visible = (attributes.y_position + object_height) > (line + 16);
                // let x_visible = (attributes.x_position != 0) && (attributes.x_position < 168);
                return first_line_visible && last_line_visible /* && x_visible */;
            })
            .collect::<Vec<ObjectAttributes>>();
        return filtered_object_attributes;
    }
}
