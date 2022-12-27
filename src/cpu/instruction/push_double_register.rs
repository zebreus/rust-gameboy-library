use super::phases::FourPhases;
use super::Instruction;
use crate::cpu::DoubleRegister;
use crate::{cpu::Cpu, memory::MemoryDevice};

/// Store a double register at the stack pointer. Decrement the stackpointer twice
///
/// | SP - 2 | SP - 1 |    SP     |
/// |--------|--------|-----------|
/// |  LSB   |  MSB   | unchanged |
#[doc(alias = "PUSH")]
#[doc(alias = "PUSH BC")]
#[doc(alias = "PUSH DE")]
#[doc(alias = "PUSH HL")]
#[doc(alias = "PUSH AF")]
pub struct PushDoubleRegister {
    /// The source double register.
    pub source: DoubleRegister,
    /// The current phase of the instruction.
    pub phase: FourPhases,
}

impl Instruction for PushDoubleRegister {
    fn execute<T: MemoryDevice>(
        &self,
        cpu: &mut crate::cpu::CpuState,
        memory: &mut T,
    ) -> super::InstructionEnum {
        match self.phase {
            FourPhases::First => {
                cpu.write_stack_pointer(cpu.read_stack_pointer() - 1);
                Self {
                    source: self.source,
                    phase: FourPhases::Second,
                }
                .into()
            }
            FourPhases::Second => {
                let data = cpu.read_register(self.source.id().msb);
                memory.write(cpu.read_stack_pointer(), data);

                cpu.write_stack_pointer(cpu.read_stack_pointer() - 1);

                Self {
                    source: self.source,
                    phase: FourPhases::Third,
                }
                .into()
            }
            FourPhases::Third => {
                let data = cpu.read_register(self.source.id().lsb);
                memory.write(cpu.read_stack_pointer(), data);

                Self {
                    source: self.source,
                    phase: FourPhases::Fourth,
                }
                .into()
            }
            FourPhases::Fourth => {
                return cpu.load_instruction(memory);
            }
        }
    }
    fn encode(&self) -> Vec<u8> {
        let register_part = self.source.numerical_id() << 4;
        let opcode = 0b11000101 | register_part;
        Vec::from([opcode])
    }
}

#[cfg(test)]
mod tests {
    use super::PushDoubleRegister;
    use crate::cpu::instruction::phases::FourPhases;
    use crate::cpu::instruction::{Instruction, InstructionEnum};
    use crate::cpu::{Cpu, CpuState, DoubleRegister};
    use crate::memory::Memory;
    use crate::memory::MemoryDevice;

    #[test]
    fn push_double_register_works() {
        // Write 42 to A and then copy A to C
        let mut cpu = CpuState::new();
        let mut memory = Memory::new();

        cpu.write_stack_pointer(0x1234);
        cpu.write_double_register(DoubleRegister::BC, 0x1234);

        let instruction = PushDoubleRegister {
            source: DoubleRegister::BC,
            phase: FourPhases::First,
        };

        let instruction = instruction.execute(&mut cpu, &mut memory);
        let instruction = instruction.execute(&mut cpu, &mut memory);
        let instruction = instruction.execute(&mut cpu, &mut memory);

        assert!(matches!(
            instruction,
            InstructionEnum::PushDoubleRegister(PushDoubleRegister {
                phase: FourPhases::Fourth,
                source: DoubleRegister::BC,
            })
        ));

        assert_eq!(cpu.read_stack_pointer(), 0x1234 - 2);
        assert_eq!(cpu.read_double_register(DoubleRegister::BC), 0x1234);
        assert_eq!(memory.read(0x1234 - 2), 0x34);
        assert_eq!(memory.read(0x1234 - 1), 0x12);
    }
}
