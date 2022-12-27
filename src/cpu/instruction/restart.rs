use super::phases::FourPhases;
use super::Instruction;
use crate::{
    cpu::{Cpu, RestartAddress},
    memory::MemoryDevice,
};

/// Jump to the specified [RestartAddress]. Writes the program counter before the jump onto the stack.
///
/// See [PushDoubleRegister](super::PushDoubleRegister) for more details on how data is pushed to the stack.
///
/// The value pushed to the stack points to the next instruction directly after this one.
#[doc(alias = "RST")]
pub struct Restart {
    /// The immediate address. Will only valid after the second phase.
    pub address: RestartAddress,
    /// The current phase of the instruction.
    pub phase: FourPhases,
}

impl Instruction for Restart {
    fn execute<T: MemoryDevice>(
        &self,
        cpu: &mut crate::cpu::CpuState,
        memory: &mut T,
    ) -> super::InstructionEnum {
        match self.phase {
            FourPhases::First => {
                cpu.write_stack_pointer(cpu.read_stack_pointer() - 1);

                Self {
                    phase: FourPhases::Second,
                    address: self.address,
                }
                .into()
            }
            FourPhases::Second => {
                let data = cpu.read_program_counter().to_le_bytes()[1];
                memory.write(cpu.read_stack_pointer(), data);

                cpu.write_stack_pointer(cpu.read_stack_pointer() - 1);

                Self {
                    phase: FourPhases::Third,
                    address: self.address,
                }
                .into()
            }
            FourPhases::Third => {
                let data = cpu.read_program_counter().to_le_bytes()[0];
                memory.write(cpu.read_stack_pointer(), data);

                cpu.write_program_counter(self.address.get_address());
                Self {
                    phase: FourPhases::Fourth,
                    address: self.address,
                }
                .into()
            }
            FourPhases::Fourth => {
                return cpu.load_instruction(memory);
            }
        }
    }
    fn encode(&self) -> Vec<u8> {
        let base_code = 0b11000111;
        let address_code: u8 = Into::<u8>::into(self.address) << 3;
        let opcode = base_code | address_code;
        Vec::from([opcode])
    }
}

#[cfg(test)]
mod tests {
    use super::Restart;
    use crate::cpu::instruction::phases::FourPhases;
    use crate::cpu::instruction::{Instruction, InstructionEnum};
    use crate::cpu::{Cpu, CpuState, RestartAddress};
    use crate::memory::Memory;
    use crate::memory::MemoryDevice;

    #[test]
    fn instruction_works() {
        let mut cpu = CpuState::new();
        let mut memory = Memory::new_with_init(&[0x34, 0x12]);
        cpu.write_stack_pointer(0xff00);
        let initial_program_counter = cpu.read_program_counter();

        let instruction = Restart {
            address: RestartAddress::B,
            phase: FourPhases::First,
        };

        let instruction = instruction.execute(&mut cpu, &mut memory);
        let instruction = instruction.execute(&mut cpu, &mut memory);
        let instruction = instruction.execute(&mut cpu, &mut memory);

        assert!(matches!(
            instruction,
            InstructionEnum::Restart(Restart {
                phase: FourPhases::Fourth,
                address: RestartAddress::B
            })
        ));

        assert_eq!(cpu.read_program_counter(), RestartAddress::B.get_address());
        assert_eq!(cpu.read_stack_pointer(), 0xff00 - 2);
        assert_eq!(
            memory.read(cpu.read_stack_pointer()),
            (initial_program_counter).to_le_bytes()[0]
        );
        assert_eq!(
            memory.read(cpu.read_stack_pointer() + 1),
            (initial_program_counter).to_le_bytes()[1]
        );
    }
}
