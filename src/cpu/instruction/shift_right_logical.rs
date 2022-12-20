use super::generate_instruction::{generate_instruction, prepare_generate_instruction};

prepare_generate_instruction!($, ShiftRightLogicalRegister);
generate_instruction!(
    ///
    /// Transformation when the bitorder is `0b76543210`
    ///
    /// | Bits before  | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
    /// |--------------|---|---|---|---|---|---|---|---|
    /// | Bits after   |`0`| 7 | 6 | 5 | 4 | 3 | 2 | 1 |
    ///
    /// Flags:
    ///
    /// | [Zero](Flag::Zero)  | [Subtract](Flag::Subtract) | [HalfCarry](Flag::HalfCarry) | [Carry](Flag::Carry)          |
    /// |---------------------|----------------------------|------------------------------|-------------------------------|
    /// | true if result is 0 | false                      | false                        | set to the value of old bit 0 |
    #[doc(alias = "SRL")]
    (
        /// [Logical shift](https://en.wikipedia.org/wiki/Bitwise_operation#Logical_shift) the operand right by one bit. Bit 7 will be set to 0.
        #[doc(alias = "SRL R")]
        ShiftRightLogicalRegister,
        /// [Logical shift](https://en.wikipedia.org/wiki/Bitwise_operation#Logical_shift) the value at the memory address in [DoubleRegister::HL] right by one bit. Bit 7 will be set to 0.
        #[doc(alias = "SRL (HL)")]
        ShiftRightLogicalAtHl
    ),
    cb,
    0b00111000,
    cpu,
    memory,
    operand,
    "store into operand",
    {
        let result = operand >> 1;
        let zero_flag = result == 0;
        let carry_flag = operand % 2 == 1;

        cpu.write_flag(Flag::Zero, zero_flag);
        cpu.write_flag(Flag::Subtract, false);
        cpu.write_flag(Flag::HalfCarry, false);
        cpu.write_flag(Flag::Carry, carry_flag);

        result
    },
    fn instruction_works() {
        assert_result!((B: 0b00000001,), (B: 0b00000000,));
        assert_result!((B: 0b01000000,), (B: 0b00100000,));
        assert_result!((B: 0b10000000,), (B: 0b01000000,));
    },
    fn flags_look_correct() {
        assert_result!((B: 0b00000000,), (B: 0b00000000, FLAG: Flag::Zero, FLAG_UNSET: Flag::HalfCarry, FLAG_UNSET: Flag::Carry, FLAG_UNSET: Flag::Subtract,));
        assert_result!((B: 0b00000001,), (B: 0b00000000, FLAG: Flag::Zero, FLAG: Flag::Carry, FLAG_UNSET: Flag::HalfCarry, FLAG_UNSET: Flag::Subtract,));
        assert_result!((B: 0b10000001,), (B: 0b01000000, FLAG: Flag::Carry, FLAG_UNSET: Flag::Zero, FLAG_UNSET: Flag::HalfCarry, FLAG_UNSET: Flag::Subtract,));
        assert_result!((B: 0b10000000,), (B: 0b01000000, FLAG_UNSET: Flag::Zero, FLAG_UNSET: Flag::HalfCarry, FLAG_UNSET: Flag::Carry, FLAG_UNSET: Flag::Subtract,));
        assert_result!((B: 0b01000001,), (B: 0b00100000, FLAG: Flag::Carry, FLAG_UNSET: Flag::Zero, FLAG_UNSET: Flag::HalfCarry, FLAG_UNSET: Flag::Subtract,));
    }
);
