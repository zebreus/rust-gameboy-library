use super::phases::TwoPhases;
use super::Instruction;
use crate::{
    cpu::{Cpu, Register},
    memory_device::MemoryDevice,
};

/// Loads the byte following the opcode of the instruction to a register
#[doc(alias = "LD")]
#[doc(alias = "LD R,n")]
pub struct LoadImmediateToRegister {
    /// The destination register.
    pub destination: Register,
    /// The immediate value. Will only valid in the second phase.
    pub value: u8,
    /// The current phase of the instruction.
    pub phase: TwoPhases,
}

impl Instruction for LoadImmediateToRegister {
    fn execute<T: MemoryDevice>(
        &self,
        cpu: &mut crate::cpu::CpuState,
        memory: &mut T,
    ) -> super::InstructionEnum {
        match self.phase {
            TwoPhases::First => {
                let address = cpu.advance_program_counter();
                let value = memory.read(address);
                return Self {
                    destination: self.destination,
                    value,
                    phase: TwoPhases::Second,
                }
                .into();
            }
            TwoPhases::Second => {
                cpu.write_register(self.destination, self.value);
                return cpu.load_instruction(memory);
            }
        }
    }
    fn encode(&self) -> Vec<u8> {
        if matches!(self.destination, Register::F) {
            panic!("Cannot encode load immediate to register for destination register Register::F")
        }
        let base_code = 0b00000110 & 0b11000111u8;
        let destination_code = (self.destination.id() << 3) & 0b00111000u8;
        let opcode = base_code | destination_code;
        match self.phase {
            TwoPhases::First => Vec::from([opcode]),
            TwoPhases::Second => Vec::from([opcode, self.value]),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LoadImmediateToRegister;
    use crate::cpu::instruction::phases::TwoPhases;
    use crate::cpu::instruction::{Instruction, InstructionEnum};
    use crate::cpu::{Cpu, CpuState, Register};
    use crate::debug_memory::DebugMemory;

    #[test]
    fn load_immediate_to_register_works() {
        // Write 42 to A and then copy A to C
        let mut cpu = CpuState::new();
        let mut memory = DebugMemory::new_with_init(&[42]);

        let instruction = LoadImmediateToRegister {
            destination: Register::B,
            value: 0,
            phase: TwoPhases::First,
        };

        let instruction = instruction.execute(&mut cpu, &mut memory);

        assert!(matches!(
            instruction,
            InstructionEnum::LoadImmediateToRegister(LoadImmediateToRegister {
                phase: TwoPhases::Second,
                destination: Register::B,
                value: 42
            })
        ));

        instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cpu.read_register(Register::B), 42);
    }
}
