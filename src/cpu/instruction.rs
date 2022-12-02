use crate::memory_device::MemoryDevice;

use super::Cpu;
use super::CpuState;

use super::DoubleRegister;
use super::Register;

pub mod decode;
pub mod encode;
pub mod execute;

use decode::decode;

#[derive(Debug)]
pub enum TwoPhases {
    First,
    Second,
}

#[derive(Debug)]
pub enum ThreePhases {
    First,
    Second,
    Third,
}

#[derive(Debug)]
pub enum FourPhases {
    First,
    Second,
    Third,
    Fourth,
}

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
    LoadAccumulatorToHlAndIncrement {
        phase: TwoPhases,
    },
    None,
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
    use super::Cpu;
    use super::CpuState;
    use crate::cpu::instruction::load_instruction;
    use crate::cpu::Register;
    use crate::debug_memory::DebugMemory;

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
