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
pub struct PngDisplayConnection {}

impl PngDisplayConnection {
    /// Create a new png display connection
    pub fn new() -> PngDisplayConnection {
        PngDisplayConnection {}
    }
}

impl DisplayConnection for PngDisplayConnection {
    /// Define the current color of a pixel
    fn set_pixel(&mut self, _x: usize, _y: usize, _value: (u8, u8, u8, u8)) {
        todo!();
    }
    /// Notify the display that the current frame has finished drawing
    fn finish_frame(&mut self) {
        todo!();
    }
}
