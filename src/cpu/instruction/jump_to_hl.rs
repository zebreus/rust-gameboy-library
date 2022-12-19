use super::phases::TwoPhases;
use super::Instruction;
use crate::{
    cpu::{Cpu, DoubleRegister},
    memory_device::MemoryDevice,
};

/// Jumps to the address stored in [DoubleRegister::HL].
#[doc(alias = "JP")]
pub struct JumpToHl {
    /// The current phase of the instruction.
    pub phase: TwoPhases,
}

impl Instruction for JumpToHl {
    fn execute<T: MemoryDevice>(
        &self,
        cpu: &mut crate::cpu::CpuState,
        memory: &mut T,
    ) -> super::InstructionEnum {
        match self.phase {
            TwoPhases::First => {
                let address = cpu.read_double_register(DoubleRegister::HL);
                cpu.write_program_counter(address);

                Self {
                    phase: TwoPhases::Second,
                }
                .into()
            }
            TwoPhases::Second => {
                return cpu.load_instruction(memory);
            }
        }
    }
    fn encode(&self) -> Vec<u8> {
        Vec::from([0b11101001])
    }
}

#[cfg(test)]
mod tests {
    use super::JumpToHl;
    use crate::cpu::instruction::phases::TwoPhases;
    use crate::cpu::instruction::{Instruction, InstructionEnum};
    use crate::cpu::{Cpu, CpuState, DoubleRegister};
    use crate::debug_memory::DebugMemory;

    #[test]
    fn jump_to_hl_works() {
        let mut cpu = CpuState::new();
        let mut memory = DebugMemory::new();
        cpu.write_double_register(DoubleRegister::HL, 0x1234);

        let instruction = JumpToHl {
            phase: TwoPhases::First,
        };

        let instruction = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cpu.read_program_counter(), 0x1234);

        assert!(matches!(
            instruction,
            InstructionEnum::JumpToHl(JumpToHl {
                phase: TwoPhases::Second,
            })
        ));
    }
}
