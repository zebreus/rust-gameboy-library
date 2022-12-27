use super::phases::TwoPhases;
use super::Instruction;
use crate::cpu::DoubleRegister;
use crate::{cpu::Cpu, memory::MemoryDevice};

/// Increment a [DoubleRegister] by `1`.
///
/// Setting destination to DoubleRegister::AF actually means incrementing the stackpointer
///
/// | [Zero](Flag::Zero)  | [Subtract](Flag::Subtract) | [HalfCarry](Flag::HalfCarry) | [Carry](Flag::Carry) |
/// |---------------------|----------------------------|------------------------------|----------------------|
/// | unchanged           | unchanged                  | unchanged                    | unchanged            |
#[doc(alias = "INC")]
#[doc(alias = "INC BC")]
#[doc(alias = "INC DE")]
#[doc(alias = "INC HL")]
#[doc(alias = "INC SP")]
pub struct IncrementDoubleRegister {
    /// The destination double register.
    pub destination: DoubleRegister,
    /// The current phase of the instruction.
    pub phase: TwoPhases,
}

impl Instruction for IncrementDoubleRegister {
    fn execute<T: MemoryDevice>(
        &self,
        cpu: &mut crate::cpu::CpuState,
        memory: &mut T,
    ) -> super::InstructionEnum {
        match self.phase {
            TwoPhases::First => {
                match self.destination {
                    DoubleRegister::AF => {
                        cpu.write_stack_pointer(cpu.read_stack_pointer().wrapping_add(1))
                    }
                    _ => cpu.write_double_register(
                        self.destination,
                        cpu.read_double_register(self.destination).wrapping_add(1),
                    ),
                };

                Self {
                    destination: self.destination,
                    phase: TwoPhases::Second,
                }
                .into()
            }
            TwoPhases::Second => {
                return cpu.load_instruction(memory);
            }
        }
    }
    fn encode(&self) -> Vec<u8> {
        let register_part = self.destination.numerical_id() << 4;
        let opcode = 0b00000011 | register_part;
        Vec::from([opcode])
    }
}

#[cfg(test)]
mod tests {
    use super::IncrementDoubleRegister;
    use crate::cpu::instruction::phases::TwoPhases;
    use crate::cpu::instruction::{Instruction, InstructionEnum};
    use crate::cpu::{Cpu, CpuState, DoubleRegister};
    use crate::memory::Memory;

    #[test]
    fn instruction_works() {
        let mut cpu = CpuState::new();
        let mut memory = Memory::new_with_init(&[0x34, 0x12]);
        cpu.write_double_register(DoubleRegister::BC, 788);

        let instruction = IncrementDoubleRegister {
            destination: DoubleRegister::BC,
            phase: TwoPhases::First,
        };

        let instruction = instruction.execute(&mut cpu, &mut memory);

        assert!(matches!(
            instruction,
            InstructionEnum::IncrementDoubleRegister(IncrementDoubleRegister {
                phase: TwoPhases::Second,
                destination: DoubleRegister::BC,
            })
        ));

        assert_eq!(cpu.read_double_register(DoubleRegister::BC), 789);
    }

    #[test]
    fn instruction_works_with_stackpointer() {
        let mut cpu = CpuState::new();
        let mut memory = Memory::new();
        cpu.write_stack_pointer(788);
        cpu.write_double_register(DoubleRegister::AF, 0b0011110000000000);

        let instruction = IncrementDoubleRegister {
            destination: DoubleRegister::AF,
            phase: TwoPhases::First,
        };

        let instruction = instruction.execute(&mut cpu, &mut memory);

        assert!(matches!(
            instruction,
            InstructionEnum::IncrementDoubleRegister(IncrementDoubleRegister {
                phase: TwoPhases::Second,
                destination: DoubleRegister::AF,
            })
        ));

        assert_eq!(cpu.read_stack_pointer(), 789);
        // AF should stay unchanged
        assert_eq!(
            cpu.read_double_register(DoubleRegister::AF),
            0b0011110000000000
        );
    }
}
