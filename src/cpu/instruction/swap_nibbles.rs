use super::generate_instruction::{generate_instruction, prepare_generate_instruction};

prepare_generate_instruction!($, SwapNibblesRegister);
generate_instruction!(
    ///
    /// Transformation when the bitorder is `0b76543210`
    ///
    /// | Bits before  | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
    /// |--------------|---|---|---|---|---|---|---|---|
    /// | Bits after   | 3 | 2 | 1 | 0 | 7 | 6 | 5 | 4 |
    ///
    /// Flags:
    ///
    /// | [Zero](Flag::Zero)  | [Subtract](Flag::Subtract) | [HalfCarry](Flag::HalfCarry) | [Carry](Flag::Carry) |
    /// |---------------------|----------------------------|------------------------------|----------------------|
    /// | true if result is 0 | false                      | false                        | false                |
    #[doc(alias = "SWAP")]
    (
        /// Swap the high and the low nibble of the operand left by one bit.
        #[doc(alias = "SWAP R")]
        SwapNibblesRegister,
        /// Swap the high and the low nibble of the value at the memory address in [DoubleRegister::HL] left by one bit.
        #[doc(alias = "SWAP (HL)")]
        SwapNibblesAtHl
    ),
    cb,
    0b00110000,
    cpu,
    memory,
    operand,
    "store into operand",
    {
        let result = (operand << 4) | (operand >> 4);
        let zero_flag = result == 0;

        cpu.write_flag(Flag::Zero, zero_flag);
        cpu.write_flag(Flag::Subtract, false);
        cpu.write_flag(Flag::HalfCarry, false);
        cpu.write_flag(Flag::Carry, false);

        result
    },
    fn instruction_works() {
        assert_result!((B: 0b11010000,), (B: 0b00001101,));
        assert_result!((B: 0b00001101,), (B: 0b11010000,));
    },
    fn flags_look_correct() {
        assert_result!((B: 0b00000000,), (B: 0b00000000, FLAG: Flag::Zero, FLAG_UNSET: Flag::HalfCarry, FLAG_UNSET: Flag::Carry, FLAG_UNSET: Flag::Subtract,));
        assert_result!((B: 0b00010001,), (B: 0b00010001, FLAG_UNSET: Flag::Zero, FLAG_UNSET: Flag::HalfCarry, FLAG_UNSET: Flag::Carry, FLAG_UNSET: Flag::Subtract,));
        assert_result!((B: 0b00000001,), (B: 0b00010000, FLAG_UNSET: Flag::Zero, FLAG_UNSET: Flag::HalfCarry, FLAG_UNSET: Flag::Carry, FLAG_UNSET: Flag::Subtract,));
        assert_result!((B: 0b11111111,), (B: 0b11111111, FLAG_UNSET: Flag::Zero, FLAG_UNSET: Flag::HalfCarry, FLAG_UNSET: Flag::Carry, FLAG_UNSET: Flag::Subtract,));
        assert_result!((B: 0b11111110,), (B: 0b11101111, FLAG_UNSET: Flag::Zero, FLAG_UNSET: Flag::HalfCarry, FLAG_UNSET: Flag::Carry, FLAG_UNSET: Flag::Subtract,));
    }
);
