use bitmatch::bitmatch;

use super::Instruction;
use super::Register;
use super::TwoPhases;

#[bitmatch]
pub fn decode(byte: u8) -> Instruction {
    #[bitmatch]
    // TODO: How can we get rid of this (soon) massive match clause
    match byte {
        "01aaa110" => Instruction::LoadFromHlToRegister {
            destination: Register::try_from(a)
                .expect("3 bit value should always correspond to a register"),
            phase: 0,
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
            phase: 0,
        },
        "00100010" => Instruction::LoadAccumulatorToHlAndIncrement {
            phase: TwoPhases::One,
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
    fn decode_load_accumulator_to_hl_and_increment() {
        let opcode = 0b00100010u8;
        let instruction = decode(opcode);
        assert!(matches!(
            instruction,
            Instruction::LoadAccumulatorToHlAndIncrement { phase: _ }
        ))
    }
}
