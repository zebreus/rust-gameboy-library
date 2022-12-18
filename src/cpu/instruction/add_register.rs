use super::{generate_instruction::generate_instruction, Instruction};
use crate::{
    cpu::{Cpu, Flag, Register},
    memory_device::MemoryDevice,
};

generate_instruction!(
    /// Add a value of the operand register to the accumulator.
    AddRegister,
    0b10000000,
    cpu,
    memory,
    operand,
    {
        let accumulator_value = cpu.read_register(Register::A);
        let result = accumulator_value.wrapping_add(operand);

        // set flags
        let zero_flag = result == 0;
        let subtract_flag = false;
        let half_carry_flag = (accumulator_value ^ operand ^ result) & 0b00010000 == 0b00010000;
        let carry_flag = result < accumulator_value;

        cpu.write_flag(Flag::Zero, zero_flag);
        cpu.write_flag(Flag::Subtract, subtract_flag);
        cpu.write_flag(Flag::HalfCarry, half_carry_flag);
        cpu.write_flag(Flag::Carry, carry_flag);

        cpu.write_register(Register::A, result);
    }, $
);

#[cfg(test)]
mod tests {

    use super::AddRegister;
    use crate::cpu::instruction::Instruction;
    use crate::cpu::{Cpu, CpuState, Flag, Register};
    use crate::debug_memory::DebugMemory;

    #[test]
    fn load_instruction_works() {
        assert_result!((A: 100, B: 100,), (A: 200,));
    }

    #[test]
    fn zero_flag_works() {
        assert_result!((A: 100, B: 100,), (FLAG_UNSET: Flag::Zero,));
    }

    #[test]
    fn half_carry_flag_works() {
        assert_result!((A: 0b00001111, B: 0b00100000,), (FLAG_UNSET: Flag::HalfCarry,));
        assert_result!((A: 0b00001111, B: 0b00000001,), (FLAG: Flag::HalfCarry,));
        assert_result!((A: 0b00001111, B: 0b00010001,), (FLAG: Flag::HalfCarry,));
        assert_result!((A: 0b00001111, B: 0b00010000,), (FLAG_UNSET: Flag::HalfCarry,));

        assert_result!((A: 0b00010001, B: 0b00001111,), (FLAG: Flag::HalfCarry,));
        assert_result!((A: 0b00010001, B: 0b00000010,), (FLAG_UNSET: Flag::HalfCarry,));
        assert_result!((A: 0b00010001, B: 0b00010000,), (FLAG_UNSET: Flag::HalfCarry,));
        assert_result!((A: 0b00010001, B: 0b00011111,), (FLAG: Flag::HalfCarry,));
    }

    #[test]
    fn carry_flag_works() {
        assert_result!((A: 0b11111111, B: 0b00000001,), (FLAG: Flag::Carry,));

        assert_result!((A: 0b01111111, B: 0b00000001,), (FLAG_UNSET: Flag::Carry,));
    }
}
