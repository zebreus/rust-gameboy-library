use crate::cpu::instruction::phases::{ThreePhases, TwoPhases};
use crate::cpu::{DoubleRegister, Register};
use bitmatch::bitmatch;

use super::phases::FourPhases;
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
    LoadAccumulatorToDoubleRegister, LoadAccumulatorToImmediateAddress,
    LoadAccumulatorToRegisterCOffset, LoadFromDoubleRegisterToAccumulator,
    LoadFromImmediateAddressToAccumulator, LoadFromRegisterCOffsetToAccumulator, LoadImmediateToHl,
    LoadRegisterToHl,
};

/// Create a instruction from an opcode.
///
/// Some instructions are longer than one byte because they have immediate arguments. The additional arguments are loaded here. Instead they are loaded in the appropriate cycles when executing the instructions.
///
/// # Examples
///
/// ```
/// # use rust_gameboy_library::cpu::Register;
/// # use rust_gameboy_library::cpu::instruction::InstructionEnum;
/// # use rust_gameboy_library::cpu::instruction::LoadFromRegisterToRegister;
/// # use rust_gameboy_library::cpu::instruction::decode;
/// #
/// let load_a_to_c = 0b01000010u8;
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
        let load_a_to_c = 0b01000010u8;
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
        let load_a_to_c = 0b01000110u8;
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
        let load_a_to_c = 0b00000110u8;
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
