use super::phases::TwoPhases;
use super::Instruction;
use crate::{
    cpu::{Cpu, DoubleRegister, Flag},
    memory::MemoryDevice,
};

/// Adds a [DoubleRegister] to [DoubleRegister::HL].
///
/// Setting the operand to DoubleRegister::AF actually means adding the stackpointer the stackpointer
///
/// | [Zero](Flag::Zero)  | [Subtract](Flag::Subtract) | [HalfCarry](Flag::HalfCarry)        | [Carry](Flag::Carry)       |
/// |---------------------|----------------------------|-------------------------------------|----------------------------|
/// | unchanged           | false                      | true if the lower nibble overflowed | true if a overflow occured |
#[doc(alias = "ADD")]
#[doc(alias = "ADD HL,BC")]
#[doc(alias = "ADD HL,DE")]
#[doc(alias = "ADD HL,HL")]
#[doc(alias = "ADD HL,SP")]
pub struct AddDoubleRegisterToHl {
    /// The destination double register.
    pub operand: DoubleRegister,
    /// The current phase of the instruction.
    pub phase: TwoPhases,
}

impl Instruction for AddDoubleRegisterToHl {
    fn execute<T: MemoryDevice>(
        &self,
        cpu: &mut crate::cpu::CpuState,
        memory: &mut T,
    ) -> super::InstructionEnum {
        match self.phase {
            TwoPhases::First => {
                let operand = match self.operand {
                    DoubleRegister::AF => cpu.read_stack_pointer(),
                    _ => cpu.read_double_register(self.operand),
                };
                let previous_value = cpu.read_double_register(DoubleRegister::HL);
                let (result, carry_flag) = previous_value.overflowing_add(operand);
                let subtract_flag = false;
                let half_carry_flag = (previous_value.to_le_bytes()[1]
                    ^ operand.to_le_bytes()[1]
                    ^ result.to_le_bytes()[1])
                    & 0b00010000
                    == 0b00010000;

                cpu.write_flag(Flag::Subtract, subtract_flag);
                cpu.write_flag(Flag::HalfCarry, half_carry_flag);
                cpu.write_flag(Flag::Carry, carry_flag);

                cpu.write_double_register(DoubleRegister::HL, result);

                Self {
                    operand: self.operand,
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
        let register_part = self.operand.numerical_id() << 4;
        let opcode = 0b00001001 | register_part;
        Vec::from([opcode])
    }
}

#[cfg(test)]
mod tests {
    use super::AddDoubleRegisterToHl;
    use crate::cpu::instruction::phases::TwoPhases;
    use crate::cpu::instruction::{Instruction, InstructionEnum};
    use crate::cpu::{Cpu, CpuState, DoubleRegister};
    use crate::debug_memory::DebugMemory;

    #[test]
    fn instruction_works() {
        let mut cpu = CpuState::new();
        let mut memory = DebugMemory::new();
        cpu.write_double_register(DoubleRegister::HL, 324);
        cpu.write_double_register(DoubleRegister::BC, 788);

        let instruction = AddDoubleRegisterToHl {
            operand: DoubleRegister::BC,
            phase: TwoPhases::First,
        };

        let instruction = instruction.execute(&mut cpu, &mut memory);

        assert!(matches!(
            instruction,
            InstructionEnum::AddDoubleRegisterToHl(AddDoubleRegisterToHl {
                phase: TwoPhases::Second,
                operand: DoubleRegister::BC,
            })
        ));

        assert_eq!(cpu.read_double_register(DoubleRegister::BC), 788);
        assert_eq!(cpu.read_double_register(DoubleRegister::HL), 788 + 324);
    }

    #[test]
    fn instruction_works_with_stackpointer() {
        let mut cpu = CpuState::new();
        let mut memory = DebugMemory::new();
        cpu.write_double_register(DoubleRegister::AF, 0b0011110000000000);
        cpu.write_stack_pointer(788);
        cpu.write_double_register(DoubleRegister::HL, 324);

        let instruction = AddDoubleRegisterToHl {
            operand: DoubleRegister::AF,
            phase: TwoPhases::First,
        };

        let instruction = instruction.execute(&mut cpu, &mut memory);

        assert!(matches!(
            instruction,
            InstructionEnum::AddDoubleRegisterToHl(AddDoubleRegisterToHl {
                phase: TwoPhases::Second,
                operand: DoubleRegister::AF,
            })
        ));

        assert_eq!(
            cpu.read_double_register(DoubleRegister::AF),
            0b0011110000000000
        );
        assert_eq!(cpu.read_stack_pointer(), 788);
        assert_eq!(cpu.read_double_register(DoubleRegister::HL), 788 + 324);
    }
}
