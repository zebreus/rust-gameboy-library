use super::CpuState;
use crate::memory_device::MemoryDevice;
use enum_dispatch::enum_dispatch;

mod add_register;
mod call;
mod call_conditional;
mod decode;
mod jump_by_immediate_offset;
mod jump_by_immediate_offset_conditional;
mod jump_to_hl;
mod jump_to_immediate_address;
mod jump_to_immediate_address_conditional;
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
mod load_hl_to_sp;
mod load_immediate_to_double_register;
mod load_immediate_to_hl;
mod load_immediate_to_register;
mod load_register_to_hl;
mod load_sp_to_immediate_address;
mod pop_double_register;
mod push_double_register;
mod return_conditional;
mod return_instruction;

/// Different phases for instructions
pub mod phases;

#[doc(inline)]
pub use add_register::AddRegister;
#[doc(inline)]
pub use call::Call;
#[doc(inline)]
pub use call_conditional::CallConditional;
#[doc(inline)]
pub use decode::decode;
#[doc(inline)]
pub use jump_by_immediate_offset::JumpByImmediateOffset;
#[doc(inline)]
pub use jump_by_immediate_offset_conditional::JumpByImmediateOffsetConditional;
#[doc(inline)]
pub use jump_to_hl::JumpToHl;
#[doc(inline)]
pub use jump_to_immediate_address::JumpToImmediateAddress;
#[doc(inline)]
pub use jump_to_immediate_address_conditional::JumpToImmediateAddressConditional;
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
pub use load_hl_to_sp::LoadHlToSp;
#[doc(inline)]
pub use load_immediate_to_double_register::LoadImmediateToDoubleRegister;
#[doc(inline)]
pub use load_immediate_to_hl::LoadImmediateToHl;
#[doc(inline)]
pub use load_immediate_to_register::LoadImmediateToRegister;
#[doc(inline)]
pub use load_register_to_hl::LoadRegisterToHl;
#[doc(inline)]
pub use load_sp_to_immediate_address::LoadSpToImmediateAddress;
#[doc(inline)]
pub use pop_double_register::PopDoubleRegister;
#[doc(inline)]
pub use push_double_register::PushDoubleRegister;
#[doc(inline)]
pub use return_conditional::ReturnConditional;
#[doc(inline)]
pub use return_instruction::Return;

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
    /// See [LoadImmediateToDoubleRegister]
    LoadImmediateToDoubleRegister,
    /// See [LoadSpToImmediateAddress]
    LoadSpToImmediateAddress,
    /// See [LoadHlToSp]
    LoadHlToSp,
    /// See [PushDoubleRegister]
    PushDoubleRegister,
    /// See [PopDoubleRegister]
    PopDoubleRegister,
    /// See [JumpToImmediateAddress]
    JumpToImmediateAddress,
    /// See [JumpToImmediateAddressConditional]
    JumpToImmediateAddressConditional,
    /// See [JumpToHl]
    JumpToHl,
    /// See [JumpByImmediateOffset]
    JumpByImmediateOffset,
    /// See [JumpByImmediateOffsetConditional]
    JumpByImmediateOffsetConditional,
    /// See [Call]
    Call,
    /// See [CallConditional]
    CallConditional,
    /// See [AddRegister]
    AddRegister,
    /// See [Return]
    Return,
    /// See [ReturnConditional]
    ReturnConditional,
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
    /// assert_eq!(encoded, Vec::from([0b01111001u8]));
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
        cpu.write_register(Register::A, 0);
        cpu.write_register(Register::B, 0);
        cpu.write_register(Register::C, 0);

        let mut memory = DebugMemory::new_with_init(&[
            0b00100110,
            0,
            0b00101110,
            9,
            0b01111110u8,
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
