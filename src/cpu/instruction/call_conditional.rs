use super::phases::SixPhases;
use super::Instruction;
use crate::{
    cpu::{ConditionCode, Cpu},
    memory_device::MemoryDevice,
};

/// Jumps to the address specified in the two bytes following the opcode. Writes the program counter before the jump onto the stack.
///
/// See [PushDoubleRegister](super::PushDoubleRegister) for more details on how data is pushed to the stack.
///
/// The value pushed to the stack points to the next instruction directly after this one.
pub struct CallConditional {
    /// The call is only made if the condition is fullfilled in the third phase.
    pub condition: ConditionCode,
    /// The immediate address. Will only valid after the second phase.
    pub address: u16,
    /// The current phase of the instruction.
    pub phase: SixPhases,
}

impl Instruction for CallConditional {
    fn execute<T: MemoryDevice>(
        &self,
        cpu: &mut crate::cpu::CpuState,
        memory: &mut T,
    ) -> super::InstructionEnum {
        match self.phase {
            SixPhases::First => {
                let program_counter = cpu.advance_program_counter();
                let address_lsb = memory.read(program_counter);

                Self {
                    condition: self.condition,
                    phase: SixPhases::Second,
                    address: address_lsb as u16,
                }
                .into()
            }
            SixPhases::Second => {
                let program_counter = cpu.advance_program_counter();
                let address_msb = memory.read(program_counter);

                Self {
                    condition: self.condition,
                    phase: SixPhases::Third,
                    address: u16::from_le_bytes([self.address as u8, address_msb]),
                }
                .into()
            }
            SixPhases::Third => {
                let condition_fullfilled = cpu.check_condition(self.condition);
                if !condition_fullfilled {
                    return cpu.load_instruction(memory);
                }

                cpu.write_stack_pointer(cpu.read_stack_pointer() - 1);

                Self {
                    condition: self.condition,
                    phase: SixPhases::Fourth,
                    address: self.address,
                }
                .into()
            }
            SixPhases::Fourth => {
                let data = cpu.read_program_counter().to_le_bytes()[1];
                memory.write(cpu.read_stack_pointer(), data);

                cpu.write_stack_pointer(cpu.read_stack_pointer() - 1);

                Self {
                    condition: self.condition,
                    phase: SixPhases::Fifth,
                    address: self.address,
                }
                .into()
            }
            SixPhases::Fifth => {
                let data = cpu.read_program_counter().to_le_bytes()[0];
                memory.write(cpu.read_stack_pointer(), data);

                cpu.write_program_counter(self.address);
                Self {
                    condition: self.condition,
                    phase: SixPhases::Sixth,
                    address: self.address,
                }
                .into()
            }
            SixPhases::Sixth => {
                return cpu.load_instruction(memory);
            }
        }
    }
    fn encode(&self) -> Vec<u8> {
        let condition_code_part = ((self.condition as u8) << 3) & 0b00011000;
        let opcode = 0b11000100 | condition_code_part;

        match self.phase {
            SixPhases::First => Vec::from([opcode]),
            SixPhases::Second => Vec::from([opcode, self.address.to_le_bytes()[0]]),
            _ => Vec::from([
                opcode,
                self.address.to_le_bytes()[0],
                self.address.to_le_bytes()[1],
            ]),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::CallConditional;
    use crate::cpu::instruction::phases::SixPhases;
    use crate::cpu::instruction::{Instruction, InstructionEnum};
    use crate::cpu::{ConditionCode, Cpu, CpuState, Flag};
    use crate::debug_memory::DebugMemory;
    use crate::memory_device::MemoryDevice;

    #[test]
    fn call_conditional_works() {
        let mut cpu = CpuState::new();
        let mut memory = DebugMemory::new_with_init(&[0x34, 0x12]);
        cpu.write_stack_pointer(0xff00);
        cpu.write_flag(Flag::Carry, true);
        let initial_program_counter = cpu.read_program_counter();

        let instruction = CallConditional {
            condition: ConditionCode::CarryFlagSet,
            address: 0,
            phase: SixPhases::First,
        };

        let instruction = instruction.execute(&mut cpu, &mut memory);
        let instruction = instruction.execute(&mut cpu, &mut memory);

        assert!(matches!(
            instruction,
            InstructionEnum::CallConditional(CallConditional {
                condition: ConditionCode::CarryFlagSet,
                phase: SixPhases::Third,
                address: 0x1234
            })
        ));

        let instruction = instruction.execute(&mut cpu, &mut memory);
        let instruction = instruction.execute(&mut cpu, &mut memory);
        let instruction = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cpu.read_program_counter(), 0x1234);

        assert!(matches!(
            instruction,
            InstructionEnum::CallConditional(CallConditional {
                condition: ConditionCode::CarryFlagSet,
                phase: SixPhases::Sixth,
                address: 0x1234
            })
        ));

        assert_eq!(cpu.read_stack_pointer(), 0xff00 - 2);
        assert_eq!(
            memory.read(cpu.read_stack_pointer()),
            (initial_program_counter + 2).to_le_bytes()[0]
        );
        assert_eq!(
            memory.read(cpu.read_stack_pointer() + 1),
            (initial_program_counter + 2).to_le_bytes()[1]
        );
    }

    #[test]
    fn call_conditional_does_not_jump_when_the_condition_is_not_met() {
        let mut cpu = CpuState::new();
        let mut memory = DebugMemory::new_with_init(&[0x34, 0x12]);
        cpu.write_stack_pointer(0xff00);
        cpu.write_flag(Flag::Carry, false);
        let initial_program_counter = cpu.read_program_counter();

        let instruction = CallConditional {
            condition: ConditionCode::CarryFlagSet,
            address: 0,
            phase: SixPhases::First,
        };

        let instruction = instruction.execute(&mut cpu, &mut memory);
        let instruction = instruction.execute(&mut cpu, &mut memory);

        assert!(matches!(
            instruction,
            InstructionEnum::CallConditional(CallConditional {
                condition: ConditionCode::CarryFlagSet,
                phase: SixPhases::Third,
                address: 0x1234
            })
        ));
        assert_eq!(cpu.read_program_counter(), initial_program_counter + 2);

        let instruction = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cpu.read_program_counter(), initial_program_counter + 3);
        assert_eq!(cpu.read_stack_pointer(), 0xff00);

        assert!(!matches!(instruction, InstructionEnum::CallConditional(_)));
    }

    #[test]
    fn encode_call_conditional() {
        let mut cpu = CpuState::new();
        let mut memory = DebugMemory::new_with_init(&[0x34, 0x12]);

        let instruction = CallConditional {
            condition: crate::cpu::ConditionCode::ZeroFlagSet,
            address: 0,
            phase: SixPhases::First,
        };

        let instruction = instruction.execute(&mut cpu, &mut memory);
        let instruction = instruction.execute(&mut cpu, &mut memory);

        let encoded = instruction.encode();

        assert_eq!(encoded[0], 0b11001100);
        assert_eq!(encoded[1], 0x34);
        assert_eq!(encoded[2], 0x12);
    }
}
