use super::{phases::TwoPhases, Instruction};
use crate::{
    cpu::{Cpu, DoubleRegister, Register},
    memory::MemoryDevice,
};

/// Loads from the address specified in the double register into the [accumulator](Register::A).
#[doc(alias = "LD")]
#[doc(alias = "LD A,(BC)")]
#[doc(alias = "LD A,(DE)")]
pub struct LoadFromDoubleRegisterToAccumulator {
    /// The double register containing the address
    pub address_register: DoubleRegister,
    /// The current phase of the instruction.
    pub phase: TwoPhases,
}

impl Instruction for LoadFromDoubleRegisterToAccumulator {
    fn execute<T: MemoryDevice>(
        &self,
        cpu: &mut crate::cpu::CpuState,
        memory: &mut T,
    ) -> super::InstructionEnum {
        match self.phase {
            TwoPhases::First => {
                let address = cpu.read_double_register(self.address_register);
                let data = memory.read(address);
                cpu.write_register(Register::A, data);

                Self {
                    address_register: self.address_register,
                    phase: TwoPhases::Second,
                }
                .into()
            }
            TwoPhases::Second => cpu.load_instruction(memory),
        }
    }
    fn encode(&self) -> Vec<u8> {
        match self.address_register {
            DoubleRegister::BC => Vec::from([0b00001010]),
            DoubleRegister::DE => Vec::from([0b00011010]),
            _ => panic!(
                "Cannot only encode LoadFromDoubleRegisterToAccumulator for DoubleRegisters BC and DE"
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LoadFromDoubleRegisterToAccumulator;
    use crate::cpu::instruction::phases::TwoPhases;
    use crate::cpu::instruction::{Instruction, InstructionEnum};
    use crate::cpu::{Cpu, CpuState, DoubleRegister, Register};
    use crate::debug_memory::DebugMemory;
    use crate::memory::MemoryDevice;

    #[test]
    fn load_from_double_register_to_accumulator_works() {
        let mut cpu = CpuState::new();
        let mut memory = DebugMemory::new_with_init(&[0, 0, 0, 42]);

        let instruction = LoadFromDoubleRegisterToAccumulator {
            address_register: DoubleRegister::BC,
            phase: TwoPhases::First,
        };

        cpu.write_double_register(DoubleRegister::BC, 3);

        let instruction = instruction.execute(&mut cpu, &mut memory);

        assert!(matches!(
            instruction,
            InstructionEnum::LoadFromDoubleRegisterToAccumulator(
                LoadFromDoubleRegisterToAccumulator {
                    address_register: DoubleRegister::BC,
                    phase: TwoPhases::Second,
                }
            )
        ));

        instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cpu.read_double_register(DoubleRegister::BC), 3);
        assert_eq!(memory.read(3), 42);
        assert_eq!(cpu.read_register(Register::A), 42);
    }
}
