use super::phases::FourPhases;
use super::Instruction;
use crate::{
    cpu::{ConditionCode, Cpu},
    memory::MemoryDevice,
};

/// If the condition is met it jumps to the address specified in the two bytes following the opcode
///
/// This instruction skips the fourth phase, if the condition is not met in the third phase.
///
/// The condition is evaluated in the third phase
#[doc(alias = "JP")]
pub struct JumpToImmediateAddressConditional {
    /// The jump is only made if the condition is fullfilled in the third phase.
    pub condition: ConditionCode,
    /// The immediate address. Will only valid after the second phase.
    pub address: u16,
    /// The current phase of the instruction.
    pub phase: FourPhases,
}

impl Instruction for JumpToImmediateAddressConditional {
    fn execute<T: MemoryDevice>(
        &self,
        cpu: &mut crate::cpu::CpuState,
        memory: &mut T,
    ) -> super::InstructionEnum {
        match self.phase {
            FourPhases::First => {
                let program_counter = cpu.advance_program_counter();
                let address_lsb = memory.read(program_counter);

                Self {
                    condition: self.condition,
                    phase: FourPhases::Second,
                    address: address_lsb as u16,
                }
                .into()
            }
            FourPhases::Second => {
                let program_counter = cpu.advance_program_counter();
                let address_msb = memory.read(program_counter);

                Self {
                    condition: self.condition,
                    phase: FourPhases::Third,
                    address: u16::from_le_bytes([self.address as u8, address_msb]),
                }
                .into()
            }
            FourPhases::Third => {
                let condition_fullfilled = cpu.check_condition(self.condition);
                if !condition_fullfilled {
                    return cpu.load_instruction(memory);
                }

                cpu.write_program_counter(self.address);

                Self {
                    condition: self.condition,
                    phase: FourPhases::Fourth,
                    address: self.address,
                }
                .into()
            }
            FourPhases::Fourth => {
                return cpu.load_instruction(memory);
            }
        }
    }
    fn encode(&self) -> Vec<u8> {
        let condition_code_part = ((self.condition as u8) << 3) & 0b00011000;
        let opcode = 0b11000010 | condition_code_part;

        match self.phase {
            FourPhases::First => Vec::from([opcode]),
            FourPhases::Second => Vec::from([opcode, self.address.to_le_bytes()[0]]),
            _ => Vec::from([
                opcode,
                self.address.to_le_bytes()[0],
                self.address.to_le_bytes()[1],
            ]),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::JumpToImmediateAddressConditional;
    use crate::cpu::instruction::phases::FourPhases;
    use crate::cpu::instruction::{Instruction, InstructionEnum};
    use crate::cpu::{Cpu, CpuState, Flag};
    use crate::memory::Memory;

    #[test]
    fn jump_by_immediate_address_conditional_jumps_when_it_should() {
        // Write 42 to A and then copy A to C
        let mut cpu = CpuState::new();
        let mut memory = Memory::new_with_init(&[0x34, 0x12]);

        cpu.write_flag(Flag::Carry, true);

        let instruction = JumpToImmediateAddressConditional {
            condition: crate::cpu::ConditionCode::CarryFlagSet,
            address: 0,
            phase: FourPhases::First,
        };

        let instruction = instruction.execute(&mut cpu, &mut memory);
        let instruction = instruction.execute(&mut cpu, &mut memory);

        assert!(matches!(
            instruction,
            InstructionEnum::JumpToImmediateAddressConditional(JumpToImmediateAddressConditional {
                condition: crate::cpu::ConditionCode::CarryFlagSet,
                phase: FourPhases::Third,
                address: 0x1234
            })
        ));

        let instruction = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cpu.read_program_counter(), 0x1234);

        assert!(matches!(
            instruction,
            InstructionEnum::JumpToImmediateAddressConditional(JumpToImmediateAddressConditional {
                condition: crate::cpu::ConditionCode::CarryFlagSet,
                phase: FourPhases::Fourth,
                address: 0x1234
            })
        ));
    }

    #[test]
    fn jump_by_immediate_address_conditional_does_not_jump_when_the_condition_is_not_met() {
        // Write 42 to A and then copy A to C
        let mut cpu = CpuState::new();
        let mut memory = Memory::new_with_init(&[0x34, 0x12]);

        let initial_pc = cpu.read_program_counter();

        cpu.write_flag(Flag::Carry, true);

        let instruction = JumpToImmediateAddressConditional {
            condition: crate::cpu::ConditionCode::CarryFlagUnset,
            address: 0,
            phase: FourPhases::First,
        };

        let instruction = instruction.execute(&mut cpu, &mut memory);
        let instruction = instruction.execute(&mut cpu, &mut memory);

        assert!(matches!(
            instruction,
            InstructionEnum::JumpToImmediateAddressConditional(JumpToImmediateAddressConditional {
                condition: crate::cpu::ConditionCode::CarryFlagUnset,
                phase: FourPhases::Third,
                address: 0x1234
            })
        ));

        let instruction = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cpu.read_program_counter(), initial_pc + 3);

        assert!(!matches!(
            instruction,
            InstructionEnum::JumpToImmediateAddressConditional(_)
        ));
    }

    #[test]
    fn encode_jump_by_immediate_address_conditional() {
        // Write 42 to A and then copy A to C
        let mut cpu = CpuState::new();
        let mut memory = Memory::new_with_init(&[0x34, 0x12]);

        let instruction = JumpToImmediateAddressConditional {
            condition: crate::cpu::ConditionCode::ZeroFlagSet,
            address: 0,
            phase: FourPhases::First,
        };

        let instruction = instruction.execute(&mut cpu, &mut memory);
        let instruction = instruction.execute(&mut cpu, &mut memory);

        let encoded = instruction.encode();

        assert_eq!(encoded[0], 0b11001010);
        assert_eq!(encoded[1], 0x34);
        assert_eq!(encoded[2], 0x12);
    }
}
