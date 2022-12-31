use super::generate_instruction::{generate_instruction, prepare_generate_instruction};

prepare_generate_instruction!($, CheckBitRegister, bit);
generate_instruction!(
    ///
    /// Does not modify the operand.
    ///
    /// | Bits before  | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
    /// |--------------|---|---|---|---|---|---|---|---|
    /// | Bits after   | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
    ///
    /// Flags:
    ///
    /// | [Zero](Flag::Zero)     | [Subtract](Flag::Subtract) | [HalfCarry](Flag::HalfCarry) | [Carry](Flag::Carry) |
    /// |------------------------|----------------------------|------------------------------|----------------------|
    /// | true if the bit is `0` | false                      | true                         | unchanged            |
    #[doc(alias = "BIT")]
    (
        /// Check a specific bit in the operand register. Sets [Flag::Zero] if the bit is `0`
        #[doc(alias = "BIT n,R")]
        CheckBitRegister,
        /// Check a specific bit in the value at the memory address in [DoubleRegister::HL]. Sets [Flag::Zero] if the bit is `0`
        #[doc(alias = "BIT n,(HL)")]
        CheckBitAtHl
    ),
    cb,
    0b01000000,
    cpu,
    memory,
    operand,
    (bit),
    [dont_write],
    "store into operand",
    {
        let zero_flag = (operand & bit.get_mask()) == 0;

        cpu.write_flag(Flag::Zero, zero_flag);
        cpu.write_flag(Flag::Subtract, false);
        cpu.write_flag(Flag::HalfCarry, true);

        operand
    },
    fn instruction_works() {
        assert_result!((B: 0b10101010, BIT: Bit::Zero,), (B: 0b10101010, FLAG: Flag::Zero,));
        assert_result!((B: 0b01010101, BIT: Bit::Zero,), (B: 0b01010101, FLAG_UNSET: Flag::Zero,));
    },
    fn flags_look_correct() {
        assert_result!((B: 0b00000000,), (B: 0b00000000, FLAG: Flag::Zero, FLAG: Flag::HalfCarry,  FLAG_UNSET: Flag::Subtract,));

        assert_result!((B: 0b10101010, BIT: Bit::Zero,), (B: 0b10101010, FLAG: Flag::Zero,));
        assert_result!((B: 0b10101010, BIT: Bit::One,), (B: 0b10101010, FLAG_UNSET: Flag::Zero,));
        assert_result!((B: 0b10101010, BIT: Bit::Two,), (B: 0b10101010, FLAG: Flag::Zero,));
        assert_result!((B: 0b10101010, BIT: Bit::Three,), (B: 0b10101010, FLAG_UNSET: Flag::Zero,));
        assert_result!((B: 0b10101010, BIT: Bit::Four,), (B: 0b10101010, FLAG: Flag::Zero,));
        assert_result!((B: 0b10101010, BIT: Bit::Five,), (B: 0b10101010, FLAG_UNSET: Flag::Zero,));
        assert_result!((B: 0b10101010, BIT: Bit::Six,), (B: 0b10101010, FLAG: Flag::Zero,));
        assert_result!((B: 0b10101010, BIT: Bit::Seven,), (B: 0b10101010, FLAG_UNSET: Flag::Zero,));
    },
    fn does_not_affect_carry() {
        assert_result!((B: 0b00000000, FLAG: Flag::Carry,), (B: 0b00000000, FLAG: Flag::Carry, ));
        assert_result!((B: 0b00000000,), (B: 0b00000000, FLAG_UNSET: Flag::Carry,));
        assert_result!((B: 0b00000001, FLAG: Flag::Carry,), (B: 0b00000001, FLAG: Flag::Carry, ));
        assert_result!((B: 0b00000001,), (B: 0b00000001, FLAG_UNSET: Flag::Carry,));
    },
    fn sets_subtract_and_halfcarry_correctly() {
        assert_result!((B: 0b00000000,), (B: 0b00000000, FLAG: Flag::HalfCarry,  FLAG_UNSET: Flag::Subtract,));
        assert_result!((B: 0b11111111,), (B: 0b11111111, FLAG: Flag::HalfCarry,  FLAG_UNSET: Flag::Subtract,));
    }
);
