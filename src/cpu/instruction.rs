use super::CpuState;
use crate::memory::MemoryDevice;
use enum_dispatch::enum_dispatch;

mod decode;
mod decode_cb;
/// Really hacky macro for generating arithmetic instructions
pub mod generate_instruction;
/// Different phases for instructions
pub mod phases;

pub use decode::decode;
pub use decode_cb::decode_cb;

macro_rules! generate_instruction_enum {
    ($enum_name:ident, $( ( $module_path:ident, $( $instruction:ident ),* ) ),+) => {
        $(
            mod $module_path;
        )+

        $(
            #[doc(inline)]
            pub use $module_path::{
                $(
                    $instruction,
                )*
            };
        )+


        /// Contains a variant for every [Instruction]
        #[enum_dispatch]
        pub enum $enum_name {
            $(
                $(
                    #[doc = stringify!(See [$instruction])]
                    $instruction ,
                )*
            )+
        }
    };
}

generate_instruction_enum!(
    InstructionEnum,
    (add, AddRegister, AddFromHl, AddImmediate),
    (
        add_with_carry,
        AddWithCarryRegister,
        AddWithCarryFromHl,
        AddWithCarryImmediate
    ),
    (
        subtract,
        SubtractRegister,
        SubtractFromHl,
        SubtractImmediate
    ),
    (
        subtract_with_carry,
        SubtractWithCarryRegister,
        SubtractWithCarryFromHl,
        SubtractWithCarryImmediate
    ),
    (
        bitwise_and,
        BitwiseAndRegister,
        BitwiseAndFromHl,
        BitwiseAndImmediate
    ),
    (increment, IncrementRegister, IncrementAtHl),
    (decrement, DecrementRegister, DecrementAtHl),
    (
        bitwise_or,
        BitwiseOrRegister,
        BitwiseOrFromHl,
        BitwiseOrImmediate
    ),
    (
        bitwise_exclusive_or,
        BitwiseExclusiveOrRegister,
        BitwiseExclusiveOrFromHl,
        BitwiseExclusiveOrImmediate
    ),
    (rotate_left, RotateLeftRegister, RotateLeftAtHl),
    (rotate_right, RotateRightRegister, RotateRightAtHl),
    (
        rotate_left_through_carry,
        RotateLeftThroughCarryRegister,
        RotateLeftThroughCarryAtHl
    ),
    (
        rotate_right_through_carry,
        RotateRightThroughCarryRegister,
        RotateRightThroughCarryAtHl
    ),
    (shift_left, ShiftLeftRegister, ShiftLeftAtHl),
    (shift_right, ShiftRightRegister, ShiftRightAtHl),
    (swap_nibbles, SwapNibblesRegister, SwapNibblesAtHl),
    (check_bit, CheckBitRegister, CheckBitAtHl),
    (set_bit, SetBitRegister, SetBitAtHl),
    (reset_bit, ResetBitRegister, ResetBitAtHl),
    (
        shift_right_logical,
        ShiftRightLogicalRegister,
        ShiftRightLogicalAtHl
    ),
    (compare, CompareRegister, CompareFromHl, CompareImmediate),
    (call, Call),
    (call_conditional, CallConditional),
    (complement, Complement),
    (disable_interrupts, DisableInterrupts),
    (enable_interrupts, EnableInterrupts),
    (halt, Halt),
    (interrupt_service_routine, InterruptServiceRoutine),
    (invert_carry, InvertCarry),
    (jump_by_immediate_offset, JumpByImmediateOffset),
    (
        jump_by_immediate_offset_conditional,
        JumpByImmediateOffsetConditional
    ),
    (jump_to_hl, JumpToHl),
    (jump_to_immediate_address, JumpToImmediateAddress),
    (
        jump_to_immediate_address_conditional,
        JumpToImmediateAddressConditional
    ),
    (
        load_accumulator_to_double_register,
        LoadAccumulatorToDoubleRegister
    ),
    (
        load_accumulator_to_hl_and_decrement,
        LoadAccumulatorToHlAndDecrement
    ),
    (
        load_accumulator_to_hl_and_increment,
        LoadAccumulatorToHlAndIncrement
    ),
    (
        load_accumulator_to_immediate_address,
        LoadAccumulatorToImmediateAddress
    ),
    (
        load_accumulator_to_immediate_offset,
        LoadAccumulatorToImmediateOffset
    ),
    (
        load_accumulator_to_register_c_offset,
        LoadAccumulatorToRegisterCOffset
    ),
    (
        load_from_double_register_to_accumulator,
        LoadFromDoubleRegisterToAccumulator
    ),
    (load_from_hl_to_register, LoadFromHlToRegister),
    (
        load_from_immediate_address_to_accumulator,
        LoadFromImmediateAddressToAccumulator
    ),
    (
        load_from_immediate_offset_to_accumulator,
        LoadFromImmediateOffsetToAccumulator
    ),
    (
        load_from_register_c_offset_to_accumulator,
        LoadFromRegisterCOffsetToAccumulator
    ),
    (load_from_register_to_register, LoadFromRegisterToRegister),
    (
        load_hl_to_accumulator_and_decrement,
        LoadHlToAccumulatorAndDecrement
    ),
    (
        load_hl_to_accumulator_and_increment,
        LoadHlToAccumulatorAndIncrement
    ),
    (load_hl_to_sp, LoadHlToSp),
    (
        load_immediate_to_double_register,
        LoadImmediateToDoubleRegister
    ),
    (load_immediate_to_hl, LoadImmediateToHl),
    (load_immediate_to_register, LoadImmediateToRegister),
    (load_register_to_hl, LoadRegisterToHl),
    (load_sp_to_immediate_address, LoadSpToImmediateAddress),
    (nop, Nop),
    (pop_double_register, PopDoubleRegister),
    (push_double_register, PushDoubleRegister),
    (return_conditional, ReturnConditional),
    (return_instruction, Return),
    (return_from_interrupt, ReturnFromInterrupt),
    (set_carry, SetCarry),
    (stop, Stop),
    (to_binary_coded_decimal, ToBinaryCodedDecimal),
    (add_immediate_offset_to_sp, AddImmediateOffsetToSp),
    (
        load_sp_plus_immediate_offset_to_hl,
        LoadSpPlusImmediateOffsetToHl
    ),
    (add_double_register_to_hl, AddDoubleRegisterToHl),
    (increment_double_register, IncrementDoubleRegister),
    (decrement_double_register, DecrementDoubleRegister),
    (halt_and_catch_fire, HaltAndCatchFire),
    (restart, Restart),
    (rotate_accumulator_left, RotateAccumulatorLeft),
    (
        rotate_accumulator_left_through_carry,
        RotateAccumulatorLeftThroughCarry
    ),
    (rotate_accumulator_right, RotateAccumulatorRight),
    (
        rotate_accumulator_right_through_carry,
        RotateAccumulatorRightThroughCarry
    ),
    (prefix_cb, PrefixCb)
);

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
pub trait Instruction: Sized {
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
