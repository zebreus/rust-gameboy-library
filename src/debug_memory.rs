use crate::memory::MemoryDevice;
use arr_macro::arr;

/// Debug memory does simple reads and writes to 64kb of memory. It also prints every read or write
pub struct DebugMemory {
    memory: [u8; 65536],
}

impl DebugMemory {
    /// Create a new DebugMemory filled with `0`.
    pub fn new() -> DebugMemory {
        DebugMemory {
            memory: arr![0; 65536],
        }
    }

    /// Create a new DebugMemory. `init` will be placed at memory address 0. The remaining memory will be filled with `0`.
    pub fn new_with_init(init: &[u8]) -> DebugMemory {
        let mut memory = DebugMemory {
            memory: arr![0; 65536],
        };
        for (dst, src) in memory.memory.iter_mut().zip(init) {
            *dst = *src;
        }
        return memory;
    }
}

impl MemoryDevice for DebugMemory {
    fn read(&self, address: u16) -> u8 {
        let value = self.memory[address as usize];
        println!("Read {}({:#04x}) from {:#06x}", value, value, address);
        return value;
    }
    fn write(&mut self, address: u16, value: u8) -> () {
        println!(
            "Write value {}({:#04x}) from {:#06x}",
            value, value, address
        );
        self.memory[address as usize] = value;
    }
}

#[cfg(test)]
mod tests {
    use crate::{debug_memory::DebugMemory, memory::MemoryDevice};

    #[test]
    fn can_read_written_value() {
        let mut debug_memory = DebugMemory::new();
        debug_memory.write(0, 99);
        let read_value = debug_memory.read(0);
        assert_eq!(read_value, 99);
    }

    #[test]
    fn reads_zero_in_unused_memory() {
        let debug_memory = DebugMemory::new();
        assert_eq!(debug_memory.read(0), 0);
        assert_eq!(debug_memory.read(65535), 0);
        assert_eq!(debug_memory.read(10), 0);
        assert_eq!(debug_memory.read(65000), 0);
        assert_eq!(debug_memory.read(30000), 0);
    }

    #[test]
    fn initializing_memory_works() {
        let debug_memory = DebugMemory::new_with_init(&[7, 5, 0, 255]);
        assert_eq!(debug_memory.read(0), 7);
        assert_eq!(debug_memory.read(1), 5);
        assert_eq!(debug_memory.read(2), 0);
        assert_eq!(debug_memory.read(3), 255);
        assert_eq!(debug_memory.read(4), 0);
    }
}
