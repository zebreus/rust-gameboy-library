use crate::memory_device::MemoryDevice;
use arr_macro::arr;
use bitmatch::bitmatch;
use num_enum::{IntoPrimitive, TryFromPrimitive};

struct CpuState {
    acc: u8, // Not sure if this is a register
    program_counter: u16,
    stack_pointer: u16,
    registers: [u8; 8],
}

impl CpuState {
    fn new() -> Self {
        Self {
            acc: 0, // Not sure if this is a register
            program_counter: 0,
            stack_pointer: 300,
            registers: [0, 0, 0, 0, 0, 0, 0, 0],
        }
    }
}

trait Cpu {
    fn process_cycle(&mut self, memory: &mut dyn MemoryDevice) -> ();
    fn read_program_counter(&mut self, memory: &mut dyn MemoryDevice) -> u8;
    fn load_instruction(&mut self, memory: &mut dyn MemoryDevice) -> Instruction;
    fn read_register(&self, register: Register) -> u8;
    fn write_register(&mut self, register: Register, value: u8) -> ();
    fn read_double_register(&self, register: DoubleRegister) -> u16;
    fn write_double_register(&mut self, register: DoubleRegister, value: u16) -> ();
}

impl Cpu for CpuState {
    fn process_cycle(&mut self, memory: &mut dyn MemoryDevice) -> () {
        memory.read(7);
    }
    fn read_program_counter(&mut self, memory: &mut dyn MemoryDevice) -> u8 {
        let opcode = memory.read(self.program_counter);
        self.program_counter = self.program_counter + 1;
        return opcode;
    }
    fn load_instruction(&mut self, memory: &mut dyn MemoryDevice) -> Instruction {
        let opcode = memory.read(self.program_counter);
        self.program_counter = self.program_counter + 1;
        return decode(opcode);
    }
    fn read_register(&self, register: Register) -> u8 {
        let index = register as usize;
        return self.registers[index];
    }
    fn write_register(&mut self, register: Register, value: u8) -> () {
        let index = register as usize;

        if let Register::F = register {
            // You cannot write bit 0-3 on the flags register
            self.registers[index] = value & 0b11110000;
        }
        self.registers[index] = value;
    }
    fn read_double_register(&self, register: DoubleRegister) -> u16 {
        let registers = register.id();
        let low: u16 = self.read_register(registers.low).into();
        let high: u16 = self.read_register(registers.high).into();
        let value: u16 = high << 8 & low;
        return value;
    }
    fn write_double_register(&mut self, register: DoubleRegister, value: u16) -> () {
        let registers = register.id();
        let [high, low] = u16::to_be_bytes(value);
        self.write_register(registers.high, high);
        self.write_register(registers.low, low);
    }
}

#[derive(TryFromPrimitive, Debug, IntoPrimitive, Clone, Copy)]
#[repr(u8)]
enum Register {
    A = 0,
    B = 1,
    C = 2,
    D = 3,
    E = 4,
    /** The flags register. Bits 0-3 are write protected
     */
    F = 5,
    H = 6,
    L = 7,
}

impl Register {
    fn id(&self) -> u8 {
        *self as u8
    }
}

struct RegisterCombination {
    low: Register,
    high: Register,
}

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
enum DoubleRegister {
    AF,
    BC,
    DE,
    HL,
}

impl DoubleRegister {
    fn id(&self) -> RegisterCombination {
        match self {
            DoubleRegister::AF => RegisterCombination {
                high: Register::A,
                low: Register::F,
            },
            DoubleRegister::BC => RegisterCombination {
                high: Register::B,
                low: Register::C,
            },
            DoubleRegister::DE => RegisterCombination {
                high: Register::D,
                low: Register::E,
            },
            DoubleRegister::HL => RegisterCombination {
                high: Register::H,
                low: Register::L,
            },
        }
    }
}

#[derive(Debug)]
enum Instruction {
    LoadFromRegisterToRegister {
        source: Register,
        destination: Register,
    },
    LoadImmediateToRegister {
        destination: Register,
        value: u8,
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
                return cpu.load_instruction(memory);
            }
            Instruction::LoadImmediateToRegister {
                destination,
                value: _,
                phase: 0,
            } => {
                let value = cpu.read_program_counter(memory);
                return Instruction::LoadImmediateToRegister {
                    destination: *destination,
                    value: value,
                    phase: 1,
                };
            }
            Instruction::LoadImmediateToRegister {
                destination,
                value,
                phase: 1,
            } => {
                cpu.write_register(*destination, *value);
                return cpu.load_instruction(memory);
            }
            _ => Instruction::None,
        }
    }
}

#[bitmatch]
fn decode(byte: u8) -> Instruction {
    #[bitmatch]
    // TODO: How can we get rid of this (soon) massive match clause
    match byte {
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
            let baseCode = 0b01000000 & 0b11000000u8;
            let sourceCode = (source.id() << 3) & 0b00111000u8;
            let destinationCode = destination.id() & 0b00000111u8;
            let opcode = baseCode | sourceCode | destinationCode;
            Vec::from([opcode])
        }
        Instruction::LoadImmediateToRegister {
            destination,
            value,
            phase,
        } => {
            let baseCode = 0b00000110 & 0b11000111u8;
            let destinationCode = (destination.id() << 3) & 0b00111000u8;
            let opcode = baseCode | destinationCode;
            match phase {
                0 => Vec::from([opcode]),
                1 => Vec::from([opcode, value]),
                _ => Vec::new(),
            }
        }
        Instruction::None => Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::{decode, encode, CpuState};
    use super::{Cpu, Instruction};
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
    fn cpu_read_program_counter_works() {
        let mut cpu = CpuState::new();

        let mut memory = DebugMemory::new_with_init(&[0b01000010u8, 8]);
        let opcode = cpu.read_program_counter(&mut memory);
        assert_eq!(opcode, 0b01000010u8);

        let opcode = cpu.read_program_counter(&mut memory);
        assert_eq!(opcode, 8);
    }

    #[test]
    fn cpu_can_fetch_and_decode_instructions() {
        let mut cpu = CpuState::new();
        cpu.write_register(Register::A, 100);

        let mut memory = DebugMemory::new_with_init(&[0b01000010u8]);
        let instruction = cpu.load_instruction(&mut memory);
        assert!(matches!(
            instruction,
            Instruction::LoadFromRegisterToRegister {
                source: Register::A,
                destination: Register::C
            }
        ))
    }

    // #[test]
    // fn load_instruction_integration() {
    //     let mut cpu = CpuState::new();
    //     cpu.write_register(Register::A, 100);

    //     let mut memory = DebugMemory::new_with_init(&[0b01000010u8]);

    //     let value_c_before = cpu.read_register(Register::C);
    //     assert_eq!(value_c_before, 0);

    //     instruction.execute(&mut cpu, &mut memory);
    //     let value_c_after = cpu.read_register(Register::C);

    //     assert_eq!(value_c_after, 100);
    // }

    #[test]
    fn load_instruction_integration() {
        // Write 42 to A and then copy A to C
        let mut cpu = CpuState::new();

        let mut memory = DebugMemory::new_with_init(&[0b00000110, 42, 0b01000010u8]);

        let instruction = cpu.load_instruction(&mut memory);

        let instruction = instruction.execute(&mut cpu, &mut memory);
        let instruction = instruction.execute(&mut cpu, &mut memory);
        instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cpu.read_register(Register::A), 42);
        assert_eq!(cpu.read_register(Register::B), 0);
        assert_eq!(cpu.read_register(Register::C), 42);
    }
}
