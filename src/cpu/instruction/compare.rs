use super::generate_instruction::{generate_instruction, prepare_generate_instruction};

prepare_generate_instruction!($, CompareRegister);
generate_instruction!(
    ///
    /// | [Zero](Flag::Zero)  | [Subtract](Flag::Subtract) | [HalfCarry](Flag::HalfCarry)        | [Carry](Flag::Carry)       |
    /// |---------------------|----------------------------|-------------------------------------|----------------------------|
    /// | true if result is 0 | true                       | true if the lower nibble overflowed | true if a overflow occured |
    (
        /// Check if the operand register and the [accumulator](Register::A) are equal.
        ///
        /// Basically identical with [SubtractRegister](super::SubtractRegister), but the result is discarded.
        CompareRegister,
        /// Check if the value at the memory address in [DoubleRegister::HL] and the [accumulator](Register::A) are equal.
        ///
        /// Basically identical with [SubtractFromHl](super::SubtractFromHl), but the result is discarded.
        CompareFromHl,
        /// Check if an immediate and the [accumulator](Register::A) are equal.
        ///
        /// Basically identical with [SubtractImmediate](super::SubtractImmediate), but the result is discarded.
        CompareImmediate
    ),
    0b10111000,
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

        accumulator
    },
    fn instruction_works() {
        assert_result!((A: 100, B: 100,), (A: 100, FLAG: Flag::Zero,));
    },
    fn zero_flag_works() {
        assert_result!((A: 100, B: 100,), (A: 100, FLAG: Flag::Zero, FLAG: Flag::Subtract,));
        assert_result!((A: 100, B: 50,), (A: 100, FLAG: Flag::Subtract, FLAG_UNSET: Flag::Zero,));
    },
    fn carry_flag_works() {
        assert_result!((A: 0b00000001, B: 0b00000010,), (A: 0b00000001, FLAG: Flag::Carry, FLAG: Flag::Subtract,));

        assert_result!((A: 0b01111111, B: 0b00000001,), (A: 0b01111111,FLAG_UNSET: Flag::Carry,));
    },
    fn some_examples_from_reddit_set_the_correct_flags() {
        assert_result!((A: 0x11, B: 0x0f,), (A: 0x11, FLAG: Flag::HalfCarry,));
    }
);
