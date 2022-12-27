use super::phases::FivePhases;
use super::Instruction;
use crate::{cpu::Cpu, memory::MemoryDevice};

/// Waits two phases then pushes the program counter to the stack and jumps to the interrupt handler.
///
///  The InterruptServiceRoutine is a special instruction in that it c does not have an opcode.
/// It can will only be created by the cpu itself as a response to an interrupt.
/// Structurally it is quite similar to [Call](super::Call).
///
/// It also disables the interrupt master enable. That needs to be enabled again before the net interrupt can be processed.
///
/// For some reason this is one phase shorter than [Call](super::Call), idk why maybe the docs are wrong.
pub struct InterruptServiceRoutine {
    /// The address of the interrupt handler.
    pub address: u16,
    /// The current phase of the instruction.
    pub phase: FivePhases,
}

impl InterruptServiceRoutine {
    /// Create a new [InterruptServiceRoutine] instruction that calls the interrupt handler at the given address.
    pub fn create(interrupt_handler: u16) -> InterruptServiceRoutine {
        InterruptServiceRoutine {
            address: interrupt_handler,
            phase: FivePhases::First,
        }
    }
}

impl Instruction for InterruptServiceRoutine {
    fn execute<T: MemoryDevice>(
        &self,
        cpu: &mut crate::cpu::CpuState,
        memory: &mut T,
    ) -> super::InstructionEnum {
        match self.phase {
            FivePhases::First => {
                cpu.write_interrupt_master_enable(false);
                Self {
                    address: self.address,
                    phase: FivePhases::Second,
                }
                .into()
            }
            FivePhases::Second => Self {
                address: self.address,
                phase: FivePhases::Third,
            }
            .into(),
            FivePhases::Third => {
                cpu.write_stack_pointer(cpu.read_stack_pointer() - 1);
                let data = cpu.read_program_counter().to_le_bytes()[1];
                memory.write(cpu.read_stack_pointer(), data);

                Self {
                    phase: FivePhases::Fourth,
                    address: self.address,
                }
                .into()
            }
            FivePhases::Fourth => {
                cpu.write_stack_pointer(cpu.read_stack_pointer() - 1);
                let data = cpu.read_program_counter().to_le_bytes()[0];
                memory.write(cpu.read_stack_pointer(), data);

                Self {
                    phase: FivePhases::Fifth,
                    address: self.address,
                }
                .into()
            }
            FivePhases::Fifth => {
                cpu.write_program_counter(self.address);

                return cpu.load_instruction(memory);
            }
        }
    }
    fn encode(&self) -> Vec<u8> {
        panic!("The interrupt service routine does not have an opcode.")
    }
}

#[cfg(test)]
mod tests {
    use super::InterruptServiceRoutine;
    use crate::cpu::instruction::phases::FivePhases;
    use crate::cpu::instruction::{Instruction, InstructionEnum};
    use crate::cpu::{Cpu, CpuState};
    use crate::debug_memory::DebugMemory;
    use crate::memory::MemoryDevice;

    #[test]
    fn interrupt_service_routine_works() {
        let mut cpu = CpuState::new();
        let mut memory = DebugMemory::new_with_init(&[0x34, 0x12]);
        cpu.write_stack_pointer(0xff00);
        let initial_program_counter = cpu.read_program_counter();

        let instruction = InterruptServiceRoutine {
            address: 0x40,
            phase: FivePhases::First,
        };

        let instruction = instruction.execute(&mut cpu, &mut memory);
        let instruction = instruction.execute(&mut cpu, &mut memory);
        let instruction = instruction.execute(&mut cpu, &mut memory);
        let instruction = instruction.execute(&mut cpu, &mut memory);

        assert!(matches!(
            instruction,
            InstructionEnum::InterruptServiceRoutine(InterruptServiceRoutine {
                phase: FivePhases::Fifth,
                address: 0x40
            })
        ));

        instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cpu.read_program_counter(), 0x0041);

        assert_eq!(cpu.read_stack_pointer(), 0xff00 - 2);
        assert_eq!(
            memory.read(cpu.read_stack_pointer()),
            initial_program_counter.to_le_bytes()[0]
        );
        assert_eq!(
            memory.read(cpu.read_stack_pointer() + 1),
            initial_program_counter.to_le_bytes()[1]
        );
    }

    #[test]
    #[should_panic]
    fn encode_interrupt_service_routine() {
        let instruction = InterruptServiceRoutine {
            phase: FivePhases::First,
            address: 0x40,
        };

        instruction.encode();
    }
}
