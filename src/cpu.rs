mod instruction;

use crate::memory_device::MemoryDevice;
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

pub trait Cpu {
    fn process_cycle(&mut self, memory: &mut dyn MemoryDevice) -> ();
    fn read_program_counter(&mut self) -> u16;
    fn read_register(&self, register: Register) -> u8;
    fn write_register(&mut self, register: Register, value: u8) -> ();
    fn read_double_register(&self, register: DoubleRegister) -> u16;
    fn write_double_register(&mut self, register: DoubleRegister, value: u16) -> ();
}

impl Cpu for CpuState {
    fn process_cycle(&mut self, memory: &mut dyn MemoryDevice) -> () {
        memory.read(7);
    }
    fn read_program_counter(&mut self) -> u16 {
        let result = self.program_counter;
        self.program_counter += 1;
        return result;
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
        let value: u16 = high << 8 | low;
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
pub enum Register {
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
pub enum DoubleRegister {
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

#[cfg(test)]
mod tests {
    use super::instruction::{load_instruction, Instruction};
    use super::Cpu;
    use super::{CpuState, DoubleRegister};
    use crate::cpu::instruction::load_opcode;
    use crate::cpu::Register;
    use crate::debug_memory::DebugMemory;

    #[test]
    fn read_double_register() {
        let mut cpu = CpuState::new();
        cpu.write_register(Register::B, 1);
        cpu.write_register(Register::C, 3);
        let double_value = cpu.read_double_register(DoubleRegister::BC);
        assert_eq!(double_value, 259)
    }

    #[test]
    fn write_double_register() {
        let mut cpu = CpuState::new();
        cpu.write_double_register(DoubleRegister::BC, 259);
        assert_eq!(cpu.read_register(Register::B), 1);
        assert_eq!(cpu.read_register(Register::C), 3);
    }

    #[test]
    fn write_read_double_register() {
        let mut cpu = CpuState::new();
        cpu.write_double_register(DoubleRegister::BC, 9874);
        assert_eq!(cpu.read_double_register(DoubleRegister::BC), 9874);
    }

    #[test]
    fn cpu_read_program_counter_works() {
        let mut cpu = CpuState::new();

        let memory = DebugMemory::new_with_init(&[0b01000010u8, 8]);
        let opcode = load_opcode(&mut cpu, &memory);

        assert_eq!(opcode, 0b01000010u8);

        let opcode = load_opcode(&mut cpu, &memory);
        assert_eq!(opcode, 8);
    }

    #[test]
    fn cpu_can_fetch_and_decode_instructions() {
        let mut cpu = CpuState::new();
        cpu.write_register(Register::A, 100);

        let memory = DebugMemory::new_with_init(&[0b01000010u8]);
        let instruction = load_instruction(&mut cpu, &memory);
        assert!(matches!(
            instruction,
            Instruction::LoadFromRegisterToRegister {
                source: Register::A,
                destination: Register::C
            }
        ))
    }
}
