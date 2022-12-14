use super::phases::ThreePhases;
use super::Instruction;
use crate::{cpu::Cpu, memory_device::MemoryDevice};

/// Jumps by a signed offset specified in the byte following the opcode.
pub struct JumpByImmediateOffset {
    /// The immediate offset. Will only valid after the first phase.
    pub offset: i8,
    /// The current phase of the instruction.
    pub phase: ThreePhases,
}

impl Instruction for JumpByImmediateOffset {
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
                    phase: ThreePhases::Second,
                    offset,
                }
                .into()
            }
            ThreePhases::Second => {
                // TODO: Update to wrapping_add_signed once 1.66 releases
                cpu.write_program_counter(
                    cpu.read_program_counter().wrapping_add(self.offset as u16),
                );

                Self {
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
        match self.phase {
            ThreePhases::First => Vec::from([0b00011000]),
            _ => Vec::from([0b00011000, self.offset.to_ne_bytes()[0]]),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::JumpByImmediateOffset;
    use crate::cpu::instruction::phases::ThreePhases;
    use crate::cpu::instruction::{Instruction, InstructionEnum};
    use crate::cpu::{Cpu, CpuState};
    use crate::debug_memory::DebugMemory;

    #[test]
    fn jump_by_immediate_offset_works() {
        let mut cpu = CpuState::new();
        let mut memory = DebugMemory::new_with_init(&[10]);

        let initial_program_counter = cpu.read_program_counter();

        let instruction = JumpByImmediateOffset {
            offset: 0,
            phase: ThreePhases::First,
        };

        let instruction = instruction.execute(&mut cpu, &mut memory);

        assert!(matches!(
            instruction,
            InstructionEnum::JumpByImmediateOffset(JumpByImmediateOffset {
                phase: ThreePhases::Second,
                offset: 10
            })
        ));

        let instruction = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cpu.read_program_counter(), initial_program_counter + 11);

        assert!(matches!(
            instruction,
            InstructionEnum::JumpByImmediateOffset(JumpByImmediateOffset {
                phase: ThreePhases::Third,
                offset: 10
            })
        ));
    }

    #[test]
    fn encode_jump_by_immediate_address() {
        // Write 42 to A and then copy A to C
        let mut cpu = CpuState::new();
        let mut memory = DebugMemory::new_with_init(&[0x34]);

        let instruction = JumpByImmediateOffset {
            offset: 0,
            phase: ThreePhases::First,
        };

        let instruction = instruction.execute(&mut cpu, &mut memory);

        let encoded = instruction.encode();

        assert_eq!(encoded[0], 0b00011000);
        assert_eq!(encoded[1], 0x34);
    }
}
