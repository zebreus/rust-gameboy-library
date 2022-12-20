use super::generate_instruction::{generate_instruction, prepare_generate_instruction};

prepare_generate_instruction!($, SetBitRegister, bit);
generate_instruction!(
    ///
    /// Sets the specified bit to `1`. Leaves all other bits unchanged.
    ///
    /// Flags:
    ///
    /// | [Zero](Flag::Zero) | [Subtract](Flag::Subtract) | [HalfCarry](Flag::HalfCarry) | [Carry](Flag::Carry) |
    /// |--------------------|----------------------------|------------------------------|----------------------|
    /// | unchanged          | unchanged                  | unchanged                    | unchanged            |
    #[doc(alias = "SET")]
    (
        /// Set a specific bit of the operand register to `1`.
        #[doc(alias = "SET n,R")]
        SetBitRegister,
        /// Set a specific bit of the value at the memory address in [DoubleRegister::HL] to `1`.
        #[doc(alias = "SET n,(HL)")]
        SetBitAtHl
    ),
    cb,
    0b11000000,
    cpu,
    memory,
    operand,
    (bit),
    "store into operand",
    { operand | bit.get_mask() },
    fn instruction_works() {
        assert_result!((B: 0b00000000, BIT: Bit::Zero,), (B: 0b00000001,));
        assert_result!((B: 0b11111111, BIT: Bit::Zero,), (B: 0b11111111,));
        assert_result!((B: 0b00000000, BIT: Bit::Three,), (B: 0b00001000,));
        assert_result!((B: 0b11111111, BIT: Bit::Three,), (B: 0b11111111,));
        assert_result!((B: 0b00000000, BIT: Bit::Seven,), (B: 0b10000000,));
    },
    fn does_not_affect_flags() {
        assert_result!((B: 0b00000000, FLAG: Flag::Zero, FLAG: Flag::Carry, FLAG: Flag::HalfCarry,  FLAG: Flag::Subtract,), (B: 0b00000001, FLAG: Flag::Zero, FLAG: Flag::Carry, FLAG: Flag::HalfCarry,  FLAG: Flag::Subtract,));
        assert_result!((B: 0b00000000,), (B: 0b00000001, FLAG_UNSET: Flag::Zero, FLAG_UNSET: Flag::Carry, FLAG_UNSET: Flag::HalfCarry,  FLAG_UNSET: Flag::Subtract,));
    }
);
