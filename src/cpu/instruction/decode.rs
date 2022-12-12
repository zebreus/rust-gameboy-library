use bitmatch::bitmatch;

use super::Instruction;
use super::Register;
use super::ThreePhases;
use super::TwoPhases;

#[bitmatch]
pub fn decode(byte: u8) -> Instruction {
    #[bitmatch]
    // TODO: How can we get rid of this (soon) massive match clause
    match byte {
        "01aaa110" => Instruction::LoadFromHlToRegister {
            destination: Register::try_from(a)
                .expect("3 bit value should always correspond to a register"),
            phase: TwoPhases::First,
        },
        "01aaabbb" => Instruction::LoadFromRegisterToRegister {
            source: Register::try_from(a)
                .expect("3 bit value should always correspond to a register"),
            destination: Register::try_from(b)
                .expect("3 bit value should always correspond to a register"),
        },
        "00aaa110" => Instruction::LoadImmediateToRegister {
            destination: Register::try_from(a)
                .expect("3 bit value should always correspond to a register"),
            value: 0,
            phase: TwoPhases::First,
        },
        "11110000" => Instruction::LoadFromImmediateOffsetToAccumulator {
            offset: 0,
            phase: ThreePhases::First,
        },
        "11100000" => Instruction::LoadAccumulatorToImmediateOffset {
            offset: 0,
            phase: ThreePhases::First,
        },
        "00111010" => Instruction::LoadHlToAccumulatorAndDecrement {
            phase: TwoPhases::First,
        },
        "00110010" => Instruction::LoadAccumulatorToHlAndDecrement {
            phase: TwoPhases::First,
        },
        "00101010" => Instruction::LoadHlToAccumulatorAndIncrement {
            phase: TwoPhases::First,
        },
        "00100010" => Instruction::LoadAccumulatorToHlAndIncrement {
            phase: TwoPhases::First,
        },
        _ => Instruction::None {},
    }
}

#[cfg(test)]
mod tests {
    use super::decode;
    use super::Instruction;
    use crate::cpu::Register;

    #[test]
    fn decode_load_from_register_to_register() {
        let load_a_to_c = 0b01000010u8;
        let instruction = decode(load_a_to_c);
        assert!(matches!(
            instruction,
            Instruction::LoadFromRegisterToRegister {
                source: Register::A,
                destination: Register::C
            }
        ))
    }

    #[test]
    fn decode_load_from_hl_to_register() {
        let load_a_to_c = 0b01000110u8;
        let instruction = decode(load_a_to_c);
        assert!(matches!(
            instruction,
            Instruction::LoadFromHlToRegister {
                destination: Register::A,
                phase: _
            }
        ))
    }

    #[test]
    fn decode_load_immediate_to_register() {
        let load_a_to_c = 0b00000110u8;
        let instruction = decode(load_a_to_c);
        assert!(matches!(
            instruction,
            Instruction::LoadImmediateToRegister {
                destination: Register::A,
                value: _,
                phase: _
            }
        ))
    }

    #[test]
    fn decode_load_from_immediate_offset_to_accumulator() {
        let opcode = 0b11110000u8;
        let instruction = decode(opcode);
        assert!(matches!(
            instruction,
            Instruction::LoadFromImmediateOffsetToAccumulator {
                phase: _,
                offset: _
            }
        ))
    }

    #[test]
    fn decode_load_accumulator_to_immediate_offset() {
        let opcode = 0b11100000u8;
        let instruction = decode(opcode);
        assert!(matches!(
            instruction,
            Instruction::LoadAccumulatorToImmediateOffset {
                phase: _,
                offset: _
            }
        ))
    }

    #[test]
    fn decode_load_hl_to_accumulator_and_decrement() {
        let opcode = 0b00111010u8;
        let instruction = decode(opcode);
        assert!(matches!(
            instruction,
            Instruction::LoadHlToAccumulatorAndDecrement { phase: _ }
        ))
    }

    #[test]
    fn decode_load_accumulator_to_hl_and_decrement() {
        let opcode = 0b00110010u8;
        let instruction = decode(opcode);
        assert!(matches!(
            instruction,
            Instruction::LoadAccumulatorToHlAndDecrement { phase: _ }
        ))
    }

    #[test]
    fn decode_load_hl_to_accumulator_and_increment() {
        let opcode = 0b00101010u8;
        let instruction = decode(opcode);
        assert!(matches!(
            instruction,
            Instruction::LoadHlToAccumulatorAndIncrement { phase: _ }
        ))
    }
    #[test]
    fn decode_load_accumulator_to_hl_and_increment() {
        let opcode = 0b00100010u8;
        let instruction = decode(opcode);
        assert!(matches!(
            instruction,
            Instruction::LoadAccumulatorToHlAndIncrement { phase: _ }
        ))
    }
}
