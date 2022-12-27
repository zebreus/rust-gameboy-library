use super::{phases::TwoPhases, Instruction};
use crate::{
    cpu::{Cpu, DoubleRegister, Register},
    memory::MemoryDevice,
};

/// Loads from memory at the address stored in [DoubleRegister::HL] to a register.
#[doc(alias = "LD")]
#[doc(alias = "LD R,(HL)")]
pub struct LoadFromHlToRegister {
    /// The destination register.
    pub destination: Register,
    /// The current phase of the instruction.
    pub phase: TwoPhases,
}

impl Instruction for LoadFromHlToRegister {
    fn execute<T: MemoryDevice>(
        &self,
        cpu: &mut crate::cpu::CpuState,
        memory: &mut T,
    ) -> super::InstructionEnum {
        match self.phase {
            TwoPhases::First => {
                let address = cpu.read_double_register(DoubleRegister::HL);
                let data = memory.read(address);
                // This should probably happen in the next phase of this instruction
                cpu.write_register(self.destination, data);
                Self {
                    destination: self.destination,
                    phase: TwoPhases::Second,
                }
                .into()
            }
            TwoPhases::Second => cpu.load_instruction(memory),
        }
    }
    fn encode(&self) -> Vec<u8> {
        if matches!(self.destination, Register::F) {
            panic!("Cannot encode load from hl for Register::F")
        }
        let base_code = 0b01000110;
        let destination_code = (self.destination.id() << 3) & 0b00111000u8;
        let opcode = base_code | destination_code;
        Vec::from([opcode])
    }
}

#[cfg(test)]
mod tests {
    use super::LoadFromHlToRegister;
    use crate::cpu::instruction::phases::TwoPhases;
    use crate::cpu::instruction::{Instruction, InstructionEnum};
    use crate::cpu::{Cpu, CpuState, DoubleRegister, Register};
    use crate::memory::Memory;
    #[test]
    fn load_from_hl_to_register_works() {
        // Write 42 to A and then copy A to C
        let mut cpu = CpuState::new();
        let mut memory = Memory::new_with_init(&[0, 0, 0, 42]);

        let instruction = LoadFromHlToRegister {
            destination: Register::B,
            phase: TwoPhases::First,
        };

        cpu.write_double_register(DoubleRegister::HL, 3);

        let instruction = instruction.execute(&mut cpu, &mut memory);

        assert!(matches!(
            instruction,
            InstructionEnum::LoadFromHlToRegister(LoadFromHlToRegister {
                phase: TwoPhases::Second,
                destination: Register::B,
            })
        ));

        instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cpu.read_register(Register::B), 42);
    }
}
