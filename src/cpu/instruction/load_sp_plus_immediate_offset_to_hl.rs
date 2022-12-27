use super::phases::ThreePhases;
use super::Instruction;
use crate::{
    cpu::{Cpu, DoubleRegister, Flag},
    memory::MemoryDevice,
};

/// Copies the stackpointer plus a signed offset specified in the byte following the opcode into [DoubleRegister::HL].
///
/// | [Zero](Flag::Zero)  | [Subtract](Flag::Subtract) | [HalfCarry](Flag::HalfCarry)        | [Carry](Flag::Carry)       |
/// |---------------------|----------------------------|-------------------------------------|----------------------------|
/// | false               | false                      | true if the lower nibble overflowed | true if a overflow occured |
#[doc(alias = "LD")]
#[doc(alias = "LD HL,SP+n")]
#[doc(alias = "LDHL")]
#[doc(alias = "LDHL SP,n")]
pub struct LoadSpPlusImmediateOffsetToHl {
    /// The immediate offset. Will only valid after the first phase.
    pub offset: i8,
    /// The current phase of the instruction.
    pub phase: ThreePhases,
}

impl Instruction for LoadSpPlusImmediateOffsetToHl {
    fn execute<T: MemoryDevice>(
        &self,
        cpu: &mut crate::cpu::CpuState,
        memory: &mut T,
    ) -> super::InstructionEnum {
        match self.phase {
            ThreePhases::First => {
                let program_counter = cpu.advance_program_counter();
                let offset: i8 = memory.read_signed(program_counter);

                Self {
                    phase: ThreePhases::Second,
                    offset,
                }
                .into()
            }
            ThreePhases::Second => {
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

                cpu.write_double_register(DoubleRegister::HL, result);

                Self {
                    phase: ThreePhases::Third,
                    offset: self.offset,
                }
                .into()
            }
            ThreePhases::Third => {
                return cpu.load_instruction(memory);
            }
        }
    }
    fn encode(&self) -> Vec<u8> {
        match self.phase {
            ThreePhases::First => Vec::from([0b11111000]),
            _ => Vec::from([0b11111000, self.offset.to_ne_bytes()[0]]),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LoadSpPlusImmediateOffsetToHl;
    use crate::cpu::instruction::phases::ThreePhases;
    use crate::cpu::instruction::{Instruction, InstructionEnum};
    use crate::cpu::{Cpu, CpuState, DoubleRegister};
    use crate::debug_memory::DebugMemory;

    #[test]
    fn instruction_works() {
        let mut cpu = CpuState::new();
        let mut memory = DebugMemory::new_with_init(&[(-20 as i8).to_ne_bytes()[0]]);

        cpu.write_stack_pointer(0x0f00);

        let instruction = LoadSpPlusImmediateOffsetToHl {
            offset: 0,
            phase: ThreePhases::First,
        };

        let instruction = instruction.execute(&mut cpu, &mut memory);

        assert!(matches!(
            instruction,
            InstructionEnum::LoadSpPlusImmediateOffsetToHl(LoadSpPlusImmediateOffsetToHl {
                phase: ThreePhases::Second,
                offset: -20
            })
        ));

        let instruction = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cpu.read_double_register(DoubleRegister::HL), 0x0f00 - 20);

        assert!(matches!(
            instruction,
            InstructionEnum::LoadSpPlusImmediateOffsetToHl(LoadSpPlusImmediateOffsetToHl {
                phase: ThreePhases::Third,
                offset: -20
            })
        ));
    }
}
