use super::{phases::TwoPhases, Instruction};
use crate::{
    cpu::{Cpu, Register},
    memory::MemoryDevice,
};

/// Stores the value of the [accumulator](Register::A) to memory at `0xff00 + the value of [Register::C]` .
#[doc(alias = "LD")]
#[doc(alias = "LD (C),A")]
#[doc(alias = "LD ($FF00+C),A")]
#[derive(Debug)]
pub struct LoadAccumulatorToRegisterCOffset {
    /// The current phase of the instruction.
    pub phase: TwoPhases,
}

impl Instruction for LoadAccumulatorToRegisterCOffset {
    fn execute<T: MemoryDevice>(
        &self,
        cpu: &mut crate::cpu::CpuState,
        memory: &mut T,
    ) -> super::InstructionEnum {
        match self.phase {
            TwoPhases::First => {
                let offset_16: u16 = cpu.read_register(Register::C).into();
                let address: u16 = 0xff00u16 | offset_16;
                let data = cpu.read_register(Register::A);
                memory.write(address, data);

                Self {
                    phase: TwoPhases::Second,
                }
                .into()
            }
            TwoPhases::Second => cpu.load_instruction(memory),
        }
    }
    fn encode(&self) -> Vec<u8> {
        Vec::from([0b11100010])
    }
}

#[cfg(test)]
mod tests {
    use super::LoadAccumulatorToRegisterCOffset;
    use crate::cpu::instruction::phases::TwoPhases;
    use crate::cpu::instruction::{Instruction, InstructionEnum};
    use crate::cpu::{Cpu, CpuState, Register};
    use crate::memory::Memory;
    use crate::memory::MemoryDevice;

    #[test]
    fn load_accumulator_to_register_c_offset_works() {
        // Write 42 to A and then copy A to C
        let mut cpu = CpuState::new();
        let mut memory = Memory::new_with_init(&[03]);
        cpu.write_register(Register::A, 42);
        cpu.write_register(Register::C, 3);

        let instruction = LoadAccumulatorToRegisterCOffset {
            phase: TwoPhases::First,
        };

        let instruction = instruction.execute(&mut cpu, &mut memory);

        assert!(matches!(
            instruction,
            InstructionEnum::LoadAccumulatorToRegisterCOffset(LoadAccumulatorToRegisterCOffset {
                phase: TwoPhases::Second,
            })
        ));

        assert_eq!(memory.read(0xFF03), 42);
    }
}
