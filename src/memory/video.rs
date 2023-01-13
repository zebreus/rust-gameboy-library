use crate::cpu::Interrupt;

use self::{
    display_connection::DisplayConnection,
    lcd_control::LcdControl,
    lcd_status::{LcdStatus, PpuMode},
    object_attributes::ObjectAttributes,
    palette::Palette,
};

use super::{
    memory_addresses::{
        BACKGROUND_PALETTE_ADDRESS, CURRENT_LINE_ADDRESS, FIRST_OBJECT_PALETTE_ADDRESS,
        INITIATE_OBJECT_ATTRIBUTE_MEMORY_TRANSFER_ADDRESS, INTERRUPT_LINE_ADDRESS,
        LCD_CONTROL_ADDRESS, LCD_STATUS_ADDRESS, SECOND_OBJECT_PALETTE_ADDRESS,
    },
    Memory,
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
    pub current_lcd_status: LcdStatus,
    /// Set to a None if no transfer is in progress.
    pub current_transfer: Option<ObjectAttributeMemoryTransfer>,

    /// Cycles in current mode
    pub cycles_on_current_line: usize,
    /// The line that is currently rendered
    pub current_line: u8,
    /// The objects that are relevant for the current line
    pub current_objects: Vec<ObjectAttributes>,
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
            cycles_on_current_line: 0,
            current_line: 0,
            current_objects: Vec::new(),
        }
    }

    /// Advance to the next line
    ///
    /// Resets the cycle counter and sets `current_lcd_status` into the correct mode.
    ///
    /// The updated `current_lcd_status` has to be written to memory afterwards
    pub fn advance_to_next_line(&mut self) {
        self.current_line = self.current_line + 1;
        self.cycles_on_current_line = 0;

        if self.current_line >= 154 {
            self.current_line = 0;
            self.current_lcd_status.ppu_mode = PpuMode::Oam;
            return;
        }
        if self.current_line >= 144 {
            self.current_lcd_status.ppu_mode = PpuMode::VBlank;
            return;
        }
        self.current_lcd_status.ppu_mode = PpuMode::Oam;
    }

    /// Handle writes to the video memory
    ///
    /// Returns [Ok] if the write was handled
    pub fn write(&mut self, memory: &mut Memory, address: u16, value: u8) -> Option<()> {
        match address as usize {
            LCD_CONTROL_ADDRESS => {
                self.current_lcd_control = value.into();
                memory.data[LCD_CONTROL_ADDRESS] = value;
                return Some(());
            }
            LCD_STATUS_ADDRESS => {
                let old_value = memory.data[LCD_STATUS_ADDRESS];
                let new_value = (value & 0b11111000) | (old_value & 0b00000111);
                self.current_lcd_status = new_value.into();
                memory.data[LCD_STATUS_ADDRESS] = new_value;
                return Some(());
            }
            CURRENT_LINE_ADDRESS => Some(()),
            INTERRUPT_LINE_ADDRESS => {
                memory.data[INTERRUPT_LINE_ADDRESS] = value;
                Some(())
            }
            BACKGROUND_PALETTE_ADDRESS => {
                self.background_palette = Palette::from_background_register(value);
                memory.data[BACKGROUND_PALETTE_ADDRESS] = value;
                return Some(());
            }
            FIRST_OBJECT_PALETTE_ADDRESS => {
                self.first_object_palette = Palette::from_object_register(value);
                memory.data[FIRST_OBJECT_PALETTE_ADDRESS] = value;
                return Some(());
            }
            SECOND_OBJECT_PALETTE_ADDRESS => {
                self.second_object_palette = Palette::from_object_register(value);
                memory.data[SECOND_OBJECT_PALETTE_ADDRESS] = value;
                return Some(());
            }
            INITIATE_OBJECT_ATTRIBUTE_MEMORY_TRANSFER_ADDRESS => {
                self.current_transfer = Some(ObjectAttributeMemoryTransfer {
                    current_source_address: u16::from_be_bytes([value, 0]) as usize,
                    current_target_address: 0xFF00,
                });
                memory.data[SECOND_OBJECT_PALETTE_ADDRESS] = value;
                return Some(());
            }
            _ => None,
        }
    }
    /// Will be called on every cycle
    pub fn cycle(&mut self, memory: &mut Memory) {
        const CYCLES_PER_LINE: usize = 114;

        match &mut self.current_transfer {
            Some(transfer) => {
                memory.data[transfer.current_target_address] =
                    memory.data[transfer.current_source_address];
                transfer.current_source_address += 1;
                transfer.current_target_address += 1;
                if transfer.current_target_address > 0xFF9F {
                    self.current_transfer = None;
                }
            }
            None => {}
        }

        if !self.current_lcd_control.lcd_ppu_enable {
            return;
        }

        self.cycles_on_current_line += 1;

        match self.current_lcd_status.ppu_mode {
            PpuMode::Oam => {
                if self.cycles_on_current_line == 1 {
                    self.current_objects =
                        memory.get_relevant_object_attributes(self, self.current_line);
                }

                if self.cycles_on_current_line >= 20 {
                    self.current_lcd_status.ppu_mode = PpuMode::TransferringData;
                    memory.data[LCD_STATUS_ADDRESS] = (&self.current_lcd_status).into();
                }
            }
            PpuMode::TransferringData => {
                if self.cycles_on_current_line == 21 {
                    self.render_line(memory);
                }
                if self.cycles_on_current_line >= 70 {
                    self.current_lcd_status.ppu_mode = PpuMode::HBlank;
                    memory.data[LCD_STATUS_ADDRESS] = (&self.current_lcd_status).into();
                }
            }
            PpuMode::HBlank => {
                if self.cycles_on_current_line >= CYCLES_PER_LINE {
                    self.advance_to_next_line();
                    memory.data[LCD_STATUS_ADDRESS] = (&self.current_lcd_status).into();
                    memory.data[CURRENT_LINE_ADDRESS] = self.current_line;
                }
            }
            PpuMode::VBlank => {
                if self.current_line == 144 && self.cycles_on_current_line == 1 {
                    self.display_connection.finish_frame();
                    memory.write_interrupt_flag(Interrupt::VBlank, true);
                }
                if self.cycles_on_current_line >= CYCLES_PER_LINE {
                    self.advance_to_next_line();
                    memory.data[LCD_STATUS_ADDRESS] = (&self.current_lcd_status).into();
                    memory.data[CURRENT_LINE_ADDRESS] = self.current_line;
                }
            }
        }
    }

    /// Render the current line into the video connection.
    pub fn render_line(&mut self, memory: &mut Memory) {
        // let background_tilemap =
        //     self.get_tile_map(&self.current_lcd_control.background_tilemap);
        let window_tilemap = memory.get_tile_map(&self.current_lcd_control.window_tilemap);
        let window_background_tile_data =
            memory.get_tile_data(&self.current_lcd_control.window_bg_tile_data);
        // let object_tile_data = self.get_tile_data(&TileDataArea::First);
        // let window_palette = &self.background_palette;
        let background_palette = &self.background_palette;

        let line = self.current_line;

        // let relevant_window_tiles = window_tilemap.get_tiles_for_line(line);
        let relevant_background_tiles = window_tilemap.get_tiles_for_line(line);

        // for x in 0..160 {
        //     let window_tile_index = x / 8;
        //     let window_tile_row = x % 8;
        // }
        let y_offset_in_tile = line % 8;
        for (index, tile) in relevant_background_tiles.iter().enumerate() {
            let tile_data = &window_background_tile_data[*tile as usize];
            let pixels = tile_data.get_line(y_offset_in_tile as usize);
            for (pixel_index, pixel) in pixels.iter().enumerate() {
                let x = (index * 8) + pixel_index;
                if x >= 160 {
                    break;
                }
                let color = background_palette.get_color(*pixel as usize).get_rgba();
                if *pixel != 0 {
                    let _x = 8;
                }
                self.display_connection.set_pixel(x, line as usize, color)
            }
        }
    }
}
