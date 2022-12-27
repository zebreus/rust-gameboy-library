use super::Instruction;
use crate::{
    cpu::{Cpu, Flag, Register},
    memory::MemoryDevice,
};

/// Convert the value in the [accumulator][Register::A] to a binary coded decimal
///
// TODO: The behaviour is currently totally wrong. One would call this instruction after a `0x60 + 0x60 = 0xc0` and expect the result to be `0x120`,  because `60 + 60 = 120`
///
/// | [Zero](Flag::Zero)  | [Subtract](Flag::Subtract) | [HalfCarry](Flag::HalfCarry) | [Carry](Flag::Carry)       |
/// |---------------------|----------------------------|------------------------------|----------------------------|
/// | true if result is 0 | unchanged                  | false                        | true if a carry occurred   |
#[doc(alias = "DAA")]
pub struct ToBinaryCodedDecimal {}

impl Instruction for ToBinaryCodedDecimal {
    fn execute<T: MemoryDevice>(
        &self,
        cpu: &mut crate::cpu::CpuState,
        memory: &mut T,
    ) -> super::InstructionEnum {
        let original = cpu.read_register(Register::A);
        let second_digit = original % 10;
        let first_digit = (original / 10) % 10;
        let binary_coded_decimal = (first_digit << 4) | second_digit;
        cpu.write_register(Register::A, binary_coded_decimal);

        cpu.write_flag(Flag::Zero, binary_coded_decimal == 0);
        cpu.write_flag(Flag::HalfCarry, false);
        cpu.write_flag(Flag::Carry, false);

        return cpu.load_instruction(memory);
    }
    fn encode(&self) -> Vec<u8> {
        // TODO: This is the wrong opcode
        Vec::from([0b00100111])
    }
}

#[cfg(test)]
mod tests {
    use super::ToBinaryCodedDecimal;
    use crate::cpu::instruction::Instruction;
    use crate::cpu::{Cpu, CpuState, Register};
    use crate::debug_memory::DebugMemory;

    #[test]
    fn to_binary_coded_decimal_works() {
        let mut cpu = CpuState::new();
        let mut memory = DebugMemory::new_with_init(&[1, 1, 1, 1]);

        cpu.write_register(Register::A, 56);

        let instruction = ToBinaryCodedDecimal {};

        instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cpu.read_register(Register::A), 0x56);
    }

    #[test]
    fn to_binary_coded_decimal_works_with_big_number() {
        let mut cpu = CpuState::new();
        let mut memory = DebugMemory::new_with_init(&[1, 1, 1, 1]);

        cpu.write_register(Register::A, 223);

        let instruction = ToBinaryCodedDecimal {};

        instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cpu.read_register(Register::A), 0x23);
    }
}
