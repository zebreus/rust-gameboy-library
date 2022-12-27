use super::{phases::TwoPhases, Instruction};
use crate::{
    cpu::{Cpu, DoubleRegister},
    memory::MemoryDevice,
};

/// Copies the data stored in [DoubleRegister::HL] to the stackpointer register
#[doc(alias = "LD")]
#[doc(alias = "LD SP,HL")]
pub struct LoadHlToSp {
    /// The current phase of the instruction.
    pub phase: TwoPhases,
}

impl Instruction for LoadHlToSp {
    fn execute<T: MemoryDevice>(
        &self,
        cpu: &mut crate::cpu::CpuState,
        memory: &mut T,
    ) -> super::InstructionEnum {
        match self.phase {
            TwoPhases::First => {
                let data = cpu.read_double_register(DoubleRegister::HL);
                cpu.write_stack_pointer(data);

                Self {
                    phase: TwoPhases::Second,
                }
                .into()
            }
            TwoPhases::Second => cpu.load_instruction(memory),
        }
    }
    fn encode(&self) -> Vec<u8> {
        Vec::from([0b11111001])
    }
}

#[cfg(test)]
mod tests {
    use super::LoadHlToSp;
    use crate::cpu::instruction::phases::TwoPhases;
    use crate::cpu::instruction::{Instruction, InstructionEnum};
    use crate::cpu::{Cpu, CpuState, DoubleRegister};
    use crate::debug_memory::DebugMemory;
    #[test]
    fn load_hl_to_sp_works() {
        let mut cpu = CpuState::new();
        let mut memory = DebugMemory::new();
        cpu.write_double_register(DoubleRegister::HL, 0x1234);

        let instruction = LoadHlToSp {
            phase: TwoPhases::First,
        };

        let instruction = instruction.execute(&mut cpu, &mut memory);

        assert!(matches!(
            instruction,
            InstructionEnum::LoadHlToSp(LoadHlToSp {
                phase: TwoPhases::Second,
            })
        ));

        assert_eq!(cpu.read_stack_pointer(), 0x1234);
    }
}
