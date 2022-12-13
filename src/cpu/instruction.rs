use super::CpuState;
use crate::memory_device::MemoryDevice;
use enum_dispatch::enum_dispatch;

mod decode;
mod load_accumulator_to_double_register;
mod load_accumulator_to_hl_and_decrement;
mod load_accumulator_to_hl_and_increment;
mod load_accumulator_to_immediate_address;
mod load_accumulator_to_immediate_offset;
mod load_accumulator_to_register_c_offset;
mod load_from_double_register_to_accumulator;
mod load_from_hl_to_register;
mod load_from_immediate_address_to_accumulator;
mod load_from_immediate_offset_to_accumulator;
mod load_from_register_c_offset_to_accumulator;
mod load_from_register_to_register;
mod load_hl_to_accumulator_and_decrement;
mod load_hl_to_accumulator_and_increment;
mod load_immediate_to_hl;
mod load_immediate_to_register;
mod load_register_to_hl;

/// Different phases for instructions
pub mod phases;

#[doc(inline)]
pub use decode::decode;
#[doc(inline)]
pub use load_accumulator_to_double_register::LoadAccumulatorToDoubleRegister;
#[doc(inline)]
pub use load_accumulator_to_hl_and_decrement::LoadAccumulatorToHlAndDecrement;
#[doc(inline)]
pub use load_accumulator_to_hl_and_increment::LoadAccumulatorToHlAndIncrement;
#[doc(inline)]
pub use load_accumulator_to_immediate_address::LoadAccumulatorToImmediateAddress;
#[doc(inline)]
pub use load_accumulator_to_immediate_offset::LoadAccumulatorToImmediateOffset;
#[doc(inline)]
pub use load_accumulator_to_register_c_offset::LoadAccumulatorToRegisterCOffset;
#[doc(inline)]
pub use load_from_double_register_to_accumulator::LoadFromDoubleRegisterToAccumulator;
#[doc(inline)]
pub use load_from_hl_to_register::LoadFromHlToRegister;
#[doc(inline)]
pub use load_from_immediate_address_to_accumulator::LoadFromImmediateAddressToAccumulator;
#[doc(inline)]
pub use load_from_immediate_offset_to_accumulator::LoadFromImmediateOffsetToAccumulator;
#[doc(inline)]
pub use load_from_register_c_offset_to_accumulator::LoadFromRegisterCOffsetToAccumulator;
#[doc(inline)]
pub use load_from_register_to_register::LoadFromRegisterToRegister;
#[doc(inline)]
pub use load_hl_to_accumulator_and_decrement::LoadHlToAccumulatorAndDecrement;
#[doc(inline)]
pub use load_hl_to_accumulator_and_increment::LoadHlToAccumulatorAndIncrement;
#[doc(inline)]
pub use load_immediate_to_hl::LoadImmediateToHl;
#[doc(inline)]
pub use load_immediate_to_register::LoadImmediateToRegister;
#[doc(inline)]
pub use load_register_to_hl::LoadRegisterToHl;

/// Contains a variant for every [Instruction]
#[enum_dispatch]
pub enum InstructionEnum {
    /// See [LoadFromHlToRegister]
    LoadFromHlToRegister,
    /// See [LoadFromHlToRegister]
    LoadRegisterToHl,
    /// See [LoadFromRegisterToRegister]
    LoadFromRegisterToRegister,
    /// See [LoadImmediateToRegister]
    LoadImmediateToRegister,
    /// See [LoadAccumulatorToImmediateOffset]
    LoadAccumulatorToImmediateOffset,
    /// See [LoadFromImmediateOffsetToAccumulator]
    LoadFromImmediateOffsetToAccumulator,
    /// See [LoadHlToAccumulatorAndDecrement]
    LoadHlToAccumulatorAndDecrement,
    /// See [LoadHlToAccumulatorAndIncrement]
    LoadHlToAccumulatorAndIncrement,
    /// See [LoadAccumulatorToHlAndDecrement]
    LoadAccumulatorToHlAndDecrement,
    /// See [LoadAccumulatorToHlAndIncrement]
    LoadAccumulatorToHlAndIncrement,
    /// See [LoadAccumulatorToRegisterCOffset]
    LoadAccumulatorToRegisterCOffset,
    /// See [LoadFromRegisterCOffsetToAccumulator]
    LoadFromRegisterCOffsetToAccumulator,
    /// See [LoadAccumulatorToImmediateAddress]
    LoadAccumulatorToImmediateAddress,
    /// See [LoadFromImmediateAddressToAccumulator]
    LoadFromImmediateAddressToAccumulator,
    /// See [LoadImmediateToHl]
    LoadImmediateToHl,
    /// See [LoadAccumulatorToDoubleRegister]
    LoadAccumulatorToDoubleRegister,
    /// See [LoadFromDoubleRegisterToAccumulator]
    LoadFromDoubleRegisterToAccumulator,
}

