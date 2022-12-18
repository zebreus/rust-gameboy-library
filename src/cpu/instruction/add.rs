use super::generate_instruction::{generate_instruction, prepare_generate_instruction};

prepare_generate_instruction!($, AddRegister);
generate_instruction!(
    ///
    /// | [Zero](Flag::Zero)  | [Subtract](Flag::Subtract) | [HalfCarry](Flag::HalfCarry)        | [Carry](Flag::Carry)       |
    /// |---------------------|----------------------------|-------------------------------------|----------------------------|
    /// | true if result is 0 | false                      | true if the lower nibble overflowed | true if a overflow occured |
    (
        /// Add the operand register to the [accumulator](Register::A).
        AddRegister,
        /// Add the value at the memory address in [DoubleRegister::HL] to the [accumulator](Register::A)
        AddFromHl,
        /// Add an immediate to the [accumulator](Register::A)
        AddImmediate
    ),
    0b10000000,
    cpu,
    memory,
    operand,
    accumulator,
    {
        let result = accumulator.wrapping_add(operand);

        // set flags
        let zero_flag = result == 0;
        let subtract_flag = false;
        let half_carry_flag = (accumulator ^ operand ^ result) & 0b00010000 == 0b00010000;
        let carry_flag = result < accumulator;

        cpu.write_flag(Flag::Zero, zero_flag);
        cpu.write_flag(Flag::Subtract, subtract_flag);
        cpu.write_flag(Flag::HalfCarry, half_carry_flag);
        cpu.write_flag(Flag::Carry, carry_flag);

        result
    },
    fn instruction_works() {
        assert_result!((A: 100, B: 100,), (A: 200,));
    },
    fn zero_flag_works() {
        assert_result!((A: 100, B: 100,), (FLAG_UNSET: Flag::Zero,));
    },
    fn carry_flag_works() {
        assert_result!((A: 0b11111111, B: 0b00000001,), (FLAG: Flag::Carry,));

        assert_result!((A: 0b01111111, B: 0b00000001,), (FLAG_UNSET: Flag::Carry,));
    },
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
);
