use super::phases::ThreePhases;
use super::Instruction;
use crate::{
    cpu::{ConditionCode, Cpu},
    memory::MemoryDevice,
};

/// If the condition is met it jumps by a signed offset specified in the byte following the opcode.
///
/// This instruction skips the third phase, if the condition is not met in the third phase.
///
/// The condition is evaluated in the second phase
#[doc(alias = "JR")]
pub struct JumpByImmediateOffsetConditional {
    /// The jump is only made if the condition is fullfilled in the third phase.
    pub condition: ConditionCode,
    /// The immediate offset. Will only valid after the first phase.
    pub offset: i8,
    /// The current phase of the instruction.
    pub phase: ThreePhases,
}

impl Instruction for JumpByImmediateOffsetConditional {
    fn execute<T: MemoryDevice>(
        &self,
        cpu: &mut crate::cpu::CpuState,
        memory: &mut T,
    ) -> super::InstructionEnum {
        match self.phase {
            ThreePhases::First => {
                let program_counter = cpu.advance_program_counter();
                let offset: i8 = memory.read_signed(program_counter);

                Self {
                    condition: self.condition,
                    phase: ThreePhases::Second,
                    offset,
                }
                .into()
            }
            ThreePhases::Second => {
                let condition_fullfilled = cpu.check_condition(self.condition);
                if !condition_fullfilled {
                    return cpu.load_instruction(memory);
                }

                // TODO: Update to wrapping_add_signed once 1.66 releases
                cpu.write_program_counter(
                    cpu.read_program_counter().wrapping_add(self.offset as u16),
                );

                Self {
                    condition: self.condition,
                    phase: ThreePhases::Third,
                    offset: self.offset,
                }
                .into()
            }
            ThreePhases::Third => {
                return cpu.load_instruction(memory);
            }
        }
    }
    fn encode(&self) -> Vec<u8> {
        let condition_code_part = ((self.condition as u8) << 3) & 0b00011000;
        let opcode = 0b00100000 | condition_code_part;

        match self.phase {
            ThreePhases::First => Vec::from([opcode]),
            _ => Vec::from([opcode, self.offset.to_ne_bytes()[0]]),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::JumpByImmediateOffsetConditional;
    use crate::cpu::instruction::phases::ThreePhases;
    use crate::cpu::instruction::{Instruction, InstructionEnum};
    use crate::cpu::{Cpu, CpuState, Flag};
    use crate::debug_memory::DebugMemory;

    #[test]
    fn jump_by_immediate_offset_conditional_jumps_when_it_should() {
        let mut cpu = CpuState::new();
        let mut memory = DebugMemory::new_with_init(&[10]);

        cpu.write_flag(Flag::Carry, true);
        let initial_program_counter = cpu.read_program_counter();

        let instruction = JumpByImmediateOffsetConditional {
            condition: crate::cpu::ConditionCode::CarryFlagSet,
            offset: 0,
            phase: ThreePhases::First,
        };

        let instruction = instruction.execute(&mut cpu, &mut memory);

        assert!(matches!(
            instruction,
            InstructionEnum::JumpByImmediateOffsetConditional(JumpByImmediateOffsetConditional {
                condition: crate::cpu::ConditionCode::CarryFlagSet,
                phase: ThreePhases::Second,
                offset: 10
            })
        ));

        let instruction = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cpu.read_program_counter(), initial_program_counter + 11);

        assert!(matches!(
            instruction,
            InstructionEnum::JumpByImmediateOffsetConditional(JumpByImmediateOffsetConditional {
                condition: crate::cpu::ConditionCode::CarryFlagSet,
                phase: ThreePhases::Third,
                offset: 10
            })
        ));
    }

    #[test]
    fn jump_by_immediate_offset_conditional_does_not_jump_when_the_condition_is_not_met() {
        let mut cpu = CpuState::new();
        let mut memory = DebugMemory::new_with_init(&[10]);

        cpu.write_flag(Flag::Carry, true);

        let initial_program_counter = cpu.read_program_counter();

        let instruction = JumpByImmediateOffsetConditional {
            condition: crate::cpu::ConditionCode::CarryFlagUnset,
            offset: 0,
            phase: ThreePhases::First,
        };

        let instruction = instruction.execute(&mut cpu, &mut memory);

        assert!(matches!(
            instruction,
            InstructionEnum::JumpByImmediateOffsetConditional(JumpByImmediateOffsetConditional {
                condition: crate::cpu::ConditionCode::CarryFlagUnset,
                phase: ThreePhases::Second,
                offset: 10
            })
        ));

        let instruction = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cpu.read_program_counter(), initial_program_counter + 2);

        assert!(!matches!(
            instruction,
            InstructionEnum::JumpByImmediateOffsetConditional(_)
        ));
    }

    #[test]
    fn encode_jump_by_immediate_offset_conditional() {
        let mut cpu = CpuState::new();
        let mut memory = DebugMemory::new_with_init(&[0x34]);

        let instruction = JumpByImmediateOffsetConditional {
            condition: crate::cpu::ConditionCode::ZeroFlagSet,
            offset: 0,
            phase: ThreePhases::First,
        };

        let instruction = instruction.execute(&mut cpu, &mut memory);

        let encoded = instruction.encode();

        assert_eq!(encoded[0], 0b00101000);
        assert_eq!(encoded[1], 0x34);
    }
}
