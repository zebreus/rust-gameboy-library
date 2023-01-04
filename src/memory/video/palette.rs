/// Represents a palette color
#[derive(Debug, PartialEq)]
#[repr(u8)]
pub enum Color {
    /// White.
    White = 0,
    /// Light gray
    LightGray = 1,
    /// Dark gray
    DarkGray = 2,
    /// Black
    Black = 3,
    /// Transparent.
    Transparent,
}

impl Into<Color> for u8 {
    fn into(self) -> Color {
        match self {
            0 => Color::White,
            1 => Color::LightGray,
            2 => Color::DarkGray,
            3 => Color::Black,
            _ => panic!("Can only convert values between 0 and 3 to colors."),
        }
    }
}

/// Represents a color palette
pub struct Palette {
    /// The color palette
    pub colors: [Color; 4],
}

impl Palette {
    /// Create a palette from the value written to the background palette register.
    ///
    /// The palette register is [BACKGROUND_PALETTE_ADDRESS]
    ///
    pub fn from_background_register(palette_register: u8) -> Palette {
        let first_color: Color = (palette_register & 0b00000011).into();
        let second_color: Color = ((palette_register & 0b00001100) >> 2).into();
        let third_color: Color = ((palette_register & 0b00110000) >> 4).into();
        let fourth_color: Color = ((palette_register & 0b11000000) >> 6).into();

        Palette {
            colors: [first_color, second_color, third_color, fourth_color],
        }
    }

    /// Create a palette from the value written to a object palette register.
    ///
    /// The palette registers are [FIRST_OBJECT_PALETTE_ADDRESS] and [SECOND_OBJECT_PALETTE_ADDRESS]
    ///
    /// This is similar to [from_background_register], but the first color is always [Color::Transparent].
    ///
    pub fn from_object_register(palette_register: u8) -> Palette {
        let second_color: Color = ((palette_register & 0b00001100) >> 2).into();
        let third_color: Color = ((palette_register & 0b00110000) >> 4).into();
        let fourth_color: Color = ((palette_register & 0b11000000) >> 6).into();

        Palette {
            colors: [Color::Transparent, second_color, third_color, fourth_color],
        }
    }
    /// Get the color for a color index
    pub fn get_color(&self, index: usize) -> &Color {
        return self
            .colors
            .get(index)
            .expect("The index should be no bigger than 3");
    }
}