/// This is the trait for executable CPU instructions.
///
/// Each instruction is a struct that contains information on the current state of the instruction.
///
/// Instructions that take longer then one cycle have a `phases` field indicating the current cycle of the instruction.
///
/// Every instruction can be executed using the [Instruction::execute] function.
///
/// To be able to distinguish instructions at compile time all instructions have a corresponding variant in [InstructionEnum]
#[enum_dispatch(InstructionEnum)]
pub trait Instruction {
    /// Execute the instruction on the cpu and memory. Returns the next instruction.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rust_gameboy_library::cpu::{CpuState, Cpu, Register};
    /// # use rust_gameboy_library::cpu::instruction::LoadFromRegisterToRegister;
    /// # use rust_gameboy_library::cpu::instruction::Instruction;
    /// # use rust_gameboy_library::debug_memory::DebugMemory;
    /// #
    /// let mut cpu = CpuState::new();
    /// let mut memory = DebugMemory::new();
    /// cpu.write_register(Register::A, 100);
    ///
    /// let instruction = LoadFromRegisterToRegister {
    ///     source: Register::A,
    ///     destination: Register::C,
    /// };
    ///
    /// instruction.execute(&mut cpu, &mut memory);
    /// assert_eq!(cpu.read_register(Register::C), 100);
    /// ```
    fn execute<T: MemoryDevice>(&self, cpu: &mut CpuState, memory: &mut T) -> InstructionEnum;
    /// Encode a instruction back into it's binary representation
    ///
    /// If the instruction is longer than 1 byte and has not yet read all relevant bytes, only the known bytes are returned.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rust_gameboy_library::cpu::Register;
    /// # use rust_gameboy_library::cpu::instruction::LoadFromRegisterToRegister;
    /// # use rust_gameboy_library::cpu::instruction::Instruction;
    /// #
    /// let instruction = LoadFromRegisterToRegister {
    ///     source: Register::A,
    ///     destination: Register::C,
    /// };
    ///
    /// let encoded: Vec<u8> = instruction.encode();
    /// assert_eq!(encoded, Vec::from([0b01000010u8]));
    /// ```
    fn encode(&self) -> Vec<u8>;
}

#[cfg(test)]
mod tests {
    use crate::cpu::instruction::Instruction;
    use crate::cpu::{Cpu, CpuState, Register};
    use crate::debug_memory::DebugMemory;

    #[test]
    fn load_from_hl_works() {
        // Write 42 to A and then copy A to C
        let mut cpu = CpuState::new();

        let mut memory = DebugMemory::new_with_init(&[
            0b00110110,
            0,
            0b00111110,
            9,
            0b01000110u8,
            0,
            0,
            0,
            0,
            42,
        ]);

        let instruction = cpu.load_instruction(&memory);

        let instruction = instruction.execute(&mut cpu, &mut memory);
        let instruction = instruction.execute(&mut cpu, &mut memory);
        let instruction = instruction.execute(&mut cpu, &mut memory);
        let instruction = instruction.execute(&mut cpu, &mut memory);
        let instruction = instruction.execute(&mut cpu, &mut memory);
        instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cpu.read_register(Register::A), 42);
        assert_eq!(cpu.read_register(Register::B), 0);
        assert_eq!(cpu.read_register(Register::C), 0);
    }
}
