use super::generate_instruction::{generate_instruction, prepare_generate_instruction};

prepare_generate_instruction!($, AddWithCarryRegister);
generate_instruction!(
    ///
    /// If [Flag::Carry] is false this just adds the operand. If [Flag::Carry] is true this adds the operand plus 1.
    ///
    /// | [Zero](Flag::Zero)  | [Subtract](Flag::Subtract) | [HalfCarry](Flag::HalfCarry)        | [Carry](Flag::Carry)       |
    /// |---------------------|----------------------------|-------------------------------------|----------------------------|
    /// | true if result is 0 | false                      | true if the lower nibble overflowed | true if a overflow occured |
    (
        /// Add the operand register and the previous carry to the [accumulator](Register::A).
        AddWithCarryRegister,
        /// Add the previous carry and the value at the memory Address in [DoubleRegister::HL] to the [accumulator](Register::A)
        AddWithCarryFromHl,
        /// Add an immediate and the previous carry to the [accumulator](Register::A)
        AddWithCarryImmediate
    ),
    0b10001000,
    cpu,
    memory,
    operand,
    accumulator,
    {
        #[cfg(test)]
        todo_or_die::issue_closed!("rust-lang", "rust", 85532);
        // Replace most of this with [carrying_add](https://doc.rust-lang.org/std/primitive.u8.html#method.carrying_add) once its standardized

        let previous_carry = cpu.read_flag(Flag::Carry);
        let (operand_with_carry, operand_overflow) = if previous_carry {
            operand.overflowing_add(1)
        } else {
            (operand, false)
        };
        let (result, carry_flag) = accumulator.overflowing_add(operand_with_carry);

        let carry_flag = carry_flag || operand_overflow;
        let zero_flag = result == 0;
        let subtract_flag = false;
        let half_carry_flag = (accumulator ^ operand ^ result) & 0b00010000 == 0b00010000;

        cpu.write_flag(Flag::Zero, zero_flag);
        cpu.write_flag(Flag::Subtract, subtract_flag);
        cpu.write_flag(Flag::HalfCarry, half_carry_flag);
        cpu.write_flag(Flag::Carry, carry_flag);

        result
    },
    fn instruction_works() {
        assert_result!((A: 100, B: 100,), (A: 200,));

        assert_result!((A: 100, B: 100, FLAG: Flag::Carry,), (A: 201,));
    },
    fn carry_works() {
        assert_result!((A: 100, B: 200,), (A: 44, FLAG: Flag::Carry,));
        assert_result!((A: 255, B: 0,), (A: 255, FLAG_UNSET: Flag::Carry,));
        assert_result!((A: 255, B: 0, FLAG: Flag::Carry,), (A: 0, FLAG: Flag::Carry,));
        assert_result!((A: 10, B: 255, FLAG: Flag::Carry,), (A: 10, FLAG: Flag::Carry,));
    },
    fn half_carry_flag_is_affected_by_previous_carry() {
        assert_result!((A: 15, B: 0,), (FLAG_UNSET: Flag::HalfCarry,));
        assert_result!((A: 15, B: 0, FLAG: Flag::Carry,), (FLAG: Flag::HalfCarry,));

        assert_result!((A: 0b00001010, B: 0b00001111, FLAG: Flag::Carry,), (FLAG: Flag::HalfCarry,));
        assert_result!((A: 0b00000000, B: 0b11111111, FLAG: Flag::Carry,), (A: 0b00000000, FLAG: Flag::HalfCarry,));
    }
);
