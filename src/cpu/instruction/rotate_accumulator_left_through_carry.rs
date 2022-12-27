use super::Instruction;
use crate::{
    cpu::{Cpu, Flag, Register},
    memory::MemoryDevice,
};

/// [Rotate through carry](https://en.wikipedia.org/wiki/Bitwise_operation#Rotate_through_carry) the [accumulator](Register::A) left by one bit.
///
/// This is like [RotateLeftThroughCarryRegister](super::RotateLeftThroughCarryRegister) with [Register::A], but it always resets [Flag::Zero]
///
/// Transformation when the bitorder is `0b76543210` and `C` is the value of the [Flag::Carry]
///
/// | Bits before  | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
/// |--------------|---|---|---|---|---|---|---|---|
/// | Bits after   | 6 | 5 | 4 | 3 | 2 | 1 | 0 | C |
///
/// Flags:
///
/// | [Zero](Flag::Zero) | [Subtract](Flag::Subtract) | [HalfCarry](Flag::HalfCarry) | [Carry](Flag::Carry)          |
/// |--------------------|----------------------------|------------------------------|-------------------------------|
/// | false              | false                      | false                        | set to the value of old bit 7 |
#[doc(alias = "RLA")]
#[derive(Debug)]
pub struct RotateAccumulatorLeftThroughCarry {}

impl Instruction for RotateAccumulatorLeftThroughCarry {
    fn execute<T: MemoryDevice>(
        &self,
        cpu: &mut crate::cpu::CpuState,
        memory: &mut T,
    ) -> super::InstructionEnum {
        let operand = cpu.read_register(Register::A);

        let result = (operand << 1) + (if cpu.read_flag(Flag::Carry) { 1 } else { 0 });
        let carry_flag = operand >= 0b10000000;
        cpu.write_flag(Flag::Zero, false);
        cpu.write_flag(Flag::Subtract, false);
        cpu.write_flag(Flag::HalfCarry, false);
        cpu.write_flag(Flag::Carry, carry_flag);

        cpu.write_register(Register::A, result);
        return cpu.load_instruction(memory);
    }
    fn encode(&self) -> Vec<u8> {
        Vec::from([0b00010111])
    }
}
