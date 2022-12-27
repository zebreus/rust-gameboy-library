use super::phases::TwoPhases;
use super::Instruction;
use crate::cpu::DoubleRegister;
use crate::{cpu::Cpu, memory::MemoryDevice};

/// Decrement a [DoubleRegister] by `1`.
///
/// Setting destination to DoubleRegister::AF actually means decrementing the stackpointer
///
/// | [Zero](Flag::Zero)  | [Subtract](Flag::Subtract) | [HalfCarry](Flag::HalfCarry) | [Carry](Flag::Carry) |
/// |---------------------|----------------------------|------------------------------|----------------------|
/// | unchanged           | unchanged                  | unchanged                    | unchanged            |
#[doc(alias = "DEC")]
#[doc(alias = "DEC BC")]
#[doc(alias = "DEC DE")]
#[doc(alias = "DEC HL")]
#[doc(alias = "DEC SP")]
#[derive(Debug)]
pub struct DecrementDoubleRegister {
    /// The destination double register.
    pub destination: DoubleRegister,
    /// The current phase of the instruction.
    pub phase: TwoPhases,
}

impl Instruction for DecrementDoubleRegister {
    fn execute<T: MemoryDevice>(
        &self,
        cpu: &mut crate::cpu::CpuState,
        memory: &mut T,
    ) -> super::InstructionEnum {
        match self.phase {
            TwoPhases::First => {
                match self.destination {
                    DoubleRegister::AF => {
                        cpu.write_stack_pointer(cpu.read_stack_pointer().wrapping_sub(1))
                    }
                    _ => cpu.write_double_register(
                        self.destination,
                        cpu.read_double_register(self.destination).wrapping_sub(1),
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
        let opcode = 0b00001011 | register_part;
        Vec::from([opcode])
    }
}

#[cfg(test)]
mod tests {
    use super::DecrementDoubleRegister;
    use crate::cpu::instruction::phases::TwoPhases;
    use crate::cpu::instruction::{Instruction, InstructionEnum};
    use crate::cpu::{Cpu, CpuState, DoubleRegister};
    use crate::memory::Memory;

    #[test]
    fn instruction_works() {
        let mut cpu = CpuState::new();
        let mut memory = Memory::new_with_init(&[0x34, 0x12]);
        cpu.write_double_register(DoubleRegister::BC, 788);

        let instruction = DecrementDoubleRegister {
            destination: DoubleRegister::BC,
            phase: TwoPhases::First,
        };

        let instruction = instruction.execute(&mut cpu, &mut memory);

        assert!(matches!(
            instruction,
            InstructionEnum::DecrementDoubleRegister(DecrementDoubleRegister {
                phase: TwoPhases::Second,
                destination: DoubleRegister::BC,
            })
        ));

        assert_eq!(cpu.read_double_register(DoubleRegister::BC), 787);
    }

    #[test]
    fn instruction_works_with_stackpointer() {
        let mut cpu = CpuState::new();
        let mut memory = Memory::new();
        cpu.write_stack_pointer(788);
        cpu.write_double_register(DoubleRegister::AF, 0b0011110000000000);

        let instruction = DecrementDoubleRegister {
            destination: DoubleRegister::AF,
            phase: TwoPhases::First,
        };

        let instruction = instruction.execute(&mut cpu, &mut memory);

        assert!(matches!(
            instruction,
            InstructionEnum::DecrementDoubleRegister(DecrementDoubleRegister {
                phase: TwoPhases::Second,
                destination: DoubleRegister::AF,
            })
        ));

        assert_eq!(cpu.read_stack_pointer(), 787);
        // AF should stay unchanged
        assert_eq!(
            cpu.read_double_register(DoubleRegister::AF),
            0b0011110000000000
        );
    }
}
