use super::{phases::ThreePhases, Instruction};
use crate::{
    cpu::{Cpu, Register},
    memory_device::MemoryDevice,
};

/// Loads from memory at `0xff00 + the byte following the opcode` into the [accumulator](Register::A).
pub struct LoadFromImmediateOffsetToAccumulator {
    /// The memory address offset from 0xff00. Only valid after the first phase.
    pub offset: u8,
    /// The current phase of the instruction.
    pub phase: ThreePhases,
}

impl Instruction for LoadFromImmediateOffsetToAccumulator {
    fn execute<T: MemoryDevice>(
        &self,
        cpu: &mut crate::cpu::CpuState,
        memory: &mut T,
    ) -> super::InstructionEnum {
        match self.phase {
            ThreePhases::First => {
                let next_address = cpu.advance_program_counter();
                let offset = memory.read(next_address);

                Self {
                    phase: ThreePhases::Second,
                    offset: offset,
                }
                .into()
            }
            ThreePhases::Second => {
                let offset_16: u16 = self.offset.into();
                let address: u16 = 0xff00u16 | offset_16;
                let data = memory.read(address);

                cpu.write_register(Register::A, data);

                Self {
                    phase: ThreePhases::Third,
                    offset: self.offset,
                }
                .into()
            }
            ThreePhases::Third => cpu.load_instruction(memory),
        }
    }
    fn encode(&self) -> Vec<u8> {
        match self.phase {
            ThreePhases::First => Vec::from([0b11100000]),
            _ => Vec::from([0b11100000, self.offset]),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LoadFromImmediateOffsetToAccumulator;
    use crate::cpu::instruction::phases::ThreePhases;
    use crate::cpu::instruction::{Instruction, InstructionEnum};
    use crate::cpu::{Cpu, CpuState, Register};
    use crate::debug_memory::DebugMemory;
    use crate::memory_device::MemoryDevice;

    #[test]
    fn load_from_immediate_offset_to_accumulator_works() {
        // Write 42 to A and then copy A to C
        let mut cpu = CpuState::new();
        let mut memory = DebugMemory::new_with_init(&[03]);
        memory.write(0xFF03, 42);

        let instruction = LoadFromImmediateOffsetToAccumulator {
            offset: 0,
            phase: ThreePhases::First,
        };

        let instruction = instruction.execute(&mut cpu, &mut memory);

        assert!(matches!(
            instruction,
            InstructionEnum::LoadFromImmediateOffsetToAccumulator(
                LoadFromImmediateOffsetToAccumulator {
                    offset: 3,
                    phase: ThreePhases::Second,
                }
            )
        ));
        let instruction = instruction.execute(&mut cpu, &mut memory);

        assert!(matches!(
            instruction,
            InstructionEnum::LoadFromImmediateOffsetToAccumulator(
                LoadFromImmediateOffsetToAccumulator {
                    offset: _,
                    phase: ThreePhases::Third,
                }
            )
        ));

        instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cpu.read_register(Register::A), 42);
    }
}
