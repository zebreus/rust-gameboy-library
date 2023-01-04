use arr_macro::arr;

/// Contains named memory addresses as constants
pub mod memory_addresses;

/// Contains functionality related to the timer
pub mod timer;

/// Contains cartridge functionality
pub mod cartridge;

/// Contains the serial connection
pub mod serial;

/// Contains the GPU and video memory
pub mod video;

use timer::Timer;

use self::{
    cartridge::Cartridge,
    memory_addresses::ALWAYS_RETURNS_FF_ADDRESS,
    serial::{
        serial_connection::{LoggerSerialConnection, SerialConnection},
        Serial,
    },
};

/// Debug memory does simple reads and writes to 64kb of memory. It also prints every read or write
pub struct Memory<T: SerialConnection> {
    /// The memory
    pub memory: [u8; 65536],
    /// Treat everything as ram
    pub test_mode: bool,
    /// Counts how often `Passed` was printed to serial
    pub printed_passed: u32,
    /// The timer is stored here because it is probably the best place for it.
    pub timer: Timer,
    /// Contains data related to the serial connection
    pub serial: Serial<T>,
    /// Contains a cartridge
    pub cartridge: Cartridge,
}

impl<T: SerialConnection> Memory<T> {
    /// Create a new Memory filled with `0`.
    pub fn new_with_connections(connection: Option<T>) -> Memory<T> {
        Memory {
            memory: arr![0; 65536],
            test_mode: false,
            printed_passed: 0,
            timer: Timer::new(),
            serial: Serial::new(connection),
            cartridge: Cartridge::new(),
        }
    }

    /// Should be called on every cycle
    pub fn process_cycle(&mut self) {
        self.cycle_timer();
        self.cycle_serial();
    }
}

impl Memory<LoggerSerialConnection> {
    /// Create a new Memory filled with `0`.
    pub fn new() -> Memory<LoggerSerialConnection> {
        Memory {
            memory: arr![0; 65536],
            test_mode: false,
            printed_passed: 0,
            timer: Timer::new(),
            serial: Serial::new(Some(LoggerSerialConnection::new())),
            cartridge: Cartridge::new(),
        }
    }
    /// Create a new Memory filled with `0`.
    pub fn new_for_tests() -> Memory<LoggerSerialConnection> {
        Memory {
            memory: arr![0; 65536],
            test_mode: true,
            printed_passed: 0,
            timer: Timer::new(),
            serial: Serial::new(Some(LoggerSerialConnection::new())),
            cartridge: Cartridge::new(),
        }
    }

    /// Create a new Memory. `init` will be placed at memory address 0. The remaining memory will be filled with `0`.
    pub fn new_with_init(init: &[u8]) -> Memory<LoggerSerialConnection> {
        let mut memory = Memory {
            memory: arr![0; 65536],
            test_mode: true,
            printed_passed: 0,
            timer: Timer::new(),
            serial: Serial::new(Some(LoggerSerialConnection::new())),
            cartridge: Cartridge::new(),
        };
        for (dst, src) in memory.memory.iter_mut().zip(init) {
            *dst = *src;
        }
        return memory;
    }
}

impl<T: SerialConnection> MemoryDevice for Memory<T> {
    fn read(&self, address: u16) -> u8 {
        match address as usize {
            0xFF44 => 0xFF,
            ALWAYS_RETURNS_FF_ADDRESS => 0xFF,
            _ => self.memory[address as usize],
        }
        // if (address == 0xff01) || (address == 0xff02) {
        //     println!("Read value {}({:#04x}) from {:#06x}", value, value, address);
        // }
        // println!("Read {}({:#04x}) from {:#06x}", value, value, address);
    }
    fn write(&mut self, address: u16, value: u8) -> () {
        // println!(
        //     "Write value {}({:#04x}) from {:#06x}",
        //     value, value, address
        // );
        if self.test_mode {
            self.memory[address as usize] = value;
        }
        let write_timer_result = self.write_timer(address, value);
        if write_timer_result.is_some() {
            return;
        }
        let write_serial_result = self.write_serial(address, value);
        if write_serial_result.is_some() {
            return;
        }
        let write_cartridge_result = self.write_cartridge(address, value);
        if write_cartridge_result.is_some() {
            return;
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
        let mut debug_memory = Memory::new_for_tests();
        debug_memory.write(0, 99);
        let read_value = debug_memory.read(0);
        assert_eq!(read_value, 99);
    }

    #[test]
    fn reads_zero_in_unused_memory() {
        let debug_memory = Memory::new_for_tests();
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
