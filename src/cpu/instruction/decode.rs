use crate::cpu::instruction::phases::{ThreePhases, TwoPhases};
use crate::cpu::{ConditionCode, DoubleRegister, Register};
use bitmatch::bitmatch;

use super::phases::{FivePhases, FourPhases, SixPhases};
use super::{
    load_accumulator_to_hl_and_decrement::LoadAccumulatorToHlAndDecrement,
    load_accumulator_to_hl_and_increment::LoadAccumulatorToHlAndIncrement,
    load_accumulator_to_immediate_offset::LoadAccumulatorToImmediateOffset,
    load_from_hl_to_register::LoadFromHlToRegister,
    load_from_immediate_offset_to_accumulator::LoadFromImmediateOffsetToAccumulator,
    load_from_register_to_register::LoadFromRegisterToRegister,
    load_hl_to_accumulator_and_decrement::LoadHlToAccumulatorAndDecrement,
    load_hl_to_accumulator_and_increment::LoadHlToAccumulatorAndIncrement,
    load_immediate_to_register::LoadImmediateToRegister, InstructionEnum,
};
use super::{
    AddDoubleRegisterToHl, AddImmediateOffsetToSp, Call, CallConditional, Complement,
    DecrementDoubleRegister, DisableInterrupts, EnableInterrupts, Halt, HaltAndCatchFire,
    IncrementDoubleRegister, InvertCarry, JumpByImmediateOffset, JumpByImmediateOffsetConditional,
    JumpToHl, JumpToImmediateAddress, JumpToImmediateAddressConditional,
    LoadAccumulatorToDoubleRegister, LoadAccumulatorToImmediateAddress,
    LoadAccumulatorToRegisterCOffset, LoadFromDoubleRegisterToAccumulator,
    LoadFromImmediateAddressToAccumulator, LoadFromRegisterCOffsetToAccumulator, LoadHlToSp,
    LoadImmediateToDoubleRegister, LoadImmediateToHl, LoadRegisterToHl,
    LoadSpPlusImmediateOffsetToHl, LoadSpToImmediateAddress, Nop, PopDoubleRegister,
    PushDoubleRegister, Restart, Return, ReturnConditional, ReturnFromInterrupt,
    RotateAccumulatorLeft, RotateAccumulatorLeftThroughCarry, RotateAccumulatorRight,
    RotateAccumulatorRightThroughCarry, SetCarry, Stop, ToBinaryCodedDecimal,
};

macro_rules! decode_arithmetic {
    ($a:ident, $register_instruction:ident, $hl_instruction:ident) => {
        match $a {
            0b00000110 => super::$hl_instruction {
                phase: TwoPhases::First,
            }
            .into(),
            _ => super::$register_instruction {
                operand: Register::try_from($a)
                    .expect("3 bit value should always correspond to a register"),
            }
            .into(),
        }
    };
}

macro_rules! decode_operand_arithmetic {
    ($a:ident, $register_instruction:ident, $hl_instruction:ident) => {
        match $a {
            0b00000110 => super::$hl_instruction {
                phase: ThreePhases::First,
            }
            .into(),
            _ => super::$register_instruction {
                operand: Register::try_from($a)
                    .expect("3 bit value should always correspond to a register"),
            }
            .into(),
        }
    };
}

macro_rules! decode_arithmetic_immediate {
    ($immediate_instruction:ident) => {
        super::$immediate_instruction {
            phase: TwoPhases::First,
            value: 0,
        }
        .into()
    };
}

