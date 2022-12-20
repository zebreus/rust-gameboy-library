use crate::cpu::instruction::phases::{ThreePhases, TwoPhases};
use crate::cpu::{ConditionCode, DoubleRegister, Register};
use bitmatch::bitmatch;

use super::phases::{FivePhases, FourPhases, SixPhases};
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
        "00001aaa" => decode_operand_arithmetic!(
            a,
            RotateRightThroughCarryRegister,
            RotateRightThroughCarryAtHl
        ),
        _ => HaltAndCatchFire { opcode: byte }.into(),
    }
}
