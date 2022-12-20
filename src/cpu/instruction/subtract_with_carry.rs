use super::generate_instruction::{generate_instruction, prepare_generate_instruction};

prepare_generate_instruction!($, SubtractWithCarryRegister);
generate_instruction!(
    ///
    /// If [Flag::Carry] is false this just subtracts the operand. If [Flag::Carry] is true this subtracts the operand plus 1.
    ///
    /// | [Zero](Flag::Zero)  | [Subtract](Flag::Subtract) | [HalfCarry](Flag::HalfCarry)        | [Carry](Flag::Carry)       |
    /// |---------------------|----------------------------|-------------------------------------|----------------------------|
    /// | true if result is 0 | true                       | true if the lower nibble overflowed | true if a overflow occured |
    #[doc(alias = "SBC")]
    (
        /// Subtract the operand register and the previous carry from the [accumulator](Register::A).
        SubtractWithCarryRegister,
        /// Subtract the previous carry and the value at the memory Address in [DoubleRegister::HL] from the [accumulator](Register::A)
        SubtractWithCarryFromHl,
        /// Subtract an immediate and the previous carry from the [accumulator](Register::A)
        SubtractWithCarryImmediate
    ),
    0b10001100,
    cpu,
    memory,
    operand,
    accumulator,
    {
        #[cfg(test)]
        todo_or_die::issue_closed!("rust-lang", "rust", 85532);
        // Replace most of this with [borrowing_sub](https://doc.rust-lang.org/std/primitive.u8.html#method.borrowing_sub) once its standardized

        let previous_carry = cpu.read_flag(Flag::Carry);
        let (operand_with_carry, operand_overflow) = if previous_carry {
            operand.overflowing_add(1)
        } else {
            (operand, false)
        };
        let (result, carry_flag) = accumulator.overflowing_sub(operand_with_carry);

        let carry_flag = carry_flag || operand_overflow;
        let zero_flag = result == 0;
        let subtract_flag = true;
        let half_carry_flag = (accumulator ^ operand ^ result) & 0b00010000 == 0b00010000;

        cpu.write_flag(Flag::Zero, zero_flag);
        cpu.write_flag(Flag::Subtract, subtract_flag);
        cpu.write_flag(Flag::HalfCarry, half_carry_flag);
        cpu.write_flag(Flag::Carry, carry_flag);

        result
    },
    fn instruction_works() {
        assert_result!((A: 100, B: 100,), (A: 0, FLAG: Flag::Subtract,));
        assert_result!((A: 100, B: 99, FLAG: Flag::Carry,), (A: 0, FLAG: Flag::Subtract,));
    },
    fn zero_flag_works() {
        assert_result!((A: 100, B: 100,), (FLAG: Flag::Zero, FLAG: Flag::Subtract,));
        assert_result!((A: 100, B: 50,), (A: 50, FLAG: Flag::Subtract, FLAG_UNSET: Flag::Zero,));
    },
    fn carry_flag_works() {
        assert_result!((A: 0b00000001, B: 0b00000010,), (FLAG: Flag::Carry, FLAG: Flag::Subtract,));

        assert_result!((A: 0b01111111, B: 0b00000001,), (FLAG_UNSET: Flag::Carry,));
        assert_result!((A: 100, B: 200,), (FLAG: Flag::Carry,));
        assert_result!((A: 255, B: 0,), (A: 255, FLAG_UNSET: Flag::Carry,));
        assert_result!((A: 0, B: 0, FLAG: Flag::Carry,), (A: 255, FLAG: Flag::Carry,));
        assert_result!((A: 10, B: 255, FLAG: Flag::Carry,), (A: 10, FLAG: Flag::Carry,));
    },
    fn some_examples_from_reddit_set_the_correct_flags() {
        assert_result!((A: 0x11, B: 0x0f,), (FLAG: Flag::HalfCarry,));
    },
    fn half_carry_flag_is_affected_by_previous_carry() {
        assert_result!((A: 15, B: 15,), (FLAG_UNSET: Flag::HalfCarry,));
        assert_result!((A: 15, B: 15, FLAG: Flag::Carry,), (FLAG: Flag::HalfCarry,));

        assert_result!((A: 30, B: 15, FLAG: Flag::Carry,), (FLAG: Flag::HalfCarry,));
    },
    fn half_carry_flag_works_even_if_operand_overflows() {
        assert_result!((A: 10, B: 255,), (A: 11, FLAG: Flag::HalfCarry,));
        assert_result!((A: 10, B: 255, FLAG: Flag::Carry,), (A: 10, FLAG: Flag::HalfCarry,));
    }
);
