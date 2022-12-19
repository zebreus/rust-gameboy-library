use super::{phases::TwoPhases, Instruction};
use crate::{
    cpu::{Cpu, DoubleRegister, Register},
    memory_device::MemoryDevice,
};

/// Stores the [accumulator](Register::A) to the address specified in [HL](DoubleRegister::HL). Increments [HL](DoubleRegister::HL) afterwards.
#[doc(alias = "LD")]
#[doc(alias = "LD (HL+),A")]
#[doc(alias = "LD (HLI),A")]
#[doc(alias = "LDI")]
#[doc(alias = "LDI (HL),A")]
pub struct LoadAccumulatorToHlAndIncrement {
    /// The current phase of the instruction.
    pub phase: TwoPhases,
}

impl Instruction for LoadAccumulatorToHlAndIncrement {
    fn execute<T: MemoryDevice>(
        &self,
        cpu: &mut crate::cpu::CpuState,
        memory: &mut T,
    ) -> super::InstructionEnum {
        match self.phase {
            TwoPhases::First => {
                let address = cpu.read_double_register(DoubleRegister::HL);
                let data = cpu.read_register(Register::A);
                memory.write(address, data);
                cpu.write_double_register(DoubleRegister::HL, address + 1);

                Self {
                    phase: TwoPhases::Second,
                }
                .into()
            }
            TwoPhases::Second => cpu.load_instruction(memory),
        }
    }
    fn encode(&self) -> Vec<u8> {
        Vec::from([0b00100010])
    }
}

#[cfg(test)]
mod tests {
    use super::LoadAccumulatorToHlAndIncrement;
    use crate::cpu::instruction::phases::TwoPhases;
    use crate::cpu::instruction::{Instruction, InstructionEnum};
    use crate::cpu::{Cpu, CpuState, DoubleRegister, Register};
    use crate::debug_memory::DebugMemory;

    #[test]
    fn load_accumulator_to_hl_and_increment_works() {
        // Write 42 to A and then copy A to C
        let mut cpu = CpuState::new();
        let mut memory = DebugMemory::new_with_init(&[]);

        let instruction = LoadAccumulatorToHlAndIncrement {
            phase: TwoPhases::First,
        };

        cpu.write_register(Register::A, 42);
        cpu.write_double_register(DoubleRegister::HL, 3);

        let instruction = instruction.execute(&mut cpu, &mut memory);

        assert!(matches!(
            instruction,
            InstructionEnum::LoadAccumulatorToHlAndIncrement(LoadAccumulatorToHlAndIncrement {
                phase: TwoPhases::Second,
            })
        ));

        instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cpu.read_double_register(DoubleRegister::HL), 4);
        assert_eq!(cpu.read_register(Register::A), 42);
    }
}
