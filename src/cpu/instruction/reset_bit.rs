use super::generate_instruction::{generate_instruction, prepare_generate_instruction};

prepare_generate_instruction!($, ResetBitRegister, bit);
generate_instruction!(
    ///
    /// Resets the specified bit to `0`. Leaves all other bits unchanged.
    ///
    /// Flags:
    ///
    /// | [Zero](Flag::Zero) | [Subtract](Flag::Subtract) | [HalfCarry](Flag::HalfCarry) | [Carry](Flag::Carry) |
    /// |--------------------|----------------------------|------------------------------|----------------------|
    /// | unchanged          | unchanged                  | unchanged                    | unchanged            |
    #[doc(alias = "RES")]
    (
        /// Reset a specific bit of the operand register to `0`.
        #[doc(alias = "RES n,R")]
        ResetBitRegister,
        /// Reset a specific bit of the value at the memory address in [DoubleRegister::HL] to `0`.
        #[doc(alias = "RES n,(HL)")]
        ResetBitAtHl
    ),
    cb,
    0b10000000,
    cpu,
    memory,
    operand,
    (bit),
    "store into operand",
    { operand & !bit.get_mask() },
    fn instruction_works() {
        assert_result!((B: 0b00000000, BIT: Bit::Zero,), (B: 0b00000000,));
        assert_result!((B: 0b11111111, BIT: Bit::Zero,), (B: 0b11111110,));
        assert_result!((B: 0b00000000, BIT: Bit::Three,), (B: 0b00000000,));
        assert_result!((B: 0b11111111, BIT: Bit::Three,), (B: 0b11110111,));
        assert_result!((B: 0b11111111, BIT: Bit::Seven,), (B: 0b01111111,));
    },
    fn does_not_affect_flags() {
        assert_result!((B: 0b11111111, FLAG: Flag::Zero, FLAG: Flag::Carry, FLAG: Flag::HalfCarry,  FLAG: Flag::Subtract,), (B: 0b11111110, FLAG: Flag::Zero, FLAG: Flag::Carry, FLAG: Flag::HalfCarry,  FLAG: Flag::Subtract,));
        assert_result!((B: 0b11111111,), (B: 0b11111110, FLAG_UNSET: Flag::Zero, FLAG_UNSET: Flag::Carry, FLAG_UNSET: Flag::HalfCarry,  FLAG_UNSET: Flag::Subtract,));
    }
);
