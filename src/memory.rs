/// Address for the interrupt enable register.
pub const INTERRUPT_ENABLE_ADDRESS: u16 = 0xFFFF;
/// Address for the interrupt flags register.
pub const INTERRUPT_FLAG_ADDRESS: u16 = 0xFF0F;

/// The trait for things that can be accessed via memory
pub trait MemoryDevice {
    /// Read a byte from an address
    fn read(&self, address: u16) -> u8;
    /// Write a byte to an address
    fn write(&mut self, address: u16, value: u8) -> ();
    // TODO: Question: Is there a way to make the return type of the read function generic (i8 or u8) and automatically infer which one is needed?
    /// Read a signed byte from an address
    fn read_signed(&self, address: u16) -> i8 {
        i8::from_ne_bytes([self.read(address)])
    }
    /// Write a signed byte to an address
    fn write_signed(&mut self, address: u16, value: i8) -> () {
        self.write(address, value.to_ne_bytes()[0]);
    }
}
