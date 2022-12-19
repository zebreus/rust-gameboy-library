use super::generate_instruction::{generate_instruction, prepare_generate_instruction};

prepare_generate_instruction!($, BitwiseExclusiveOrRegister);
generate_instruction!(
    ///
    /// | [Zero](Flag::Zero)  | [Subtract](Flag::Subtract) | [HalfCarry](Flag::HalfCarry) | [Carry](Flag::Carry)       |
    /// |---------------------|----------------------------|------------------------------|----------------------------|
    /// | true if result is 0 | false                      | false                        | false                      |
    #[doc(alias = "XOR")]
    (
        /// [Bitwise exclusive or](https://wikipedia.org/wiki/Bitwise_operation#XOR) between operand register and the [accumulator](Register::A). The result is stored in the [accumulator](Register::A).
        BitwiseExclusiveOrRegister,
        /// [Bitwise exclusive or](https://wikipedia.org/wiki/Bitwise_operation#XOR) between the value at the memory address in [DoubleRegister::HL] and the [accumulator](Register::A). The result is stored in the [accumulator](Register::A).
        BitwiseExclusiveOrFromHl,
        /// [Bitwise exclusive or](https://wikipedia.org/wiki/Bitwise_operation#XOR) the immediate after the opcode and the [accumulator](Register::A). The result is stored in the [accumulator](Register::A).
        BitwiseExclusiveOrImmediate
    ),
    0b10101000,
    cpu,
    memory,
    operand,
    accumulator,
    {
        let result = accumulator ^ operand;
        let zero_flag = result == 0;

        cpu.write_flag(Flag::Zero, zero_flag);
        cpu.write_flag(Flag::Subtract, false);
        cpu.write_flag(Flag::HalfCarry, false);
        cpu.write_flag(Flag::Carry, false);

        result
    },
    fn instruction_works() {
        assert_result!((A: 0b11111111, B: 0b00000000,), (A: 0b11111111,));
        assert_result!((A: 0b11111111, B: 0b10101010,), (A: 0b01010101,));
        assert_result!((A: 0b11111111, B: 0b11111111,), (A: 0b00000000,));
        assert_result!((A: 0b10101010, B: 0b11111111,), (A: 0b01010101,));
        assert_result!((A: 0b10101010, B: 0b10101010,), (A: 0b00000000,));
        assert_result!((A: 0b01010101, B: 0b10101010,), (A: 0b11111111,));
        assert_result!((A: 0b00000000, B: 0b00000000,), (A: 0b00000000,));
    },
    fn flags_look_correct() {
        assert_result!((A: 0b10101010, B: 0b10101010,), (A: 0, FLAG: Flag::Zero, FLAG_UNSET: Flag::HalfCarry, FLAG_UNSET: Flag::Carry, FLAG_UNSET: Flag::Subtract,));
        assert_result!((A: 0b11111111, B: 0b10101010,), ( FLAG_UNSET: Flag::HalfCarry, FLAG_UNSET: Flag::Zero, FLAG_UNSET: Flag::Carry, FLAG_UNSET: Flag::Subtract,));
    }
);
