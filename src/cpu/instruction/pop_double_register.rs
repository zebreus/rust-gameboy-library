use super::phases::ThreePhases;
use super::Instruction;
use crate::cpu::DoubleRegister;
use crate::{cpu::Cpu, memory::MemoryDevice};

/// Loads from the address stored in the stack pointer to a double register. Increments the stackpointer twice.
///
/// | SP  | SP + 1 |  SP + 2   |
/// |-----|--------|-----------|
/// | LSB |  MSB   | unchanged |
#[doc(alias = "POP")]
#[doc(alias = "POP BC")]
#[doc(alias = "POP DE")]
#[doc(alias = "POP HL")]
#[doc(alias = "POP AF")]
#[derive(Debug)]
pub struct PopDoubleRegister {
    /// The destination double register.
    pub destination: DoubleRegister,
    /// The current phase of the instruction.
    pub phase: ThreePhases,
}

impl Instruction for PopDoubleRegister {
    fn execute<T: MemoryDevice>(
        &self,
        cpu: &mut crate::cpu::CpuState,
        memory: &mut T,
    ) -> super::InstructionEnum {
        match self.phase {
            ThreePhases::First => {
                let data = memory.read(cpu.read_stack_pointer());
                cpu.write_register(self.destination.id().lsb, data);
                cpu.write_stack_pointer(cpu.read_stack_pointer() + 1);

                Self {
                    destination: self.destination,
                    phase: ThreePhases::Second,
                }
                .into()
            }
            ThreePhases::Second => {
                let data = memory.read(cpu.read_stack_pointer());
                cpu.write_register(self.destination.id().msb, data);
                cpu.write_stack_pointer(cpu.read_stack_pointer() + 1);

                Self {
                    destination: self.destination,
                    phase: ThreePhases::Third,
                }
                .into()
            }
            ThreePhases::Third => {
                return cpu.load_instruction(memory);
            }
        }
    }
    fn encode(&self) -> Vec<u8> {
        let register_part = self.destination.numerical_id() << 4;
        let opcode = 0b11000001 | register_part;
        Vec::from([opcode])
    }
}

#[cfg(test)]
mod tests {
    use super::PopDoubleRegister;
    use crate::cpu::instruction::phases::ThreePhases;
    use crate::cpu::instruction::{Instruction, InstructionEnum};
    use crate::cpu::{Cpu, CpuState, DoubleRegister};
    use crate::memory::Memory;
    use crate::memory::MemoryDevice;

    #[test]
    fn pop_double_register_works() {
        // Write 42 to A and then copy A to C
        let mut cpu = CpuState::new();
        let mut memory = Memory::new_for_tests();

        cpu.write_stack_pointer(0x1234 - 2);
        memory.write(0x1234 - 2, 0x34);
        memory.write(0x1234 - 1, 0x12);

        let instruction = PopDoubleRegister {
            destination: DoubleRegister::BC,
            phase: ThreePhases::First,
        };

        let instruction = instruction.execute(&mut cpu, &mut memory);
        let instruction = instruction.execute(&mut cpu, &mut memory);

        assert!(matches!(
            instruction,
            InstructionEnum::PopDoubleRegister(PopDoubleRegister {
                phase: ThreePhases::Third,
                destination: DoubleRegister::BC,
            })
        ));

        assert_eq!(cpu.read_stack_pointer(), 0x1234);
        assert_eq!(cpu.read_double_register(DoubleRegister::BC), 0x1234);
        assert_eq!(memory.read(0x1234 - 2), 0x34);
        assert_eq!(memory.read(0x1234 - 1), 0x12);
    }
}
