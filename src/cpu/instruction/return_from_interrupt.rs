use super::phases::FourPhases;
use super::Instruction;
use crate::{cpu::Cpu, memory_device::MemoryDevice};

/// Return from a previous [Call](super::Call) instruction and enable interrupts.
///
/// Sets IME to true.
///
/// Basically this has the same effect as calling [EnableInterrupts](super::EnableInterrupts) and then [Return](super::Return)
#[doc(alias = "RETI")]
pub struct ReturnFromInterrupt {
    /// The current phase of the instruction.
    pub phase: FourPhases,
}

impl Instruction for ReturnFromInterrupt {
    fn execute<T: MemoryDevice>(
        &self,
        cpu: &mut crate::cpu::CpuState,
        memory: &mut T,
    ) -> super::InstructionEnum {
        match self.phase {
            FourPhases::First => {
                let data = memory.read(cpu.read_stack_pointer());
                let new_program_counter =
                    u16::from_le_bytes([data, cpu.read_program_counter().to_le_bytes()[1]]);
                cpu.write_program_counter(new_program_counter);
                cpu.write_stack_pointer(cpu.read_stack_pointer() + 1);

                Self {
                    phase: FourPhases::Second,
                }
                .into()
            }
            FourPhases::Second => {
                let data = memory.read(cpu.read_stack_pointer());
                let new_program_counter =
                    u16::from_le_bytes([cpu.read_program_counter().to_le_bytes()[0], data]);
                cpu.write_program_counter(new_program_counter);
                cpu.write_stack_pointer(cpu.read_stack_pointer() + 1);

                Self {
                    phase: FourPhases::Third,
                }
                .into()
            }
            FourPhases::Third => {
                cpu.write_interrupt_master_enable(true);

                Self {
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
        Vec::from([0b11011001])
    }
}

#[cfg(test)]
mod tests {
    use super::ReturnFromInterrupt;
    use crate::cpu::instruction::phases::FourPhases;
    use crate::cpu::instruction::{Instruction, InstructionEnum};
    use crate::cpu::{Cpu, CpuState};
    use crate::debug_memory::DebugMemory;
    use crate::memory_device::MemoryDevice;

    #[test]
    fn instruction_works() {
        let mut cpu = CpuState::new();
        let mut memory = DebugMemory::new();
        cpu.write_interrupt_master_enable(false);
        cpu.write_stack_pointer(0x1234 - 2);
        memory.write(0x1234 - 2, 0x34);
        memory.write(0x1234 - 1, 0x12);

        let instruction = ReturnFromInterrupt {
            phase: FourPhases::First,
        };

        let instruction = instruction.execute(&mut cpu, &mut memory);
        let instruction = instruction.execute(&mut cpu, &mut memory);
        let instruction = instruction.execute(&mut cpu, &mut memory);

        assert!(matches!(
            instruction,
            InstructionEnum::ReturnFromInterrupt(ReturnFromInterrupt {
                phase: FourPhases::Fourth,
            })
        ));
        assert_eq!(cpu.read_interrupt_master_enable(), true);

        assert_eq!(cpu.read_stack_pointer(), 0x1234);
        assert_eq!(cpu.read_program_counter(), 0x1234);
        assert_eq!(memory.read(0x1234 - 2), 0x34);
        assert_eq!(memory.read(0x1234 - 1), 0x12);
    }
}
