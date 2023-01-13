use super::phases::FourPhases;
use super::Instruction;
use crate::{
    cpu::{Cpu, Flag},
    memory::MemoryDevice,
};

/// Adds a signed offset specified in the byte following the opcode to the stackpointer.
///
/// This is actually an addition of two unsigned 16 bit numbers. At least the flags behave that way.
///
/// First the offset is converted to its unsigned twos complement representation. Then it is added to the stackpointer.
///
/// The flags are set according to the result of the addition of both LSBs.
///
/// [Flag::HalfCarry] gets set if the nipple of the LSB overflowed.
///
/// For more details on how 16 bit operations affect flags see <https://stackoverflow.com/a/57981912/5392501>.
///
///
/// | [Zero](Flag::Zero)  | [Subtract](Flag::Subtract) | [HalfCarry](Flag::HalfCarry)             | [Carry](Flag::Carry)                  |
/// |---------------------|----------------------------|------------------------------------------|---------------------------------------|
/// | false               | false                      | true if the nibble overflowed on the LSB | true if a overflow occured on the LSB |
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
                let operand: u16 = self.offset as u16;
                let result = previous_stack_pointer.wrapping_add(operand);
                let (_, carry_flag) = previous_stack_pointer.to_le_bytes()[0]
                    .overflowing_add(operand.to_le_bytes()[0]);
                let zero_flag = false;
                let subtract_flag = false;
                let half_carry_flag = (previous_stack_pointer.to_le_bytes()[0]
                    ^ operand.to_le_bytes()[0]
                    ^ result.to_le_bytes()[0])
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
    use crate::cpu::{Cpu, CpuState, Flag};
    use crate::memory::MemoryController;

    fn run_instruction(original_stackpointer: u16, offset: i8) -> CpuState {
        let mut cpu = CpuState::new();
        let mut memory = MemoryController::new_with_init(&[(-20 as i8).to_ne_bytes()[0]]);

        cpu.write_stack_pointer(original_stackpointer);

        let instruction = AddImmediateOffsetToSp {
            offset,
            phase: FourPhases::Second,
        };
        let instruction = instruction.execute(&mut cpu, &mut memory);
        let instruction = instruction.execute(&mut cpu, &mut memory);
        instruction.execute(&mut cpu, &mut memory);

        return cpu;
    }

    #[test]
    fn instruction_works() {
        let mut cpu = CpuState::new();
        let mut memory = MemoryController::new_with_init(&[(-20 as i8).to_ne_bytes()[0]]);

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

    #[test]
    fn basic_addition_works() {
        let cpu = run_instruction(0, 1);
        assert_eq!(cpu.read_stack_pointer(), 1);
        assert_eq!(cpu.read_flag(Flag::HalfCarry), false);
        assert_eq!(cpu.read_flag(Flag::Carry), false);
    }

    #[test]
    fn addition_set_carry_flag_on_lsb_overflow() {
        let cpu = run_instruction(255, 1);
        assert_eq!(cpu.read_stack_pointer(), 256);
        assert_eq!(cpu.read_flag(Flag::HalfCarry), true);
        assert_eq!(cpu.read_flag(Flag::Carry), true);
    }

    #[test]
    fn addition_with_halfcarry_sets_flag() {
        let cpu = run_instruction(15, 1);
        assert_eq!(cpu.read_stack_pointer(), 16);
        assert_eq!(cpu.read_flag(Flag::HalfCarry), true);
        assert_eq!(cpu.read_flag(Flag::Carry), false);
    }

    #[test]
    fn addition_with_overflow_sets_flag() {
        let cpu = run_instruction(u16::MAX, 1);
        assert_eq!(cpu.read_stack_pointer(), 0);
        assert_eq!(cpu.read_flag(Flag::HalfCarry), true);
        assert_eq!(cpu.read_flag(Flag::Carry), true);
    }

    #[test]
    fn subtraction_with_overflow_sets_correct_flags() {
        let cpu = run_instruction(0xffff, -1);
        assert_eq!(cpu.read_stack_pointer(), 0xFFFE);
        assert_eq!(cpu.read_flag(Flag::HalfCarry), true);
        assert_eq!(cpu.read_flag(Flag::Carry), true);
    }

    #[test]
    fn subtraction_with_overflow_sets_flag() {
        let cpu = run_instruction(0, -1);
        assert_eq!(cpu.read_stack_pointer(), u16::MAX);
        assert_eq!(cpu.read_flag(Flag::HalfCarry), false);
        assert_eq!(cpu.read_flag(Flag::Carry), false);
    }
}
