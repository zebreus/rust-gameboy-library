use super::phases::FourPhases;
use super::Instruction;
use crate::{
    cpu::{Cpu, Flag},
    memory::MemoryDevice,
};

/// Adds a signed offset specified in the byte following the opcode to the stackpointer.
///
/// | [Zero](Flag::Zero)  | [Subtract](Flag::Subtract) | [HalfCarry](Flag::HalfCarry)        | [Carry](Flag::Carry)       |
/// |---------------------|----------------------------|-------------------------------------|----------------------------|
/// | false               | false                      | true if the lower nibble overflowed | true if a overflow occured |
#[doc(alias = "ADD")]
#[doc(alias = "ADD SP,n")]
#[derive(Debug)]
pub struct AddImmediateOffsetToSp {
    /// The immediate offset. Will only valid after the first phase.
    pub offset: i8,
    /// The current phase of the instruction.
    pub phase: FourPhases,
}

impl Instruction for AddImmediateOffsetToSp {
    fn execute<T: MemoryDevice>(
        &self,
        cpu: &mut crate::cpu::CpuState,
        memory: &mut T,
    ) -> super::InstructionEnum {
        match self.phase {
            FourPhases::First => {
                let program_counter = cpu.advance_program_counter();
                let offset: i8 = memory.read_signed(program_counter);

                Self {
                    phase: FourPhases::Second,
                    offset,
                }
                .into()
            }
            FourPhases::Second => {
                let previous_stack_pointer = cpu.read_stack_pointer();
                let (result, carry_flag) =
                    previous_stack_pointer.overflowing_add_signed(self.offset.into());
                let zero_flag = false;
                let subtract_flag = false;
                let half_carry_flag = (previous_stack_pointer.to_le_bytes()[1]
                    ^ self.offset.to_ne_bytes()[0]
                    ^ result.to_le_bytes()[1])
                    & 0b00010000
                    == 0b00010000;

                cpu.write_flag(Flag::Zero, zero_flag);
                cpu.write_flag(Flag::Subtract, subtract_flag);
                cpu.write_flag(Flag::HalfCarry, half_carry_flag);
                cpu.write_flag(Flag::Carry, carry_flag);

                cpu.write_stack_pointer(result);

                Self {
                    phase: FourPhases::Third,
                    offset: self.offset,
                }
                .into()
            }
            FourPhases::Third => Self {
                phase: FourPhases::Fourth,
                offset: self.offset,
            }
            .into(),
            FourPhases::Fourth => {
                return cpu.load_instruction(memory);
            }
        }
    }
    fn encode(&self) -> Vec<u8> {
        match self.phase {
            FourPhases::First => Vec::from([0b11101000]),
            _ => Vec::from([0b11101000, self.offset.to_ne_bytes()[0]]),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::AddImmediateOffsetToSp;
    use crate::cpu::instruction::phases::FourPhases;
    use crate::cpu::instruction::{Instruction, InstructionEnum};
    use crate::cpu::{Cpu, CpuState};
    use crate::memory::Memory;

    #[test]
    fn instruction_works() {
        let mut cpu = CpuState::new();
        let mut memory = Memory::new_with_init(&[(-20 as i8).to_ne_bytes()[0]]);

        cpu.write_stack_pointer(0x0f00);

        let instruction = AddImmediateOffsetToSp {
            offset: 0,
            phase: FourPhases::First,
        };

        let instruction = instruction.execute(&mut cpu, &mut memory);

        assert!(matches!(
            instruction,
            InstructionEnum::AddImmediateOffsetToSp(AddImmediateOffsetToSp {
                phase: FourPhases::Second,
                offset: -20
            })
        ));

        let instruction = instruction.execute(&mut cpu, &mut memory);
        let instruction = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cpu.read_stack_pointer(), 0x0f00 - 20);

        assert!(matches!(
            instruction,
            InstructionEnum::AddImmediateOffsetToSp(AddImmediateOffsetToSp {
                phase: FourPhases::Fourth,
                offset: -20
            })
        ));
    }
}
