use super::{phases::TwoPhases, Instruction};
use crate::{
    cpu::{Cpu, Register},
    memory::MemoryDevice,
};

/// Loads from memory at `0xff00 + the value of [Register::C]` into the [accumulator](Register::A).
#[doc(alias = "LD")]
#[doc(alias = "LD A,(C)")]
#[doc(alias = "LD A,($FF00+C)")]
pub struct LoadFromRegisterCOffsetToAccumulator {
    /// The current phase of the instruction.
    pub phase: TwoPhases,
}

impl Instruction for LoadFromRegisterCOffsetToAccumulator {
    fn execute<T: MemoryDevice>(
        &self,
        cpu: &mut crate::cpu::CpuState,
        memory: &mut T,
    ) -> super::InstructionEnum {
        match self.phase {
            TwoPhases::First => {
                let offset_16: u16 = cpu.read_register(Register::C).into();
                let address: u16 = 0xff00u16 | offset_16;
                let data = memory.read(address);
                cpu.write_register(Register::A, data);

                Self {
                    phase: TwoPhases::Second,
                }
                .into()
            }
            TwoPhases::Second => cpu.load_instruction(memory),
        }
    }
    fn encode(&self) -> Vec<u8> {
        Vec::from([0b11110010])
    }
}

#[cfg(test)]
mod tests {
    use super::LoadFromRegisterCOffsetToAccumulator;
    use crate::cpu::instruction::phases::TwoPhases;
    use crate::cpu::instruction::{Instruction, InstructionEnum};
    use crate::cpu::{Cpu, CpuState, Register};
    use crate::debug_memory::DebugMemory;
    use crate::memory::MemoryDevice;

    #[test]
    fn load_from_register_c_offset_to_accumulator_works() {
        // Write 42 to A and then copy A to C
        let mut cpu = CpuState::new();
        let mut memory = DebugMemory::new();
        memory.write(0xFF03, 42);
        cpu.write_register(Register::A, 0);
        cpu.write_register(Register::C, 3);

        let instruction = LoadFromRegisterCOffsetToAccumulator {
            phase: TwoPhases::First,
        };

        let instruction = instruction.execute(&mut cpu, &mut memory);

        assert!(matches!(
            instruction,
            InstructionEnum::LoadFromRegisterCOffsetToAccumulator(
                LoadFromRegisterCOffsetToAccumulator {
                    phase: TwoPhases::Second,
                }
            )
        ));

        assert_eq!(memory.read(0xFF03), 42);
        assert_eq!(cpu.read_register(Register::A), 42);
    }
}
