use super::phases::FivePhases;
use super::Instruction;
use crate::{
    cpu::{ConditionCode, Cpu},
    memory_device::MemoryDevice,
};

/// ReturnConditional from a previous [Call](super::Call) instruction if the condition is met.
///
/// The condition is evaluated in the first phase.
///
/// This instruction is only two phases long, if the condition is not met in the first phase.
///
/// Basically just [pops](super::PopDoubleRegister) a address from the stack and sets the program counter to it.
#[doc(alias = "RET")]
#[doc(alias = "RET NZ")]
#[doc(alias = "RET Z")]
#[doc(alias = "RET NC")]
#[doc(alias = "RET C")]
pub struct ReturnConditional {
    /// The jump is only made if the condition is fullfilled in the third phase.
    pub condition: ConditionCode,
    /// The current phase of the instruction.
    pub phase: FivePhases,
}

impl Instruction for ReturnConditional {
    fn execute<T: MemoryDevice>(
        &self,
        cpu: &mut crate::cpu::CpuState,
        memory: &mut T,
    ) -> super::InstructionEnum {
        match self.phase {
            FivePhases::First => {
                let condition_fullfilled = cpu.check_condition(self.condition);
                if !condition_fullfilled {
                    return Self {
                        condition: self.condition,
                        phase: FivePhases::Fifth,
                    }
                    .into();
                }

                Self {
                    condition: self.condition,
                    phase: FivePhases::Second,
                }
                .into()
            }
            FivePhases::Second => {
                let data = memory.read(cpu.read_stack_pointer());
                let new_program_counter =
                    u16::from_le_bytes([data, cpu.read_program_counter().to_le_bytes()[1]]);
                cpu.write_program_counter(new_program_counter);
                cpu.write_stack_pointer(cpu.read_stack_pointer() + 1);

                Self {
                    condition: self.condition,
                    phase: FivePhases::Third,
                }
                .into()
            }
            FivePhases::Third => {
                let data = memory.read(cpu.read_stack_pointer());
                let new_program_counter =
                    u16::from_le_bytes([cpu.read_program_counter().to_le_bytes()[0], data]);
                cpu.write_program_counter(new_program_counter);
                cpu.write_stack_pointer(cpu.read_stack_pointer() + 1);

                Self {
                    condition: self.condition,
                    phase: FivePhases::Fourth,
                }
                .into()
            }
            FivePhases::Fourth => Self {
                condition: self.condition,
                phase: FivePhases::Fifth,
            }
            .into(),
            FivePhases::Fifth => {
                return cpu.load_instruction(memory);
            }
        }
    }
    fn encode(&self) -> Vec<u8> {
        let condition_code_part = ((self.condition as u8) << 3) & 0b00011000;
        let opcode = 0b11000000 | condition_code_part;
        Vec::from([opcode])
    }
}

#[cfg(test)]
mod tests {
    use super::ReturnConditional;
    use crate::cpu::instruction::phases::FivePhases;
    use crate::cpu::instruction::{Instruction, InstructionEnum};
    use crate::cpu::{ConditionCode, Cpu, CpuState, Flag};
    use crate::debug_memory::DebugMemory;
    use crate::memory_device::MemoryDevice;

    #[test]
    fn return_conditional_returns_when_it_should() {
        let mut cpu = CpuState::new();
        let mut memory = DebugMemory::new();
        cpu.write_flag(Flag::Carry, true);
        cpu.write_stack_pointer(0x1234 - 2);
        memory.write(0x1234 - 2, 0x34);
        memory.write(0x1234 - 1, 0x12);

        let instruction = ReturnConditional {
            condition: ConditionCode::CarryFlagSet,
            phase: FivePhases::First,
        };

        let instruction = instruction.execute(&mut cpu, &mut memory);
        let instruction = instruction.execute(&mut cpu, &mut memory);
        let instruction = instruction.execute(&mut cpu, &mut memory);
        let instruction = instruction.execute(&mut cpu, &mut memory);

        assert!(matches!(
            instruction,
            InstructionEnum::ReturnConditional(ReturnConditional {
                condition: ConditionCode::CarryFlagSet,
                phase: FivePhases::Fifth,
            })
        ));

        assert_eq!(cpu.read_stack_pointer(), 0x1234);
        assert_eq!(cpu.read_program_counter(), 0x1234);
        assert_eq!(memory.read(0x1234 - 2), 0x34);
        assert_eq!(memory.read(0x1234 - 1), 0x12);
    }

    #[test]
    fn return_conditional_does_not_return_when_the_condition_is_not_met() {
        let mut cpu = CpuState::new();
        let mut memory = DebugMemory::new();
        let initial_program_counter = cpu.read_program_counter();

        cpu.write_flag(Flag::Carry, false);
        cpu.write_stack_pointer(0x1234 - 2);
        memory.write(0x1234 - 2, 0x34);
        memory.write(0x1234 - 1, 0x12);

        let instruction = ReturnConditional {
            condition: ConditionCode::CarryFlagSet,
            phase: FivePhases::First,
        };

        let instruction = instruction.execute(&mut cpu, &mut memory);
        let instruction = instruction.execute(&mut cpu, &mut memory);

        assert!(!matches!(
            instruction,
            InstructionEnum::ReturnConditional(_)
        ));

        assert_eq!(cpu.read_stack_pointer(), 0x1234 - 2);
        assert_eq!(cpu.read_program_counter(), initial_program_counter + 1);
        assert_eq!(memory.read(0x1234 - 2), 0x34);
        assert_eq!(memory.read(0x1234 - 1), 0x12);
    }
}
