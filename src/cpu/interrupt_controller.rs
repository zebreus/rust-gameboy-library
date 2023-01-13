use crate::memory::{
    memory_addresses::{INTERRUPT_ENABLE_ADDRESS, INTERRUPT_FLAG_ADDRESS},
    Memory, MemoryDevice,
};

use super::Interrupt;

impl Memory {
    /// Set the interrupt enable flag for a specific interrupt.
    ///
    /// This is equivalent to modifying the IE register at memory address 0xffff
    pub fn write_interrupt_enable(&mut self, interrupt: Interrupt, value: bool) {
        let old_byte = self.data[INTERRUPT_ENABLE_ADDRESS];
        let new_byte = if value {
            old_byte | (interrupt as u8)
        } else {
            old_byte & !(interrupt as u8)
        };
        self.data[INTERRUPT_ENABLE_ADDRESS] = new_byte
    }
    /// Read if a interrupt is enabled.
    ///
    /// This is equivalent to reading the IE register at memory address 0xffff
    pub fn read_interrupt_enable(&self, interrupt: Interrupt) -> bool {
        self.data[INTERRUPT_ENABLE_ADDRESS] & (interrupt as u8) != 0
    }
    /// Set the interrupt flag for a specific interrupt.
    ///
    /// This is equivalent to modifying the IF register at memory address 0xff0f
    pub fn write_interrupt_flag(&mut self, interrupt: Interrupt, value: bool) {
        let old_byte = self.data[INTERRUPT_FLAG_ADDRESS];
        let new_byte = if value {
            old_byte | (interrupt as u8)
        } else {
            old_byte & !(interrupt as u8)
        };
        self.data[INTERRUPT_FLAG_ADDRESS] = new_byte;
    }
    /// Read if a interrupt is requested
    ///
    /// This is equivalent to reading the IE register at memory address 0xffff
    pub fn read_interrupt_flag(&self, interrupt: Interrupt) -> bool {
        self.data[INTERRUPT_FLAG_ADDRESS] & (interrupt as u8) != 0
    }
    /// Get the complete IE

    pub fn read_interrupt_enable_register(&self) -> u8 {
        self.data[INTERRUPT_ENABLE_ADDRESS]
    }
    /// Get the complete IF
    pub fn read_interrupt_flag_register(&self) -> u8 {
        self.data[INTERRUPT_FLAG_ADDRESS]
    }

    /// Set the complete IE
    pub fn write_interrupt_enable_register(&mut self, value: u8) {
        self.data[INTERRUPT_ENABLE_ADDRESS] = value;
    }

    /// Set the complete IF
    pub fn write_interrupt_flag_register(&mut self, value: u8) {
        self.data[INTERRUPT_FLAG_ADDRESS] = value;
    }
}

/// Trait for accessing the interrupt control registers on memory
pub trait InterruptController {
    /// Set the interrupt enable flag for a specific interrupt.
    ///
    /// This is equivalent to modifying the IE register at memory address 0xffff
    fn write_interrupt_enable(&mut self, interrupt: Interrupt, value: bool);
    /// Read if a interrupt is enabled.
    ///
    /// This is equivalent to reading the IE register at memory address 0xffff
    fn read_interrupt_enable(&self, interrupt: Interrupt) -> bool;
    /// Get the complete IE
    fn read_interrupt_enable_register(&self) -> u8;
    /// Get the complete IF
    fn read_interrupt_flag_register(&self) -> u8;
    /// Set the complete IE
    fn write_interrupt_enable_register(&mut self, value: u8);
    /// Set the complete IF
    fn write_interrupt_flag_register(&mut self, value: u8);
    /// Set the interrupt flag for a specific interrupt.
    ///
    /// This is equivalent to modifying the IF register at memory address 0xff0f
    fn write_interrupt_flag(&mut self, interrupt: Interrupt, value: bool);
    /// Read if a interrupt is requested
    ///
    /// This is equivalent to reading the IE register at memory address 0xffff
    fn read_interrupt_flag(&self, interrupt: Interrupt) -> bool;
}

impl<M: MemoryDevice> InterruptController for M {
    fn write_interrupt_enable(&mut self, interrupt: Interrupt, value: bool) {
        let old_byte = self.read(INTERRUPT_ENABLE_ADDRESS as u16);
        let new_byte = if value {
            old_byte | (interrupt as u8)
        } else {
            old_byte & !(interrupt as u8)
        };
        self.write(INTERRUPT_ENABLE_ADDRESS as u16, new_byte)
    }
    fn read_interrupt_enable(&self, interrupt: Interrupt) -> bool {
        self.read(INTERRUPT_ENABLE_ADDRESS as u16) & (interrupt as u8) != 0
    }

    fn write_interrupt_flag(&mut self, interrupt: Interrupt, value: bool) {
        let old_byte = self.read(INTERRUPT_FLAG_ADDRESS as u16);
        let new_byte = if value {
            old_byte | (interrupt as u8)
        } else {
            old_byte & !(interrupt as u8)
        };
        self.write(INTERRUPT_FLAG_ADDRESS as u16, new_byte)
    }
    fn read_interrupt_flag(&self, interrupt: Interrupt) -> bool {
        self.read(INTERRUPT_FLAG_ADDRESS as u16) & (interrupt as u8) != 0
    }
    fn read_interrupt_enable_register(&self) -> u8 {
        self.read(INTERRUPT_ENABLE_ADDRESS as u16)
    }
    fn read_interrupt_flag_register(&self) -> u8 {
        self.read(INTERRUPT_FLAG_ADDRESS as u16)
    }

    fn write_interrupt_enable_register(&mut self, value: u8) {
        self.write(INTERRUPT_ENABLE_ADDRESS as u16, value);
    }

    fn write_interrupt_flag_register(&mut self, value: u8) {
        self.write(INTERRUPT_FLAG_ADDRESS as u16, value);
    }
}