/// Create a instruction from an opcode.
///
/// Some instructions are longer than one byte because they have immediate arguments. The additional arguments are not loaded here. Instead they are loaded in the appropriate cycles when executing the instructions.
///
/// # Examples
///
/// ```
/// # use rust_gameboy_library::cpu::Register;
/// # use rust_gameboy_library::cpu::instruction::InstructionEnum;
/// # use rust_gameboy_library::cpu::instruction::LoadFromRegisterToRegister;
/// # use rust_gameboy_library::cpu::instruction::decode;
/// #
/// let load_a_to_c = 0b01111001u8;
/// let instruction = decode(load_a_to_c);
/// assert!(matches!(
///     instruction,
///     InstructionEnum::LoadFromRegisterToRegister (LoadFromRegisterToRegister {
///         source: Register::A,
///         destination: Register::C
///     })
/// ))
/// ```
#[bitmatch]
pub fn decode(byte: u8) -> InstructionEnum {
    #[bitmatch]
    // We probably cannot get rid of this massive match clause
    match byte {
        "01aaa110" => LoadFromHlToRegister {
            destination: Register::try_from(a)
                .expect("3 bit value should always correspond to a register"),
            phase: TwoPhases::First,
        }
        .into(),
        "01110aaa" => LoadRegisterToHl {
            source: Register::try_from(a)
                .expect("3 bit value should always correspond to a register"),
            phase: TwoPhases::First,
        }
        .into(),
        "01aaabbb" => LoadFromRegisterToRegister {
            source: Register::try_from(a)
                .expect("3 bit value should always correspond to a register"),
            destination: Register::try_from(b)
                .expect("3 bit value should always correspond to a register"),
        }
        .into(),
        "00aaa110" => LoadImmediateToRegister {
            destination: Register::try_from(a)
                .expect("3 bit value should always correspond to a register"),
            value: 0,
            phase: TwoPhases::First,
        }
        .into(),
        "00aa0001" => LoadImmediateToDoubleRegister {
            destination: DoubleRegister::try_from(a)
                .expect("3 bit value should always correspond to a register"),
            value: 0,
            phase: ThreePhases::First,
        }
        .into(),
        "11aa0101" => PushDoubleRegister {
            source: DoubleRegister::try_from(a)
                .expect("3 bit value should always correspond to a register"),
            phase: FourPhases::First,
        }
        .into(),
        "11aa0001" => PopDoubleRegister {
            destination: DoubleRegister::try_from(a)
                .expect("3 bit value should always correspond to a register"),
            phase: ThreePhases::First,
        }
        .into(),
        "110aa010" => JumpToImmediateAddressConditional {
            condition: ConditionCode::try_from(a)
                .expect("3 bit value should always correspond to a register"),
            address: 0,
            phase: FourPhases::First,
        }
        .into(),
        "001aa000" => JumpByImmediateOffsetConditional {
            condition: ConditionCode::try_from(a)
                .expect("3 bit value should always correspond to a register"),
            offset: 0,
            phase: ThreePhases::First,
        }
        .into(),
        "110aa100" => CallConditional {
            condition: ConditionCode::try_from(a)
                .expect("3 bit value should always correspond to a register"),
            address: 0,
            phase: SixPhases::First,
        }
        .into(),
        "110aa000" => ReturnConditional {
            condition: ConditionCode::try_from(a)
                .expect("3 bit value should always correspond to a register"),
            phase: FivePhases::First,
        }
        .into(),
        "00110110" => LoadImmediateToHl {
            value: 0,
            phase: ThreePhases::First,
        }
        .into(),
        "11110000" => LoadFromImmediateOffsetToAccumulator {
            offset: 0,
            phase: ThreePhases::First,
        }
        .into(),
        "11100000" => LoadAccumulatorToImmediateOffset {
            offset: 0,
            phase: ThreePhases::First,
        }
        .into(),
        "00111010" => LoadHlToAccumulatorAndDecrement {
            phase: TwoPhases::First,
        }
        .into(),
        "00110010" => LoadAccumulatorToHlAndDecrement {
            phase: TwoPhases::First,
        }
        .into(),
        "00101010" => LoadHlToAccumulatorAndIncrement {
            phase: TwoPhases::First,
        }
        .into(),
        "00100010" => LoadAccumulatorToHlAndIncrement {
            phase: TwoPhases::First,
        }
        .into(),
        "11110010" => LoadFromRegisterCOffsetToAccumulator {
            phase: TwoPhases::First,
        }
        .into(),
        "11100010" => LoadAccumulatorToRegisterCOffset {
            phase: TwoPhases::First,
        }
        .into(),
        "11111010" => LoadFromImmediateAddressToAccumulator {
            address: 0,
            phase: FourPhases::First,
        }
        .into(),
        "11101010" => LoadAccumulatorToImmediateAddress {
            address: 0,
            phase: FourPhases::First,
        }
        .into(),
        "000a1010" => LoadAccumulatorToDoubleRegister {
            address_register: match a {
                0 => DoubleRegister::BC,
                _ => DoubleRegister::DE,
            },
            phase: TwoPhases::First,
        }
        .into(),
        "000a0010" => LoadFromDoubleRegisterToAccumulator {
            address_register: match a {
                0 => DoubleRegister::BC,
                _ => DoubleRegister::DE,
            },
            phase: TwoPhases::First,
        }
        .into(),
        "00001000" => LoadSpToImmediateAddress {
            address: 0,
            phase: FivePhases::First,
        }
        .into(),
        "11111001" => LoadHlToSp {
            phase: TwoPhases::First,
        }
        .into(),
        "11001101" => Call {
            address: 0,
            phase: SixPhases::First,
        }
        .into(),
        "11000011" => JumpToImmediateAddress {
            address: 0,
            phase: FourPhases::First,
        }
        .into(),
        "00011000" => JumpByImmediateOffset {
            offset: 0,
            phase: ThreePhases::First,
        }
        .into(),
        "11101001" => JumpToHl {
            phase: TwoPhases::First,
        }
        .into(),
        "11001001" => Return {
            phase: FourPhases::First,
        }
        .into(),
        "11011001" => ReturnFromInterrupt {
            phase: FourPhases::First,
        }
        .into(),
        "11110011" => DisableInterrupts {}.into(),
        "11111011" => EnableInterrupts {}.into(),
        "01110110" => Halt {}.into(),
        "00010000" => Stop {}.into(),
        "00000000" => Nop {}.into(),
        "00100111" => ToBinaryCodedDecimal {}.into(),
        "00101111" => Complement {}.into(),
        "00111111" => InvertCarry {}.into(),
        "00110111" => SetCarry {}.into(),
        "10000aaa" => decode_arithmetic!(a, AddRegister, AddFromHl),
        "11000110" => decode_arithmetic_immediate!(AddImmediate),
        "10001aaa" => decode_arithmetic!(a, AddWithCarryRegister, AddWithCarryFromHl),
        "11001110" => decode_arithmetic_immediate!(AddWithCarryImmediate),
        "10010aaa" => decode_arithmetic!(a, SubtractRegister, SubtractFromHl),
        "11010110" => decode_arithmetic_immediate!(SubtractImmediate),
        "10011aaa" => decode_arithmetic!(a, SubtractWithCarryRegister, SubtractWithCarryFromHl),
        "11011110" => decode_arithmetic_immediate!(SubtractWithCarryImmediate),
        "10100aaa" => decode_arithmetic!(a, BitwiseAndRegister, BitwiseAndFromHl),
        "11100110" => decode_arithmetic_immediate!(BitwiseAndImmediate),
        "10101aaa" => decode_arithmetic!(a, BitwiseExclusiveOrRegister, BitwiseExclusiveOrFromHl),
        "11101110" => decode_arithmetic_immediate!(BitwiseExclusiveOrImmediate),
        "10110aaa" => decode_arithmetic!(a, BitwiseOrRegister, BitwiseOrFromHl),
        "11110110" => decode_arithmetic_immediate!(BitwiseOrImmediate),
        "10111aaa" => decode_arithmetic!(a, CompareRegister, CompareFromHl),
        "11111110" => decode_arithmetic_immediate!(CompareImmediate),
        "00aaa100" => decode_operand_arithmetic!(a, IncrementRegister, IncrementAtHl),
        "00aaa101" => decode_operand_arithmetic!(a, DecrementRegister, DecrementAtHl),
        "11aaa111" => Restart {
            address: a.into(),
            phase: FourPhases::First,
        }
        .into(),
        "11101000" => AddImmediateOffsetToSp {
            offset: 0,
            phase: FourPhases::First,
        }
        .into(),
        "11111000" => LoadSpPlusImmediateOffsetToHl {
            offset: 0,
            phase: ThreePhases::First,
        }
        .into(),
        "00aa0011" => IncrementDoubleRegister {
            destination: DoubleRegister::try_from(a)
                .expect("2 bit value should always correspond to a double register"),
            phase: TwoPhases::First,
        }
        .into(),
        "00aa1011" => DecrementDoubleRegister {
            destination: DoubleRegister::try_from(a)
                .expect("2 bit value should always correspond to a double register"),
            phase: TwoPhases::First,
        }
        .into(),
        "00aa1001" => AddDoubleRegisterToHl {
            operand: DoubleRegister::try_from(a)
                .expect("2 bit value should always correspond to a double register"),
            phase: TwoPhases::First,
        }
        .into(),
        "00000111" => RotateAccumulatorLeft {}.into(),
        "00010111" => RotateAccumulatorLeftThroughCarry {}.into(),
        "00001111" => RotateAccumulatorRight {}.into(),
        "00011111" => RotateAccumulatorRightThroughCarry {}.into(),
        "11010011" => HaltAndCatchFire { opcode: byte }.into(),
        "11011011" => HaltAndCatchFire { opcode: byte }.into(),
        "11011101" => HaltAndCatchFire { opcode: byte }.into(),
        "11100011" => HaltAndCatchFire { opcode: byte }.into(),
        "11100100" => HaltAndCatchFire { opcode: byte }.into(),
        "11101011" => HaltAndCatchFire { opcode: byte }.into(),
        "11101100" => HaltAndCatchFire { opcode: byte }.into(),
        "11101101" => HaltAndCatchFire { opcode: byte }.into(),
        "11110100" => HaltAndCatchFire { opcode: byte }.into(),
        "11111100" => HaltAndCatchFire { opcode: byte }.into(),
        "11111101" => HaltAndCatchFire { opcode: byte }.into(),
        _ => LoadFromHlToRegister {
            destination: Register::A,
            phase: TwoPhases::First,
        }
        .into(),
    }
}

