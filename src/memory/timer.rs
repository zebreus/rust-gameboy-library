use num_enum::{IntoPrimitive, TryFromPrimitive};

use crate::cpu::Interrupt;

use super::{
    memory_addresses::{
        TIMER_CONTROL_ADDRESS, TIMER_COUNTER_ADDRESS, TIMER_DIVIDER_ADDRESS, TIMER_MODULO_ADDRESS,
    },
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

    /// Process writes to the memory
    pub fn write(&mut self, memory: &mut Memory, address: u16, value: u8) -> Option<()> {
        match address as usize {
            TIMER_DIVIDER_ADDRESS => {
                memory.data[TIMER_DIVIDER_ADDRESS] = 0;
                Some(())
            }
            TIMER_COUNTER_ADDRESS => {
                memory.data[TIMER_COUNTER_ADDRESS] = value;
                self.tima = value;
                Some(())
            }
            TIMER_MODULO_ADDRESS => {
                memory.data[TIMER_MODULO_ADDRESS] = value;
                Some(())
            }
            TIMER_CONTROL_ADDRESS => {
                self.configure_from_control_register_value(value);
                memory.data[TIMER_CONTROL_ADDRESS] = value;
                Some(())
            }
            _ => None,
        }
    }
    /// Should be called on every cycle
    pub fn cycle(&mut self, memory: &mut Memory) {
        self.counter = self.counter.wrapping_add(1);
        if self.counter % 64 == 0 {
            memory.data[TIMER_DIVIDER_ADDRESS] = memory.data[TIMER_DIVIDER_ADDRESS].wrapping_add(1);
        }

        if self.enabled && (self.counter % self.input_clock.divider() == 0) {
            let (new_timer_counter, overflow) =
                memory.data[TIMER_COUNTER_ADDRESS].overflowing_add(1);
            memory.data[TIMER_COUNTER_ADDRESS] = new_timer_counter;
            if overflow {
                memory.data[TIMER_COUNTER_ADDRESS] = memory.data[TIMER_MODULO_ADDRESS];
                memory.write_interrupt_flag(Interrupt::Timer, true);
            }
            self.tima = memory.data[TIMER_COUNTER_ADDRESS];
        }
    }
}
