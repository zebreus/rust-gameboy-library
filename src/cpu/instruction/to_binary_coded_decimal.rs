use super::Instruction;
use crate::{
    cpu::{Cpu, Flag, Register},
    memory::MemoryDevice,
};

/// Convert the value in the [accumulator][Register::A] to a binary coded decimal
///
/// See <https://ehaskins.com/2018-01-30%20Z80%20DAA/> for an explanation what this instruction does.
///
/// Our implementation is copied from [GoGB](https://github.com/guigzzz/GoGB/blob/master/backend/cpu_arithmetic.go#L349)
///
/// | [Zero](Flag::Zero)  | [Subtract](Flag::Subtract) | [HalfCarry](Flag::HalfCarry) | [Carry](Flag::Carry)       |
/// |---------------------|----------------------------|------------------------------|----------------------------|
/// | true if result is 0 | unchanged                  | false                        | true if a carry occurred   |
#[doc(alias = "DAA")]
#[derive(Debug)]
pub struct ToBinaryCodedDecimal {}

impl Instruction for ToBinaryCodedDecimal {
    fn execute<T: MemoryDevice>(
        &self,
        cpu: &mut crate::cpu::CpuState,
        memory: &mut T,
    ) -> super::InstructionEnum {
        // DAA algorithm from https://github.com/guigzzz/GoGB/blob/master/backend/cpu_arithmetic.go#L349
        let mut value = cpu.read_register(Register::A) as u16;

        if !cpu.read_flag(Flag::Subtract) {
            if cpu.read_flag(Flag::HalfCarry) || ((value & 0xF) > 0x9) {
                value = value.wrapping_add(0x6);
            }
            if cpu.read_flag(Flag::Carry) || (value > 0x9F) {
                value = value.wrapping_add(0x60);

                cpu.write_flag(Flag::Carry, true);
            }
        } else {
            if cpu.read_flag(Flag::HalfCarry) {
                value = value.wrapping_sub(0x6);
            }

            if cpu.read_flag(Flag::Carry) {
                value = value.wrapping_sub(0x60);
            }
        }
        cpu.write_register(Register::A, value.to_le_bytes()[0]);

        cpu.write_flag(Flag::Zero, value.to_le_bytes()[0] == 0);
        cpu.write_flag(Flag::HalfCarry, false);

        return cpu.load_instruction(memory);
    }
    fn encode(&self) -> Vec<u8> {
        Vec::from([0b00100111])
    }
}

#[cfg(test)]
mod tests {
    use super::ToBinaryCodedDecimal;
    use crate::cpu::instruction::{AddRegister, Instruction, SubtractRegister};
    use crate::cpu::{Cpu, CpuState, Register};
    use crate::memory::MemoryController;

    fn add_and_use_instruction(a: u8, b: u8) -> u8 {
        let mut cpu = CpuState::new();
        let mut memory = MemoryController::new();

        cpu.write_register(Register::A, a);
        cpu.write_register(Register::B, b);

        let instruction = AddRegister {
            operand: Register::B,
        };

        instruction.execute(&mut cpu, &mut memory);

        let instruction = ToBinaryCodedDecimal {};

        instruction.execute(&mut cpu, &mut memory);

        return cpu.read_register(Register::A);
    }

    fn subtract_and_use_instruction(a: u8, b: u8) -> u8 {
        let mut cpu = CpuState::new();
        let mut memory = MemoryController::new();

        cpu.write_register(Register::A, a);
        cpu.write_register(Register::B, b);

        let instruction = SubtractRegister {
            operand: Register::B,
        };

        instruction.execute(&mut cpu, &mut memory);

        let instruction = ToBinaryCodedDecimal {};

        instruction.execute(&mut cpu, &mut memory);

        return cpu.read_register(Register::A);
    }

    #[test]
    fn works_as_expected_after_addition() {
        assert_eq!(add_and_use_instruction(5, 5), 0x10);
        assert_eq!(add_and_use_instruction(0, 0), 0);
        assert_eq!(add_and_use_instruction(1, 0), 1);
        assert_eq!(add_and_use_instruction(0, 1), 1);
        assert_eq!(add_and_use_instruction(1, 1), 2);
        assert_eq!(add_and_use_instruction(5, 4), 9);
        assert_eq!(add_and_use_instruction(0x24, 0x43), 0x67);
        assert_eq!(add_and_use_instruction(0x25, 0x45), 0x70);
        assert_eq!(add_and_use_instruction(0x29, 0x49), 0x78);
        assert_eq!(add_and_use_instruction(0x99, 0x99), 0x98);
        assert_eq!(add_and_use_instruction(0x01, 0x98), 0x99);
        assert_eq!(add_and_use_instruction(0x01, 0x99), 0x00);

        assert_eq!(add_and_use_instruction(0x0A, 0x00), 0x10);
        assert_eq!(add_and_use_instruction(0x0B, 0x00), 0x11);
        assert_eq!(add_and_use_instruction(0x0C, 0x00), 0x12);
        assert_eq!(add_and_use_instruction(0x0D, 0x00), 0x13);
        assert_eq!(add_and_use_instruction(0x0E, 0x00), 0x14);
        assert_eq!(add_and_use_instruction(0x0F, 0x00), 0x15);

        assert_eq!(add_and_use_instruction(0x0A, 0x02), 0x12);
        assert_eq!(add_and_use_instruction(0x0B, 0x02), 0x13);
        assert_eq!(add_and_use_instruction(0x0C, 0x02), 0x14);
        assert_eq!(add_and_use_instruction(0x0D, 0x02), 0x15);
        assert_eq!(add_and_use_instruction(0x0E, 0x02), 0x16);
        assert_eq!(add_and_use_instruction(0x0F, 0x02), 0x17);
    }

    #[test]
    fn works_as_expected_after_subtraction() {
        assert_eq!(subtract_and_use_instruction(0x0A, 0x00), 0x0A);

        assert_eq!(subtract_and_use_instruction(0x10, 0x01), 0x09);
        assert_eq!(subtract_and_use_instruction(0, 0), 0);
        assert_eq!(subtract_and_use_instruction(1, 0), 1);
        assert_eq!(subtract_and_use_instruction(1, 1), 0);
        assert_eq!(subtract_and_use_instruction(5, 4), 0x01);
        assert_eq!(subtract_and_use_instruction(5, 5), 0x00);
        assert_eq!(subtract_and_use_instruction(0x43, 0x24), 0x19);
        assert_eq!(subtract_and_use_instruction(0x13, 0x24), 0x89);
        assert_eq!(subtract_and_use_instruction(0x45, 0x25), 0x20);
        assert_eq!(subtract_and_use_instruction(0x49, 0x29), 0x20);
        assert_eq!(subtract_and_use_instruction(0x99, 0x99), 0x00);
        assert_eq!(subtract_and_use_instruction(0x99, 0x01), 0x98);

        assert_eq!(subtract_and_use_instruction(0x0A, 0x07), 0x03);
        assert_eq!(subtract_and_use_instruction(0x0B, 0x07), 0x04);
        assert_eq!(subtract_and_use_instruction(0x0C, 0x07), 0x05);
        assert_eq!(subtract_and_use_instruction(0x0D, 0x07), 0x06);
        assert_eq!(subtract_and_use_instruction(0x0E, 0x07), 0x07);
        assert_eq!(subtract_and_use_instruction(0x0F, 0x07), 0x08);
    }
}