#[cfg(test)]
mod tests {
    use super::decode;
    use crate::cpu::{
        instruction::{
            load_from_hl_to_register::LoadFromHlToRegister,
            load_from_register_to_register::LoadFromRegisterToRegister,
            load_immediate_to_register::LoadImmediateToRegister, InstructionEnum,
            LoadAccumulatorToHlAndDecrement, LoadAccumulatorToHlAndIncrement,
            LoadAccumulatorToImmediateOffset, LoadFromImmediateOffsetToAccumulator,
            LoadHlToAccumulatorAndDecrement, LoadHlToAccumulatorAndIncrement,
        },
        Register,
    };

    #[test]
    fn decode_load_from_register_to_register() {
        let load_a_to_c = 0b01111001u8;
        let instruction = decode(load_a_to_c);
        assert!(matches!(
            instruction,
            InstructionEnum::LoadFromRegisterToRegister(LoadFromRegisterToRegister {
                source: Register::A,
                destination: Register::C
            })
        ))
    }

    #[test]
    fn decode_load_from_hl_to_register() {
        let load_a_to_c = 0b01111110u8;
        let instruction = decode(load_a_to_c);
        assert!(matches!(
            instruction,
            InstructionEnum::LoadFromHlToRegister(LoadFromHlToRegister {
                destination: Register::A,
                phase: _
            })
        ))
    }

