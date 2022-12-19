use super::generate_instruction::{generate_instruction, prepare_generate_instruction};

prepare_generate_instruction!($, SubtractRegister);
generate_instruction!(
    ///
    /// | [Zero](Flag::Zero)  | [Subtract](Flag::Subtract) | [HalfCarry](Flag::HalfCarry)        | [Carry](Flag::Carry)       |
    /// |---------------------|----------------------------|-------------------------------------|----------------------------|
    /// | true if result is 0 | true                       | true if the lower nibble overflowed | true if a overflow occured |
    (
        /// Subtract the operand register from the [accumulator](Register::A).
        SubtractRegister,
        /// Subtract the value at the memory address in [DoubleRegister::HL] from the [accumulator](Register::A)
        SubtractFromHl,
        /// Subtract an immediate from the [accumulator](Register::A)
        SubtractImmediate
    ),
    0b10010000,
    cpu,
    memory,
    operand,
    accumulator,
    {
        let (result, carry_flag) = accumulator.overflowing_sub(operand);
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
        assert_result!((A: 100, B: 100,), (A: 0,));
    },
    fn zero_flag_works() {
        assert_result!((A: 100, B: 100,), (FLAG: Flag::Zero, FLAG: Flag::Subtract,));
        assert_result!((A: 100, B: 50,), (A: 50, FLAG: Flag::Subtract, FLAG_UNSET: Flag::Zero,));
    },
    fn carry_flag_works() {
        assert_result!((A: 0b00000001, B: 0b00000010,), (FLAG: Flag::Carry, FLAG: Flag::Subtract,));

        assert_result!((A: 0b01111111, B: 0b00000001,), (FLAG_UNSET: Flag::Carry,));
    },
    fn some_examples_from_reddit_set_the_correct_flags() {
        assert_result!((A: 0x11, B: 0x0f,), (FLAG: Flag::HalfCarry,));
    }
);
