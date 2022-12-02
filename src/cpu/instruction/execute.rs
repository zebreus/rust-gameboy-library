use crate::memory_device::MemoryDevice;

use super::Cpu;
use super::CpuState;

use super::load_instruction;
use super::DoubleRegister;
use super::Instruction;

impl Instruction {
    pub fn execute(&self, cpu: &mut CpuState, memory: &mut dyn MemoryDevice) -> Instruction {
        match self {
            Instruction::LoadFromRegisterToRegister {
                source,
                destination,
            } => {
                cpu.registers[*destination as usize] = cpu.registers[*source as usize];
                return load_instruction(cpu, memory);
            }
            Instruction::LoadImmediateToRegister {
                destination,
                value: _,
                phase: 0,
            } => {
                let address = cpu.read_program_counter();
                let value = memory.read(address);
                return Instruction::LoadImmediateToRegister {
                    destination: *destination,
                    value,
                    phase: 1,
                };
            }
            Instruction::LoadImmediateToRegister {
                destination,
                value,
                phase: 1_u8..=u8::MAX,
            } => {
                cpu.write_register(*destination, *value);
                return load_instruction(cpu, memory);
            }
            Instruction::LoadFromHlToRegister {
                destination,
                phase: 0,
            } => {
                let address = cpu.read_double_register(DoubleRegister::HL);
                let data = memory.read(address);
                // This should probably happen in the next phase of this instruction
                cpu.write_register(*destination, data);
                Instruction::LoadFromHlToRegister {
                    destination: *destination,
                    phase: 1,
                }
            }
            Instruction::LoadFromHlToRegister {
                destination: _,
                phase: 1_u8..=u8::MAX,
            } => load_instruction(cpu, memory),
            Instruction::None => Instruction::None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::CpuState;
    use super::{Cpu, Instruction};
    use crate::cpu::instruction::decode::decode;
    use crate::cpu::{DoubleRegister, Register};
    use crate::debug_memory::DebugMemory;

    #[test]
    fn load_instruction_works() {
        let mut cpu = CpuState::new();
        cpu.write_register(Register::A, 100);

        let mut memory = DebugMemory::new();
        let load_a_to_c = 0b01000010u8;
        let instruction = decode(load_a_to_c);

        let value_c_before = cpu.read_register(Register::C);
        assert_eq!(value_c_before, 0);

        instruction.execute(&mut cpu, &mut memory);
        let value_c_after = cpu.read_register(Register::C);

        assert_eq!(value_c_after, 100);
    }

    #[test]
    fn load_register_from_register_works() {
        // Write 42 to A and then copy A to C
        let mut cpu = CpuState::new();
        let mut memory = DebugMemory::new_with_init(&[]);

        let instruction = Instruction::LoadFromRegisterToRegister {
            source: Register::A,
            destination: Register::B,
        };

        cpu.write_register(Register::A, 42);

        instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cpu.read_register(Register::A), 42);
        assert_eq!(cpu.read_register(Register::B), 42);
    }

    #[test]
    fn load_immediate_to_register_works() {
        // Write 42 to A and then copy A to C
        let mut cpu = CpuState::new();
        let mut memory = DebugMemory::new_with_init(&[42]);

        let instruction = Instruction::LoadImmediateToRegister {
            destination: Register::B,
            value: 0,
            phase: 0,
        };

        let instruction = instruction.execute(&mut cpu, &mut memory);

        assert!(matches!(
            instruction,
            Instruction::LoadImmediateToRegister {
                phase: 1,
                destination: Register::B,
                value: 42
            }
        ));

        instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cpu.read_register(Register::B), 42);
        assert_eq!(cpu.read_register(Register::A), 0);
    }

    #[test]
    fn load_from_hl_to_register_works() {
        // Write 42 to A and then copy A to C
        let mut cpu = CpuState::new();
        let mut memory = DebugMemory::new_with_init(&[0, 0, 0, 42]);

        let instruction = Instruction::LoadFromHlToRegister {
            destination: Register::B,
            phase: 0,
        };

        cpu.write_double_register(DoubleRegister::HL, 3);

        let instruction = instruction.execute(&mut cpu, &mut memory);

        assert!(matches!(
            instruction,
            Instruction::LoadFromHlToRegister {
                phase: 1,
                destination: Register::B,
            }
        ));

        instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cpu.read_register(Register::B), 42);
        assert_eq!(cpu.read_register(Register::A), 0);
    }
}