    #[test]
    fn decode_load_immediate_to_register() {
        let load_a_to_c = 0b00111110u8;
        let instruction = decode(load_a_to_c);
        assert!(matches!(
            instruction,
            InstructionEnum::LoadImmediateToRegister(LoadImmediateToRegister {
                destination: Register::A,
                value: _,
                phase: _
            })
        ))
    }

    #[test]
    fn decode_load_from_immediate_offset_to_accumulator() {
        let opcode = 0b11110000u8;
        let instruction = decode(opcode);
        assert!(matches!(
            instruction,
            InstructionEnum::LoadFromImmediateOffsetToAccumulator(
                LoadFromImmediateOffsetToAccumulator {
                    phase: _,
                    offset: _
                }
            )
        ))
    }

    #[test]
    fn decode_load_accumulator_to_immediate_offset() {
        let opcode = 0b11100000u8;
        let instruction = decode(opcode);
        assert!(matches!(
            instruction,
            InstructionEnum::LoadAccumulatorToImmediateOffset(LoadAccumulatorToImmediateOffset {
                phase: _,
                offset: _
            })
        ))
    }

    #[test]
    fn decode_load_hl_to_accumulator_and_decrement() {
        let opcode = 0b00111010u8;
        let instruction = decode(opcode);
        assert!(matches!(
            instruction,
            InstructionEnum::LoadHlToAccumulatorAndDecrement(LoadHlToAccumulatorAndDecrement {
                phase: _
            })
        ))
    }

    #[test]
    fn decode_load_accumulator_to_hl_and_decrement() {
        let opcode = 0b00110010u8;
        let instruction = decode(opcode);
        assert!(matches!(
            instruction,
            InstructionEnum::LoadAccumulatorToHlAndDecrement(LoadAccumulatorToHlAndDecrement {
                phase: _
            })
        ))
    }

    #[test]
    fn decode_load_hl_to_accumulator_and_increment() {
        let opcode = 0b00101010u8;
        let instruction = decode(opcode);
        assert!(matches!(
            instruction,
            InstructionEnum::LoadHlToAccumulatorAndIncrement(LoadHlToAccumulatorAndIncrement {
                phase: _
            })
        ))
    }
    #[test]
    fn decode_load_accumulator_to_hl_and_increment() {
        let opcode = 0b00100010u8;
        let instruction = decode(opcode);
        assert!(matches!(
            instruction,
            InstructionEnum::LoadAccumulatorToHlAndIncrement(LoadAccumulatorToHlAndIncrement {
                phase: _
            })
        ))
    }
}
