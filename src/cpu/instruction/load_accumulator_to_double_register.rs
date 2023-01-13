use super::{phases::TwoPhases, Instruction};
use crate::{
    cpu::{Cpu, DoubleRegister, Register},
    memory::MemoryDevice,
};

/// Stores the [accumulator](Register::A) to the address specified in the double register.
#[doc(alias = "LD")]
#[doc(alias = "LD (BC),A")]
#[doc(alias = "LD (DE),A")]
#[derive(Debug)]
pub struct LoadAccumulatorToDoubleRegister {
    /// The double register containing the address
    pub address_register: DoubleRegister,
    /// The current phase of the instruction.
    pub phase: TwoPhases,
}

impl Instruction for LoadAccumulatorToDoubleRegister {
    fn execute<T: MemoryDevice>(
        &self,
        cpu: &mut crate::cpu::CpuState,
        memory: &mut T,
    ) -> super::InstructionEnum {
        match self.phase {
            TwoPhases::First => {
                let address = cpu.read_double_register(self.address_register);
                let data = cpu.read_register(Register::A);
                memory.write(address, data);

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
            DoubleRegister::BC => Vec::from([0b00000010]),
            DoubleRegister::DE => Vec::from([0b00010010]),
            _ => panic!(
                "Cannot only encode LoadAccumulatorToDoubleRegister for DoubleRegisters BC and DE"
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LoadAccumulatorToDoubleRegister;
    use crate::cpu::instruction::phases::TwoPhases;
    use crate::cpu::instruction::{Instruction, InstructionEnum};
    use crate::cpu::{Cpu, CpuState, DoubleRegister, Register};
    use crate::memory::MemoryController;
    use crate::memory::MemoryDevice;

    #[test]
    fn load_accumulator_to_double_register_works() {
        let mut cpu = CpuState::new();
        let mut memory = MemoryController::new_with_init(&[]);

        let instruction = LoadAccumulatorToDoubleRegister {
            address_register: DoubleRegister::BC,
            phase: TwoPhases::First,
        };

        cpu.write_register(Register::A, 42);
        cpu.write_double_register(DoubleRegister::BC, 3);

        let instruction = instruction.execute(&mut cpu, &mut memory);

        assert!(matches!(
            instruction,
            InstructionEnum::LoadAccumulatorToDoubleRegister(LoadAccumulatorToDoubleRegister {
                address_register: DoubleRegister::BC,
                phase: TwoPhases::Second,
            })
        ));

        instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cpu.read_double_register(DoubleRegister::BC), 3);
        assert_eq!(memory.read(3), 42);
        assert_eq!(cpu.read_register(Register::A), 42);
    }
}
