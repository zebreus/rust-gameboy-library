use num_enum::{IntoPrimitive, TryFromPrimitive};

use crate::cpu::{interrupt_controller::InterruptController, Interrupt};

use super::{
    memory_addresses::{
        TIMER_CONTROL_ADDRESS, TIMER_COUNTER_ADDRESS, TIMER_DIVIDER_ADDRESS, TIMER_MODULO_ADDRESS,
    },
    serial::serial_connection::SerialConnection,
    Memory,
};

#[derive(TryFromPrimitive, Debug, IntoPrimitive)]
#[repr(u8)]
enum InputClock {
    Hz4096 = 0b00,
    Hz262144 = 0b01,
    Hz65536 = 0b10,
    Hz16384 = 0b11,
}

impl InputClock {
    /// Get the division factor from 1 Mhz
    pub fn divider(&self) -> u64 {
        match self {
            InputClock::Hz4096 => 256,
            InputClock::Hz262144 => 4,
            InputClock::Hz65536 => 16,
            InputClock::Hz16384 => 64,
        }
    }
}
/// Represents the timer and interrupt controller
pub struct Timer {
    enabled: bool,
    input_clock: InputClock,
    counter: u64,
    tima: u8,
}

impl Timer {
    /// Create a new timer with default values
    pub fn new() -> Timer {
        Timer {
            enabled: false,
            input_clock: InputClock::Hz4096,
            counter: 0,
            tima: 0,
        }
    }

    fn configure_from_control_register_value(&mut self, value: u8) {
        let input_clock_part = value & 0b00000011;
        let input_clock: InputClock = input_clock_part
            .try_into()
            .expect("Input clock should always be in range");
        self.input_clock = input_clock;

        let is_enabled = (value & 0b00000100) == 0b00000100;
        self.enabled = is_enabled;
    }

    /// Read an address
    pub fn read<T: SerialConnection>(_memory: &Memory<T>, address: u16) -> Option<u8> {
        match address as usize {
            TIMER_DIVIDER_ADDRESS => None,
            TIMER_COUNTER_ADDRESS => None,
            TIMER_MODULO_ADDRESS => None,
            TIMER_CONTROL_ADDRESS => None,
            _ => None,
        }
    }
}

impl<T: SerialConnection> Memory<T> {
    /// Process writes to the memory
    pub fn write_timer(&mut self, address: u16, value: u8) -> Option<()> {
        let timer = &mut self.timer;
        match address as usize {
            TIMER_DIVIDER_ADDRESS => {
                self.memory[TIMER_DIVIDER_ADDRESS] = 0;
                Some(())
            }
            TIMER_COUNTER_ADDRESS => {
                self.memory[TIMER_COUNTER_ADDRESS] = value;
                timer.tima = value;
                Some(())
            }
            TIMER_MODULO_ADDRESS => {
                self.memory[TIMER_MODULO_ADDRESS] = value;
                Some(())
            }
            TIMER_CONTROL_ADDRESS => {
                timer.configure_from_control_register_value(value);
                self.memory[TIMER_CONTROL_ADDRESS] = value;
                Some(())
            }
            _ => None,
        }
    }
    /// Should be called on every cycle
    pub fn cycle_timer(&mut self) {
        let timer = &mut self.timer;
        timer.counter = timer.counter.wrapping_add(1);
        if timer.counter % 64 == 0 {
            self.memory[TIMER_DIVIDER_ADDRESS] = self.memory[TIMER_DIVIDER_ADDRESS].wrapping_add(1);
        }

        if timer.enabled && (timer.counter % timer.input_clock.divider() == 0) {
            let (new_timer_counter, overflow) =
                self.memory[TIMER_COUNTER_ADDRESS].overflowing_add(1);
            self.memory[TIMER_COUNTER_ADDRESS] = new_timer_counter;
            if overflow {
                self.memory[TIMER_COUNTER_ADDRESS] = self.memory[TIMER_MODULO_ADDRESS];
                self.write_interrupt_flag(Interrupt::Timer, true);
            }
            self.timer.tima = self.memory[TIMER_COUNTER_ADDRESS];
        }
    }
}
