use super::{phases::TwoPhases, Instruction};
use crate::{
    cpu::{Cpu, DoubleRegister, Register},
    memory::MemoryDevice,
};

/// Stores the data from a register to the address specified in [HL](DoubleRegister::HL).
#[doc(alias = "LD")]
#[doc(alias = "LD (HL),R")]
#[derive(Debug)]
pub struct LoadRegisterToHl {
    /// The source register.
    pub source: Register,
    /// The current phase of the instruction.
    pub phase: TwoPhases,
}

impl Instruction for LoadRegisterToHl {
    fn execute<T: MemoryDevice>(
        &self,
        cpu: &mut crate::cpu::CpuState,
        memory: &mut T,
    ) -> super::InstructionEnum {
        match self.phase {
            TwoPhases::First => {
                let address = cpu.read_double_register(DoubleRegister::HL);
                let data = cpu.read_register(self.source);
                memory.write(address, data);

                Self {
                    source: self.source,
                    phase: TwoPhases::Second,
                }
                .into()
            }
            TwoPhases::Second => cpu.load_instruction(memory),
        }
    }
    fn encode(&self) -> Vec<u8> {
        if matches!(self.source, Register::F) {
            panic!("Cannot encode load register to hl for destination source Register::F")
        }
        let base_code = 0b01110000 & 0b11111000u8;
        let destination_code = self.source.id() & 0b00000111u8;
        let opcode = base_code | destination_code;
        Vec::from([opcode])
    }
}

#[cfg(test)]
mod tests {
    use super::LoadRegisterToHl;
    use crate::cpu::instruction::phases::TwoPhases;
    use crate::cpu::instruction::{Instruction, InstructionEnum};
    use crate::cpu::{Cpu, CpuState, DoubleRegister, Register};
    use crate::memory::MemoryController;
    use crate::memory::MemoryDevice;
    #[test]
    fn load_from_hl_to_register_works() {
        // Write 42 to A and then copy A to C
        let mut cpu = CpuState::new();
        let mut memory = MemoryController::new_for_tests();
        cpu.write_register(Register::B, 42);

        let instruction = LoadRegisterToHl {
            source: Register::B,
            phase: TwoPhases::First,
        };

        cpu.write_double_register(DoubleRegister::HL, 3);

        let instruction = instruction.execute(&mut cpu, &mut memory);

        assert!(matches!(
            instruction,
            InstructionEnum::LoadRegisterToHl(LoadRegisterToHl {
                phase: TwoPhases::Second,
                source: Register::B,
            })
        ));

        instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cpu.read_register(Register::B), 42);
        assert_eq!(memory.read(3), 42);
    }
}
