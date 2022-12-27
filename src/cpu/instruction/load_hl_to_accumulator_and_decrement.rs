use super::{phases::TwoPhases, Instruction};
use crate::{
    cpu::{Cpu, DoubleRegister, Register},
    memory::MemoryDevice,
};

/// Loads from memory at the address specified in [HL](DoubleRegister::HL) to the [accumulator](Register::A). Decrements [HL](DoubleRegister::HL) afterwards.
#[doc(alias = "LD")]
#[doc(alias = "LD A,(HL-)")]
#[doc(alias = "LD A,(HLD)")]
#[doc(alias = "LDD")]
#[doc(alias = "LDD A,(HL)")]
pub struct LoadHlToAccumulatorAndDecrement {
    /// The current phase of the instruction.
    pub phase: TwoPhases,
}

impl Instruction for LoadHlToAccumulatorAndDecrement {
    fn execute<T: MemoryDevice>(
        &self,
        cpu: &mut crate::cpu::CpuState,
        memory: &mut T,
    ) -> super::InstructionEnum {
        match self.phase {
            TwoPhases::First => {
                let address = cpu.read_double_register(DoubleRegister::HL);
                let data = memory.read(address);

                cpu.write_register(Register::A, data);
                cpu.write_double_register(DoubleRegister::HL, address - 1);

                Self {
                    phase: TwoPhases::Second,
                }
                .into()
            }
            TwoPhases::Second => cpu.load_instruction(memory),
        }
    }
    fn encode(&self) -> Vec<u8> {
        Vec::from([0b00111010])
    }
}

#[cfg(test)]
mod tests {
    use super::LoadHlToAccumulatorAndDecrement;
    use crate::cpu::instruction::phases::TwoPhases;
    use crate::cpu::instruction::{Instruction, InstructionEnum};
    use crate::cpu::{Cpu, CpuState, DoubleRegister, Register};
    use crate::debug_memory::DebugMemory;

    #[test]
    fn load_hl_to_accumulator_and_decrement_works() {
        // Write 42 to A and then copy A to C
        let mut cpu = CpuState::new();
        let mut memory = DebugMemory::new_with_init(&[0, 0, 0, 42]);

        let instruction = LoadHlToAccumulatorAndDecrement {
            phase: TwoPhases::First,
        };

        cpu.write_double_register(DoubleRegister::HL, 3);

        let instruction = instruction.execute(&mut cpu, &mut memory);

        assert!(matches!(
            instruction,
            InstructionEnum::LoadHlToAccumulatorAndDecrement(LoadHlToAccumulatorAndDecrement {
                phase: TwoPhases::Second,
            })
        ));

        instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cpu.read_double_register(DoubleRegister::HL), 2);
        assert_eq!(cpu.read_register(Register::A), 42);
    }
}
