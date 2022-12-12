use crate::cpu::Register;
use crate::memory_device::MemoryDevice;

use super::Cpu;
use super::CpuState;

use super::load_instruction;
use super::DoubleRegister;
use super::Instruction;
use super::ThreePhases;
use super::TwoPhases;

impl Instruction {
    /// Execute the instruction on the cpu and memory. Returns the next instruction.
    ///
    /// # Examples
    ///
    /// ```
    /// # use rust_gameboy_library::cpu::{CpuState, Cpu, Register};
    /// # use rust_gameboy_library::cpu::instruction::Instruction;
    /// # use rust_gameboy_library::debug_memory::DebugMemory;
    /// #
    /// let mut cpu = CpuState::new();
    /// let mut memory = DebugMemory::new();
    /// cpu.write_register(Register::A, 100);
    ///
    /// let instruction = Instruction::LoadFromRegisterToRegister {
    ///     source: Register::A,
    ///     destination: Register::C,
    /// };
    ///
    /// instruction.execute(&mut cpu, &mut memory);
    /// assert_eq!(cpu.read_register(Register::C), 100);
    /// ```
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
                phase: TwoPhases::First,
            } => {
                let address = cpu.read_program_counter();
                let value = memory.read(address);
                return Instruction::LoadImmediateToRegister {
                    destination: *destination,
                    value,
                    phase: TwoPhases::Second,
                };
            }
            Instruction::LoadImmediateToRegister {
                destination,
                value,
                phase: TwoPhases::Second,
            } => {
                cpu.write_register(*destination, *value);
                return load_instruction(cpu, memory);
            }
            Instruction::LoadFromHlToRegister {
                destination,
                phase: TwoPhases::First,
            } => {
                let address = cpu.read_double_register(DoubleRegister::HL);
                let data = memory.read(address);
                // This should probably happen in the next phase of this instruction
                cpu.write_register(*destination, data);
                Instruction::LoadFromHlToRegister {
                    destination: *destination,
                    phase: TwoPhases::Second,
                }
            }
            Instruction::LoadFromHlToRegister {
                destination: _,
                phase: TwoPhases::Second,
            } => load_instruction(cpu, memory),

            Instruction::LoadAccumulatorToImmediateOffset { phase, offset } => match phase {
                ThreePhases::First => {
                    let next_address = cpu.read_program_counter();
                    let offset = memory.read(next_address);

                    Instruction::LoadAccumulatorToImmediateOffset {
                        phase: ThreePhases::Second,
                        offset: offset,
                    }
                }
                ThreePhases::Second => {
                    let offset_16: u16 = (*offset).into();
                    let address: u16 = 0xff00u16 | offset_16;
                    let data = cpu.read_register(Register::A);
                    memory.write(address, data);

                    Instruction::LoadAccumulatorToImmediateOffset {
                        phase: ThreePhases::Third,
                        offset: *offset,
                    }
                }
                ThreePhases::Third => load_instruction(cpu, memory),
            },
            Instruction::LoadFromImmediateOffsetToAccumulator { phase, offset } => match phase {
                ThreePhases::First => {
                    let next_address = cpu.read_program_counter();
                    let offset = memory.read(next_address);

                    Instruction::LoadFromImmediateOffsetToAccumulator {
                        phase: ThreePhases::Second,
                        offset: offset,
                    }
                }
                ThreePhases::Second => {
                    let offset_16: u16 = (*offset).into();
                    let address: u16 = 0xff00u16 | offset_16;
                    let data = memory.read(address);

                    cpu.write_register(Register::A, data);

                    Instruction::LoadFromImmediateOffsetToAccumulator {
                        phase: ThreePhases::Third,
                        offset: *offset,
                    }
                }
                ThreePhases::Third => load_instruction(cpu, memory),
            },

            Instruction::LoadHlToAccumulatorAndDecrement {
                phase: TwoPhases::First,
            } => {
                let address = cpu.read_double_register(DoubleRegister::HL);
                let data = memory.read(address);

                cpu.write_register(Register::A, data);
                cpu.write_double_register(DoubleRegister::HL, address - 1);

                Instruction::LoadAccumulatorToHlAndDecrement {
                    phase: TwoPhases::Second,
                }
            }
            Instruction::LoadHlToAccumulatorAndDecrement {
                phase: TwoPhases::Second,
            } => load_instruction(cpu, memory),
            Instruction::LoadAccumulatorToHlAndDecrement {
                phase: TwoPhases::First,
            } => {
                let address = cpu.read_double_register(DoubleRegister::HL);
                let data = cpu.read_register(Register::A);
                memory.write(address, data);
                cpu.write_double_register(DoubleRegister::HL, address - 1);

                Instruction::LoadAccumulatorToHlAndDecrement {
                    phase: TwoPhases::Second,
                }
            }
            Instruction::LoadAccumulatorToHlAndDecrement {
                phase: TwoPhases::Second,
            } => load_instruction(cpu, memory),
            Instruction::LoadHlToAccumulatorAndIncrement {
                phase: TwoPhases::First,
            } => {
                let address = cpu.read_double_register(DoubleRegister::HL);
                let data = memory.read(address);

                cpu.write_register(Register::A, data);
                cpu.write_double_register(DoubleRegister::HL, address + 1);

                Instruction::LoadAccumulatorToHlAndIncrement {
                    phase: TwoPhases::Second,
                }
            }
            Instruction::LoadHlToAccumulatorAndIncrement {
                phase: TwoPhases::Second,
            } => load_instruction(cpu, memory),
            Instruction::LoadAccumulatorToHlAndIncrement {
                phase: TwoPhases::First,
            } => {
                let address = cpu.read_double_register(DoubleRegister::HL);
                let data = cpu.read_register(Register::A);
                memory.write(address, data);
                cpu.write_double_register(DoubleRegister::HL, address + 1);

                Instruction::LoadAccumulatorToHlAndIncrement {
                    phase: TwoPhases::Second,
                }
            }
            Instruction::LoadAccumulatorToHlAndIncrement {
                phase: TwoPhases::Second,
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
    use crate::cpu::instruction::{ThreePhases, TwoPhases};
    use crate::cpu::{DoubleRegister, Register};
    use crate::debug_memory::DebugMemory;
    use crate::memory_device::MemoryDevice;

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
            phase: TwoPhases::First,
        };

        let instruction = instruction.execute(&mut cpu, &mut memory);

        assert!(matches!(
            instruction,
            Instruction::LoadImmediateToRegister {
                phase: TwoPhases::Second,
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
            phase: TwoPhases::First,
        };

        cpu.write_double_register(DoubleRegister::HL, 3);

        let instruction = instruction.execute(&mut cpu, &mut memory);

        assert!(matches!(
            instruction,
            Instruction::LoadFromHlToRegister {
                phase: TwoPhases::Second,
                destination: Register::B,
            }
        ));

        instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cpu.read_register(Register::B), 42);
        assert_eq!(cpu.read_register(Register::A), 0);
    }

    #[test]
    fn load_accumulator_to_immediate_offset_works() {
        // Write 42 to A and then copy A to C
        let mut cpu = CpuState::new();
        let mut memory = DebugMemory::new_with_init(&[03]);
        cpu.write_register(Register::A, 42);

        let instruction = Instruction::LoadAccumulatorToImmediateOffset {
            offset: 0,
            phase: ThreePhases::First,
        };

        let instruction = instruction.execute(&mut cpu, &mut memory);

        assert!(matches!(
            instruction,
            Instruction::LoadAccumulatorToImmediateOffset {
                offset: 3,
                phase: ThreePhases::Second,
            }
        ));
        let instruction = instruction.execute(&mut cpu, &mut memory);

        assert!(matches!(
            instruction,
            Instruction::LoadAccumulatorToImmediateOffset {
                offset: _,
                phase: ThreePhases::Third,
            }
        ));

        instruction.execute(&mut cpu, &mut memory);

        assert_eq!(memory.read(0xFF03), 42);
    }

    #[test]
    fn load_from_immediate_offset_to_accumulator_works() {
        // Write 42 to A and then copy A to C
        let mut cpu = CpuState::new();
        let mut memory = DebugMemory::new_with_init(&[03]);
        memory.write(0xFF03, 42);

        let instruction = Instruction::LoadFromImmediateOffsetToAccumulator {
            offset: 0,
            phase: ThreePhases::First,
        };

        let instruction = instruction.execute(&mut cpu, &mut memory);

        assert!(matches!(
            instruction,
            Instruction::LoadFromImmediateOffsetToAccumulator {
                offset: 3,
                phase: ThreePhases::Second,
            }
        ));
        let instruction = instruction.execute(&mut cpu, &mut memory);

        assert!(matches!(
            instruction,
            Instruction::LoadFromImmediateOffsetToAccumulator {
                offset: _,
                phase: ThreePhases::Third,
            }
        ));

        instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cpu.read_register(Register::A), 42);
    }

    #[test]
    fn load_hl_to_accumulator_and_decrement_works() {
        // Write 42 to A and then copy A to C
        let mut cpu = CpuState::new();
        let mut memory = DebugMemory::new_with_init(&[0, 0, 0, 42]);

        let instruction = Instruction::LoadHlToAccumulatorAndDecrement {
            phase: TwoPhases::First,
        };

        cpu.write_double_register(DoubleRegister::HL, 3);

        let instruction = instruction.execute(&mut cpu, &mut memory);

        assert!(matches!(
            instruction,
            Instruction::LoadAccumulatorToHlAndDecrement {
                phase: TwoPhases::Second,
            }
        ));

        instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cpu.read_double_register(DoubleRegister::HL), 2);
        assert_eq!(cpu.read_register(Register::A), 42);
    }

    #[test]
    fn load_accumulator_to_hl_and_decrement_works() {
        // Write 42 to A and then copy A to C
        let mut cpu = CpuState::new();
        let mut memory = DebugMemory::new_with_init(&[]);

        let instruction = Instruction::LoadAccumulatorToHlAndDecrement {
            phase: TwoPhases::First,
        };

        cpu.write_register(Register::A, 42);
        cpu.write_double_register(DoubleRegister::HL, 3);

        let instruction = instruction.execute(&mut cpu, &mut memory);

        assert!(matches!(
            instruction,
            Instruction::LoadAccumulatorToHlAndDecrement {
                phase: TwoPhases::Second,
            }
        ));

        instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cpu.read_double_register(DoubleRegister::HL), 2);
        assert_eq!(cpu.read_register(Register::A), 42);
    }

    #[test]
    fn load_hl_to_accumulator_and_increment_works() {
        // Write 42 to A and then copy A to C
        let mut cpu = CpuState::new();
        let mut memory = DebugMemory::new_with_init(&[0, 0, 0, 42]);

        let instruction = Instruction::LoadHlToAccumulatorAndIncrement {
            phase: TwoPhases::First,
        };

        cpu.write_double_register(DoubleRegister::HL, 3);

        let instruction = instruction.execute(&mut cpu, &mut memory);

        assert!(matches!(
            instruction,
            Instruction::LoadAccumulatorToHlAndIncrement {
                phase: TwoPhases::Second,
            }
        ));

        instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cpu.read_double_register(DoubleRegister::HL), 4);
        assert_eq!(cpu.read_register(Register::A), 42);
    }

    #[test]
    fn load_accumulator_to_hl_and_increment_works() {
        // Write 42 to A and then copy A to C
        let mut cpu = CpuState::new();
        let mut memory = DebugMemory::new_with_init(&[]);

        let instruction = Instruction::LoadAccumulatorToHlAndIncrement {
            phase: TwoPhases::First,
        };

        cpu.write_register(Register::A, 42);
        cpu.write_double_register(DoubleRegister::HL, 3);

        let instruction = instruction.execute(&mut cpu, &mut memory);

        assert!(matches!(
            instruction,
            Instruction::LoadAccumulatorToHlAndIncrement {
                phase: TwoPhases::Second,
            }
        ));

        instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cpu.read_double_register(DoubleRegister::HL), 4);
        assert_eq!(cpu.read_register(Register::A), 42);
    }
}
