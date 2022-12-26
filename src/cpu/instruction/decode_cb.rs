use crate::cpu::instruction::phases::ThreePhases;
use crate::cpu::Register;
use bitmatch::bitmatch;

use super::{HaltAndCatchFire, InstructionEnum};

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

macro_rules! decode_operand_arithmetic_with_bit {
    ($a:ident, $b:ident, $register_instruction:ident, $hl_instruction:ident) => {
        match $a {
            0b00000110 => super::$hl_instruction {
                phase: ThreePhases::First,
                bit: $b.into(),
            }
            .into(),
            _ => super::$register_instruction {
                operand: Register::try_from($a)
                    .expect("3 bit value should always correspond to a register"),
                bit: $b.into(),
            }
            .into(),
        }
    };
}

/// Decode an [InstructionEnum] from the byte following the [PrefixCb](super::PrefixCb) instruction
#[bitmatch]
pub fn decode_cb(byte: u8) -> InstructionEnum {
    #[bitmatch]
    // We probably cannot get rid of this massive match clause
    match byte {
        "00000aaa" => decode_operand_arithmetic!(a, RotateLeftRegister, RotateLeftAtHl),
        "00001aaa" => decode_operand_arithmetic!(a, RotateRightRegister, RotateRightAtHl),
        "00010aaa" => decode_operand_arithmetic!(
            a,
            RotateLeftThroughCarryRegister,
            RotateLeftThroughCarryAtHl
        ),
        "00011aaa" => decode_operand_arithmetic!(
            a,
            RotateRightThroughCarryRegister,
            RotateRightThroughCarryAtHl
        ),
        "00100aaa" => decode_operand_arithmetic!(a, ShiftLeftRegister, ShiftLeftAtHl),
        "00101aaa" => decode_operand_arithmetic!(a, ShiftRightRegister, ShiftRightAtHl),
        "00110aaa" => decode_operand_arithmetic!(a, SwapNibblesRegister, SwapNibblesAtHl),
        "00111aaa" => {
            decode_operand_arithmetic!(a, ShiftRightLogicalRegister, ShiftRightLogicalAtHl)
        }
        "01bbbaaa" => decode_operand_arithmetic_with_bit!(a, b, CheckBitRegister, CheckBitAtHl),
        "10bbbaaa" => decode_operand_arithmetic_with_bit!(a, b, ResetBitRegister, ResetBitAtHl),
        "11bbbaaa" => decode_operand_arithmetic_with_bit!(a, b, SetBitRegister, SetBitAtHl),
        _ => HaltAndCatchFire { opcode: byte }.into(),
    }
}

#[cfg(test)]
mod tests {
    use super::decode_cb;
    use crate::cpu::instruction::{Instruction, InstructionEnum};

    #[test]
    fn all_cp_opcodes_reencode_to_the_same_opcode() {
        let opcodes = 0u8..255u8;
        let instructions = opcodes
            .map(|opcode| decode_cb(opcode))
            .collect::<Vec<InstructionEnum>>();
        for expected_opcode in 0u8..255u8 {
            let decoded_instruction = instructions
                .get(expected_opcode as usize)
                .expect("should always match");

            let reencoded_opcode = *decoded_instruction
                .encode()
                .get(1)
                .expect("cb instruction should always have a opcode length of at least 2");

            assert_eq!(
                expected_opcode, reencoded_opcode,
                "Expected opcode {:#010b}, got opcode {:#010b}",
                expected_opcode, reencoded_opcode
            );
        }
    }

    #[test]
    fn all_cp_opcodes_reencode_with_0xcb_as_first_byte() {
        let opcodes = 0u8..255u8;
        let instructions = opcodes
            .map(|opcode| decode_cb(opcode))
            .collect::<Vec<InstructionEnum>>();
        for expected_opcode in 0u8..255u8 {
            let decoded_instruction = instructions
                .get(expected_opcode as usize)
                .expect("should always match");

            let reencoded_opcode = *decoded_instruction
                .encode()
                .get(0)
                .expect("cb instruction should always have a opcode length of at least 2");

            assert_eq!(
                0xCB, reencoded_opcode,
                "Expected opcode first byte to be 0xCB, got {:#010b}",
                reencoded_opcode
            );
        }
    }
}
