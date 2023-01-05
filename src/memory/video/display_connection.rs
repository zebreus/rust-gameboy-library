use image::{ImageBuffer, Rgba, RgbaImage};

/// The trait is used to connect to a display
pub trait DisplayConnection {
    /// Define the current color of a pixel
    fn set_pixel(&mut self, x: usize, y: usize, value: (u8, u8, u8, u8));
    /// Notify the display that the current frame has finished drawing
    fn finish_frame(&mut self);
}

/// A dummy display connection that does nothing
pub struct DummyDisplayConnection {}

impl DisplayConnection for DummyDisplayConnection {
    fn set_pixel(&mut self, _x: usize, _y: usize, _value: (u8, u8, u8, u8)) {}
    fn finish_frame(&mut self) {}
}

/// A display connection that creates a png for each frame
pub struct PngDisplayConnection {
    image: RgbaImage,
}

impl PngDisplayConnection {
    /// Create a new png display connection
    pub fn new() -> PngDisplayConnection {
        PngDisplayConnection {
            image: ImageBuffer::new(160, 144),
        }
    }
}

impl DisplayConnection for PngDisplayConnection {
    /// Define the current color of a pixel
    fn set_pixel(&mut self, x: usize, y: usize, value: (u8, u8, u8, u8)) {
        let (red, green, blue, alpha) = value;
        self.image
            .put_pixel(x as u32, y as u32, Rgba([red, green, blue, alpha]))
    }
    /// Notify the display that the current frame has finished drawing
    fn finish_frame(&mut self) {
        self.image.save("test.png").unwrap();
        self.image = ImageBuffer::new(160, 144);
    }
}

#[cfg(test)]
mod tests {
    use super::{DisplayConnection, PngDisplayConnection};

    #[test]
    fn test_png_display() {
        let mut png_display = PngDisplayConnection::new();
        png_display.set_pixel(2, 2, (127, 127, 127, 127));
        png_display.finish_frame();
    }
}
