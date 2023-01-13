use super::phases::ThreePhases;
use super::Instruction;
use crate::cpu::DoubleRegister;
use crate::{cpu::Cpu, memory::MemoryDevice};

/// Loads the two bytes following the opcode of the instruction to a double register
///
/// Setting destination to DoubleRegister::AF actually means loading them to the stackpointer
#[doc(alias = "LD")]
#[doc(alias = "LD BC,nn")]
#[doc(alias = "LD DE,nn")]
#[doc(alias = "LD HL,nn")]
#[doc(alias = "LD SP,nn")]
#[derive(Debug)]
pub struct LoadImmediateToDoubleRegister {
    /// The destination double register.
    pub destination: DoubleRegister,
    /// The immediate value. Will only valid after the second phase.
    pub value: u16,
    /// The current phase of the instruction.
    pub phase: ThreePhases,
}

impl Instruction for LoadImmediateToDoubleRegister {
    fn execute<T: MemoryDevice>(
        &self,
        cpu: &mut crate::cpu::CpuState,
        memory: &mut T,
    ) -> super::InstructionEnum {
        match self.phase {
            ThreePhases::First => {
                let program_counter = cpu.advance_program_counter();
                let value_lsb = memory.read(program_counter);

                Self {
                    destination: self.destination,
                    phase: ThreePhases::Second,
                    value: value_lsb as u16,
                }
                .into()
            }
            ThreePhases::Second => {
                let program_counter = cpu.advance_program_counter();
                let value_msb = memory.read(program_counter) as u16;
                let data = self.value | ((value_msb) << 8);
                match self.destination {
                    DoubleRegister::AF => cpu.write_stack_pointer(data),
                    _ => cpu.write_double_register(self.destination, data),
                };

                Self {
                    destination: self.destination,
                    phase: ThreePhases::Third,
                    value: data,
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
        let opcode = 0b00000001 | register_part;
        match self.phase {
            ThreePhases::Second => Vec::from([opcode, self.value.to_le_bytes()[0]]),
            ThreePhases::Third => Vec::from([
                opcode,
                self.value.to_le_bytes()[0],
                self.value.to_le_bytes()[1],
            ]),
            _ => Vec::from([opcode]),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LoadImmediateToDoubleRegister;
    use crate::cpu::instruction::phases::ThreePhases;
    use crate::cpu::instruction::{Instruction, InstructionEnum};
    use crate::cpu::{Cpu, CpuState, DoubleRegister};
    use crate::memory::MemoryController;

    #[test]
    fn load_immediate_to_double_register_works() {
        // Write 42 to A and then copy A to C
        let mut cpu = CpuState::new();
        let mut memory = MemoryController::new_with_init(&[0x34, 0x12]);

        let instruction = LoadImmediateToDoubleRegister {
            destination: DoubleRegister::BC,
            value: 0,
            phase: ThreePhases::First,
        };

        let instruction = instruction.execute(&mut cpu, &mut memory);

        assert!(matches!(
            instruction,
            InstructionEnum::LoadImmediateToDoubleRegister(LoadImmediateToDoubleRegister {
                phase: ThreePhases::Second,
                destination: DoubleRegister::BC,
                value: 0x0034
            })
        ));

        let instruction = instruction.execute(&mut cpu, &mut memory);

        assert!(matches!(
            instruction,
            InstructionEnum::LoadImmediateToDoubleRegister(LoadImmediateToDoubleRegister {
                phase: ThreePhases::Third,
                destination: DoubleRegister::BC,
                value: 0x1234
            })
        ));

        assert_eq!(cpu.read_double_register(DoubleRegister::BC), 0x1234);
    }

    #[test]
    fn encode_load_immediate_to_register() {
        // Write 42 to A and then copy A to C
        let mut cpu = CpuState::new();
        let mut memory = MemoryController::new_with_init(&[0x34, 0x12]);

        let instruction = LoadImmediateToDoubleRegister {
            destination: DoubleRegister::DE,
            value: 0,
            phase: ThreePhases::First,
        };

        let instruction = instruction.execute(&mut cpu, &mut memory);
        let instruction = instruction.execute(&mut cpu, &mut memory);

        let encoded = instruction.encode();

        assert_eq!(encoded[0], 0b00010001);
        assert_eq!(encoded[1], 0x34);
        assert_eq!(encoded[2], 0x12);
    }
}
