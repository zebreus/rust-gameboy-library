use crate::memory_device::MemoryDevice;
use bitmatch::bitmatch;

use super::Cpu;
use super::CpuState;

use super::DoubleRegister;
use super::Register;

#[derive(Debug)]
pub enum Instruction {
    LoadFromRegisterToRegister {
        source: Register,
        destination: Register,
    },
    LoadImmediateToRegister {
        destination: Register,
        value: u8,
        phase: u8,
    },
    LoadFromHlToRegister {
        destination: Register,
        phase: u8,
    },
    None,
}

impl Instruction {
    fn execute(&self, cpu: &mut CpuState, memory: &mut dyn MemoryDevice) -> Instruction {
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
                    value: value,
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

#[bitmatch]
fn decode(byte: u8) -> Instruction {
    #[bitmatch]
    // TODO: How can we get rid of this (soon) massive match clause
    match byte {
        "01aaa110" => Instruction::LoadFromHlToRegister {
            destination: Register::try_from(a)
                .expect("3 bit value should always correspont to a register"),
            phase: 0,
        },
        "01aaabbb" => Instruction::LoadFromRegisterToRegister {
            source: Register::try_from(a)
                .expect("3 bit value should always correspont to a register"),
            destination: Register::try_from(b)
                .expect("3 bit value should always correspont to a register"),
        },
        "00aaa110" => Instruction::LoadImmediateToRegister {
            destination: Register::try_from(a)
                .expect("3 bit value should always correspont to a register"),
            value: 0,
            phase: 0,
        },
        _ => Instruction::None {},
    }
}

fn encode(instruction: Instruction) -> Vec<u8> {
    match instruction {
        Instruction::LoadFromRegisterToRegister {
            source,
            destination,
        } => {
            let base_code = 0b01000000 & 0b11000000u8;
            let source_code = (source.id() << 3) & 0b00111000u8;
            let destination_code = destination.id() & 0b00000111u8;
            let opcode = base_code | source_code | destination_code;
            Vec::from([opcode])
        }
        Instruction::LoadImmediateToRegister {
            destination,
            value,
            phase,
        } => {
            let base_code = 0b00000110 & 0b11000111u8;
            let destination_code = (destination.id() << 3) & 0b00111000u8;
            let opcode = base_code | destination_code;
            match phase {
                0 => Vec::from([opcode]),
                1 => Vec::from([opcode, value]),
                _ => Vec::new(),
            }
        }
        Instruction::LoadFromHlToRegister {
            destination,
            phase: _,
        } => {
            let base_code = 0b01000110 & 0b11000111u8;
            let destination_code = (destination.id() << 3) & 0b00111000u8;
            let opcode = base_code | destination_code;
            Vec::from([opcode])
        }
        Instruction::None => Vec::new(),
    }
}

pub fn load_opcode<T: Cpu>(cpu: &mut T, memory: &dyn MemoryDevice) -> u8 {
    let opcode = memory.read(cpu.read_program_counter());
    return opcode;
}

pub fn load_instruction<T: Cpu>(cpu: &mut T, memory: &dyn MemoryDevice) -> Instruction {
    let opcode = load_opcode(cpu, memory);
    return decode(opcode);
}

#[cfg(test)]
mod tests {
    use super::{decode, encode, CpuState};
    use super::{Cpu, Instruction};
    use crate::cpu::instruction::load_instruction;
    use crate::cpu::Register;
    use crate::debug_memory::DebugMemory;

    #[test]
    fn encode_load_instruction() {
        let load_a_to_c_instruction = Instruction::LoadFromRegisterToRegister {
            source: Register::A,
            destination: Register::C,
        };

        let encoded_instruction = encode(load_a_to_c_instruction);

        assert_eq!(encoded_instruction[0], 0b01000010u8);
    }

    #[test]
    fn decodes_load_instruction() {
        let load_a_to_c = 0b01000010u8;
        let instruction = decode(load_a_to_c);
        assert!(matches!(
            instruction,
            Instruction::LoadFromRegisterToRegister {
                source: Register::A,
                destination: Register::C
            }
        ))
    }

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
    fn load_instruction_integration() {
        // Write 42 to A and then copy A to C
        let mut cpu = CpuState::new();

        let mut memory = DebugMemory::new_with_init(&[0b00000110, 42, 0b01000010u8]);

        let instruction = load_instruction(&mut cpu, &memory);

        let instruction = instruction.execute(&mut cpu, &mut memory);
        let instruction = instruction.execute(&mut cpu, &mut memory);
        instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cpu.read_register(Register::A), 42);
        assert_eq!(cpu.read_register(Register::B), 0);
        assert_eq!(cpu.read_register(Register::C), 42);
    }

    #[test]
    fn load_from_hl_works() {
        // Write 42 to A and then copy A to C
        let mut cpu = CpuState::new();

        let mut memory = DebugMemory::new_with_init(&[
            0b00110110,
            0,
            0b00111110,
            9,
            0b01000110u8,
            0,
            0,
            0,
            0,
            42,
        ]);

        let instruction = load_instruction(&mut cpu, &memory);

        let instruction = instruction.execute(&mut cpu, &mut memory);
        let instruction = instruction.execute(&mut cpu, &mut memory);
        let instruction = instruction.execute(&mut cpu, &mut memory);
        let instruction = instruction.execute(&mut cpu, &mut memory);
        let instruction = instruction.execute(&mut cpu, &mut memory);
        instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cpu.read_register(Register::A), 42);
        assert_eq!(cpu.read_register(Register::B), 0);
        assert_eq!(cpu.read_register(Register::C), 0);
    }
}
