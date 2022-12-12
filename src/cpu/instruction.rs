use crate::memory_device::MemoryDevice;

use super::Cpu;
use super::CpuState;

use super::DoubleRegister;
use super::Register;

/// Contains the decode function
mod decode;
/// Contains the encode method
pub mod encode;
/// Contains the execute method
pub mod execute;

pub use decode::decode;

/// The phases of an instruction with two phases
#[derive(Debug)]
pub enum TwoPhases {
    /// First phase
    First,
    /// Second phase
    Second,
}

/// The phases of an instruction with three phases
#[derive(Debug)]
pub enum ThreePhases {
    /// First phase
    First,
    /// Second phase
    Second,
    /// Third phase
    Third,
}

/// The phases of an instruction with four phases
#[derive(Debug)]
pub enum FourPhases {
    /// First phase
    First,
    /// Second phase
    Second,
    /// Third phase
    Third,
    /// Fourth phase
    Fourth,
}

/// Instruction represents all available CPU instructions.
///
/// Each instruction is a struct that contains information on the current state of the instruction.
///
/// Instructions that take longer then one cycle have a `phases` field indicating the current cycle of the instruction.
///
/// Every instruction can be executed using the [execute] function.
#[derive(Debug)]
pub enum Instruction {
    /// Copy data from one register to another one.
    LoadFromRegisterToRegister {
        /// The source register
        source: Register,
        /// The destination register
        destination: Register,
    },
    /// Loads the byte following the opcode of the instruction to a register
    LoadImmediateToRegister {
        /// The destination register.
        destination: Register,
        /// The immediate value. Will only valid in the second phase.
        value: u8,
        /// The current phase of the instruction.
        phase: TwoPhases,
    },
    /// Loads from memory at the address stored in [DoubleRegister::HL] to a register.
    LoadFromHlToRegister {
        /// The destination register.
        destination: Register,
        /// The current phase of the instruction.
        phase: TwoPhases,
    },
    /// Stores the value of the [accumulator](Register::A) to memory at `0xff00 + the byte following the opcode` .
    LoadAccumulatorToImmediateOffset {
        /// The memory address offset from 0xff00. Only valid after the first phase.
        offset: u8,
        /// The current phase of the instruction.
        phase: ThreePhases,
    },
    /// Loads from memory at `0xff00 + the byte following the opcode` into the [accumulator](Register::A).
    LoadFromImmediateOffsetToAccumulator {
        /// The memory address offset from 0xff00. Only valid after the first phase.
        offset: u8,
        /// The current phase of the instruction.
        phase: ThreePhases,
    },
    /// Loads from memory at the address specified in [HL](DoubleRegister::HL) to the [accumulator](Register::A). Decrements [HL](DoubleRegister::HL) afterwards.
    LoadHlToAccumulatorAndDecrement {
        /// The current phase of the instruction.
        phase: TwoPhases,
    },
    /// Stores the [accumulator](Register::A) to the address specified in [HL](DoubleRegister::HL). Decrements [HL](DoubleRegister::HL) afterwards.
    LoadAccumulatorToHlAndDecrement {
        /// The current phase of the instruction.
        phase: TwoPhases,
    },
    /// Loads from memory at the address specified in [HL](DoubleRegister::HL) to the [accumulator](Register::A). Increments [HL](DoubleRegister::HL) afterwards.
    LoadHlToAccumulatorAndIncrement {
        /// The current phase of the instruction.
        phase: TwoPhases,
    },
    /// Stores the [accumulator](Register::A) to the address specified in [HL](DoubleRegister::HL). Increments [HL](DoubleRegister::HL) afterwards.
    LoadAccumulatorToHlAndIncrement {
        /// The current phase of the instruction.
        phase: TwoPhases,
    },
    /// Noop
    None,
}

/// Load the next opcode
///
/// Also increments the program counter
pub fn load_opcode<T: Cpu>(cpu: &mut T, memory: &dyn MemoryDevice) -> u8 {
    let opcode = memory.read(cpu.read_program_counter());
    return opcode;
}

/// Load the next [Instruction]
///
/// Also increments the program counter
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
