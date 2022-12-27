use arr_macro::arr;

/// Debug memory does simple reads and writes to 64kb of memory. It also prints every read or write
pub struct Memory {
    /// The memory
    pub memory: [u8; 65536],
    serial_line: String,
}

impl Memory {
    /// Create a new Memory filled with `0`.
    pub fn new() -> Memory {
        Memory {
            memory: arr![0; 65536],
            serial_line: String::new(),
        }
    }

    /// Create a new Memory. `init` will be placed at memory address 0. The remaining memory will be filled with `0`.
    pub fn new_with_init(init: &[u8]) -> Memory {
        let mut memory = Memory {
            memory: arr![0; 65536],
            serial_line: String::new(),
        };
        for (dst, src) in memory.memory.iter_mut().zip(init) {
            *dst = *src;
        }
        return memory;
    }

    pub fn get_video_memory(&self) -> &[u8] {
        return &self.memory[0x8000..=0x9FFF];
    }
}

impl MemoryDevice for Memory {
    fn read(&self, address: u16) -> u8 {
        let value = self.memory[address as usize];
        // if (address == 0xff01) || (address == 0xff02) {
        //     println!("Read value {}({:#04x}) from {:#06x}", value, value, address);
        // }
        // println!("Read {}({:#04x}) from {:#06x}", value, value, address);
        return value;
    }
    fn write(&mut self, address: u16, value: u8) -> () {
        // println!(
        //     "Write value {}({:#04x}) from {:#06x}",
        //     value, value, address
        // );
        if address == 0xff01 {
            let character = value as char;
            match character {
                '\n' => {
                    println!("Serial: {}", self.serial_line);
                    self.serial_line = String::new();
                }
                _ => {
                    self.serial_line.push(character);
                }
            }
        }
        self.memory[address as usize] = value;
    }
}

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

#[cfg(test)]
mod tests {
    use crate::{memory::Memory, memory::MemoryDevice};

    #[test]
    fn can_read_written_value() {
        let mut debug_memory = Memory::new();
        debug_memory.write(0, 99);
        let read_value = debug_memory.read(0);
        assert_eq!(read_value, 99);
    }

    #[test]
    fn reads_zero_in_unused_memory() {
        let debug_memory = Memory::new();
        assert_eq!(debug_memory.read(0), 0);
        assert_eq!(debug_memory.read(65535), 0);
        assert_eq!(debug_memory.read(10), 0);
        assert_eq!(debug_memory.read(65000), 0);
        assert_eq!(debug_memory.read(30000), 0);
    }

    #[test]
    fn initializing_memory_works() {
        let debug_memory = Memory::new_with_init(&[7, 5, 0, 255]);
        assert_eq!(debug_memory.read(0), 7);
        assert_eq!(debug_memory.read(1), 5);
        assert_eq!(debug_memory.read(2), 0);
        assert_eq!(debug_memory.read(3), 255);
        assert_eq!(debug_memory.read(4), 0);
    }
}
