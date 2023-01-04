use num_enum::{IntoPrimitive, TryFromPrimitive};

use crate::cpu::{interrupt_controller::InterruptController, Interrupt};

use self::serial_connection::SerialConnection;

use super::{
    memory_addresses::{SERIAL_CONTROL_ADDRESS, SERIAL_DATA_ADDRESS},
    video::display_connection::DisplayConnection,
    Memory,
};

/// Contains traits for serial connections and some implementations
pub mod serial_connection;

#[derive(TryFromPrimitive, Debug, IntoPrimitive, PartialEq)]
#[repr(u8)]
enum ClockType {
    External = 0,
    Internal = 1,
}

#[derive(TryFromPrimitive, Debug, IntoPrimitive, PartialEq)]
#[repr(u8)]
enum TransactionState {
    /// Neither sending nor receiving.
    Nothing = 0,
    InProgress = 1,
}

/// Represents a serial connection
pub struct Serial<T: SerialConnection> {
    connection: Option<T>,
    transferred_bits: usize,
    clock_source: ClockType,
    transaction_state: TransactionState,
    cycles_until_next_bit: u32,
}

/// The gameboy CPU runs at 1048576 Hz, the transfer speed is 8192 Hz. So 1 bit gets transferred per 128 cycles.
const CYCLES_PER_BIT: u32 = 128;

impl<T: SerialConnection> Serial<T> {
    /// Create a new serial connection that logs the output to the console.
    pub fn new(connection: Option<T>) -> Self {
        Self {
            connection: connection,
            transferred_bits: 0,
            clock_source: ClockType::External,
            transaction_state: TransactionState::InProgress,
            cycles_until_next_bit: CYCLES_PER_BIT,
        }
    }
}

impl<T: SerialConnection, D: DisplayConnection> Memory<T, D> {
    /// Process writes to the memory
    pub fn write_serial(&mut self, address: u16, value: u8) -> Option<()> {
        let serial = &mut self.serial;
        match address as usize {
            SERIAL_DATA_ADDRESS => None,
            SERIAL_CONTROL_ADDRESS => {
                let transfer_in_progress_bit = (value & 0b10000000) >> 7;
                let clock_source_bit = value & 0b00000001;
                serial.clock_source = clock_source_bit
                    .try_into()
                    .expect("Clock source bit should always be in range");
                serial.transaction_state = transfer_in_progress_bit
                    .try_into()
                    .expect("Transfer in progress bit should always be in range");
                self.memory[SERIAL_CONTROL_ADDRESS] = value;
                Some(())
            }
            _ => None,
        }
    }
    /// Should be called on every cycle
    pub fn cycle_serial(&mut self) {
        let serial = &mut self.serial;
        if !(serial.clock_source == ClockType::Internal
            && serial.transaction_state == TransactionState::InProgress)
        {
            return;
        }

        serial.cycles_until_next_bit -= 1;
        if serial.cycles_until_next_bit != 0 {
            return;
        }

        serial.cycles_until_next_bit = CYCLES_PER_BIT;

        let send_bit = (self.memory[SERIAL_DATA_ADDRESS] & 0b10000000) == 0b10000000;
        let received_bit = self
            .serial
            .connection
            .as_mut()
            .map(|connection| connection.exchange_bit(send_bit))
            .unwrap_or(true);
        self.memory[SERIAL_DATA_ADDRESS] =
            (self.memory[SERIAL_DATA_ADDRESS] << 1) | (if received_bit { 1 } else { 0 });

        self.serial.transferred_bits += 1;
        if self.serial.transferred_bits < 8 {
            return;
        }

        self.memory[SERIAL_CONTROL_ADDRESS] = self.memory[SERIAL_CONTROL_ADDRESS] & 0b01111111;
        self.serial.transaction_state = TransactionState::Nothing;
        self.write_interrupt_flag(Interrupt::Serial, true);
        self.serial.transferred_bits = 0;
    }
}
