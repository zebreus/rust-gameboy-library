use super::generate_instruction::{generate_instruction, prepare_generate_instruction};

prepare_generate_instruction!($, DecrementRegister);
generate_instruction!(
    ///
    /// | [Zero](Flag::Zero)  | [Subtract](Flag::Subtract) | [HalfCarry](Flag::HalfCarry)        | [Carry](Flag::Carry) |
    /// |---------------------|----------------------------|-------------------------------------|----------------------|
    /// | true if result is 0 | true                       | true if the lower nibble overflowed | unchanged            |
    #[doc(alias = "DEC")]
    (
        /// Decrement the operand [Register] by one.
        #[doc(alias = "DEC R")]
        DecrementRegister,
        /// Decrement the value at the memory address in [DoubleRegister::HL] by one.
        #[doc(alias = "DEC (HL)")]
        DecrementAtHl
    ),
    0b00000101,
    3,
    cpu,
    memory,
    operand,
    "store into operand",
    {
        let result = operand.wrapping_sub(1);
        let half_carry_flag = (0b00000001 ^ operand ^ result) & 0b00010000 == 0b00010000;

        cpu.write_flag(Flag::Zero, result == 0);
        cpu.write_flag(Flag::Subtract, true);
        cpu.write_flag(Flag::HalfCarry, half_carry_flag);

        result
    },
    fn instruction_works() {
        assert_result!(( B: 1,), (B: 0, FLAG: Flag::Zero, FLAG: Flag::Subtract,));
        assert_result!(( B: 100,), (B: 99,));
    },
    fn zero_flag_works() {
        assert_result!(( B: 2,), (B: 1, FLAG_UNSET: Flag::Zero,));
        assert_result!(( B: 1,), (B: 0, FLAG: Flag::Zero,));
        assert_result!(( B: 0,), (B: 255, FLAG_UNSET: Flag::Zero,));
        assert_result!(( B: 255,), (B: 254, FLAG_UNSET: Flag::Zero,));
        assert_result!(( B: 254,), (B: 253, FLAG_UNSET: Flag::Zero,));
    },
    fn carry_flag_is_unchanged() {
        assert_result!((B: 2,), (FLAG_UNSET: Flag::Carry,));
        assert_result!((B: 2, FLAG: Flag::Carry,), (FLAG: Flag::Carry,));
        assert_result!((B: 1,), (FLAG_UNSET: Flag::Carry,));
        assert_result!((B: 1, FLAG: Flag::Carry,), (FLAG: Flag::Carry,));
        assert_result!((B: 0,), (FLAG_UNSET: Flag::Carry,));
        assert_result!((B: 0, FLAG: Flag::Carry,), (FLAG: Flag::Carry,));
        assert_result!((B: 255,), (FLAG_UNSET: Flag::Carry,));
        assert_result!((B: 255, FLAG: Flag::Carry,), (FLAG: Flag::Carry,));
        assert_result!((B: 254,), (FLAG_UNSET: Flag::Carry,));
        assert_result!((B: 254, FLAG: Flag::Carry,), (FLAG: Flag::Carry,));
    },
    fn half_carry_flag_works() {
        assert_result!((B: 0b00001111,), (FLAG_UNSET: Flag::HalfCarry,));
        assert_result!((B: 0b00000001,), (FLAG_UNSET: Flag::HalfCarry,));
        assert_result!((B: 0b00000000,), (FLAG: Flag::HalfCarry,));
        assert_result!((B: 0b01000001,), (FLAG_UNSET: Flag::HalfCarry,));
        assert_result!((B: 0b01000000,), (FLAG: Flag::HalfCarry,));
        assert_result!((B: 0b11110001,), (FLAG_UNSET: Flag::HalfCarry,));
        assert_result!((B: 0b11110000,), (FLAG: Flag::HalfCarry,));
    }
);
