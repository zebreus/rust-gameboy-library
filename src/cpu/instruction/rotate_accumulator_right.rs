use super::Instruction;
use crate::{
    cpu::{Cpu, Flag, Register},
    memory::MemoryDevice,
};

/// [Rotate](https://en.wikipedia.org/wiki/Bitwise_operation#Rotate) the [accumulator](Register::A) right by one bit.
///
/// This is like [RotateRightRegister](super::RotateRightRegister) with [Register::A], but it always resets [Flag::Zero]
///
/// Transformation when the bitorder is `0b76543210`
///
/// | Bits before  | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
/// |--------------|---|---|---|---|---|---|---|---|
/// | Bits after   | 0 | 7 | 6 | 5 | 4 | 3 | 2 | 1 |
///
/// Flags:
///
/// | [Zero](Flag::Zero) | [Subtract](Flag::Subtract) | [HalfCarry](Flag::HalfCarry) | [Carry](Flag::Carry)          |
/// |--------------------|----------------------------|------------------------------|-------------------------------|
/// | false              | false                      | false                        | set to the value of old bit 0 |
#[doc(alias = "RRCA")]
pub struct RotateAccumulatorRight {}

impl Instruction for RotateAccumulatorRight {
    fn execute<T: MemoryDevice>(
        &self,
        cpu: &mut crate::cpu::CpuState,
        memory: &mut T,
    ) -> super::InstructionEnum {
        let operand = cpu.read_register(Register::A);

        let result = operand.rotate_right(1);
        let carry_flag = result >= 0b10000000;
        cpu.write_flag(Flag::Zero, false);
        cpu.write_flag(Flag::Subtract, false);
        cpu.write_flag(Flag::HalfCarry, false);
        cpu.write_flag(Flag::Carry, carry_flag);

        cpu.write_register(Register::A, result);
        return cpu.load_instruction(memory);
    }
    fn encode(&self) -> Vec<u8> {
        Vec::from([0b00001111])
    }
}
