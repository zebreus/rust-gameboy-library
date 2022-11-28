use crate::memory_device::MemoryDevice;
use arr_macro::arr;
use bitmatch::bitmatch;
use num_enum::{IntoPrimitive, TryFromPrimitive};

struct CpuState {
    acc: u8, // Not sure if this is a register
    program_counter: u16,
    stack_pointer: u16,
    registers: [u8; 8],
    flags: u8,
}

impl CpuState {
    fn new() -> Self {
        Self {
            acc: 0, // Not sure if this is a register
            program_counter: 0,
            stack_pointer: 300,
            registers: [0, 0, 0, 0, 0, 0, 0, 0],
            flags: 0,
        }
    }
}

trait Cpu {
    fn process_cycle(&mut self, memory: &mut dyn MemoryDevice) -> ();
    fn load_instruction(&mut self, memory: &mut dyn MemoryDevice) -> Instruction;
    fn read_register(&self, register: Register) -> u8;
    fn write_register(&mut self, register: Register, value: u8) -> ();
}

impl Cpu for CpuState {
    fn process_cycle(&mut self, memory: &mut dyn MemoryDevice) -> () {
        memory.read(7);
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
        self.registers[index] = value;
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
    F = 5,
    G = 6,
    H = 7,
}

#[derive(Debug)]
enum Instruction {
    LoadFromRegisterToRegister {
        source: Register,
        destination: Register,
    },
    None,
}

impl Instruction {
    fn execute(&self, cpu: &mut CpuState, memory: &mut dyn MemoryDevice) -> () {
        match self {
            Instruction::LoadFromRegisterToRegister {
                source,
                destination,
            } => {
                cpu.registers[*destination as usize] = cpu.registers[*source as usize];
            }
            _ => (),
        }
    }
}

#[bitmatch]
fn decode(byte: u8) -> Instruction {
    #[bitmatch]
    match byte {
        "01aaabbb" => Instruction::LoadFromRegisterToRegister {
            source: Register::try_from(a)
                .expect("3 bit value should always correspont to a register"),
            destination: Register::try_from(b)
                .expect("3 bit value should always correspont to a register"),
        },
        _ => Instruction::None {},
    }
}

#[cfg(test)]
mod tests {

    use super::{decode, CpuState};
    use super::{Cpu, Instruction};
    use crate::cpu::Register;
    use crate::debug_memory::DebugMemory;

    #[test]
    fn decodes_load_instruction() {
        let load_a_to_c = 0b01000010u8;
        let instruction = decode(load_a_to_c);
        assert!(matches!(
            instruction,
            Instruction::LoadFromRegisterToRegister {
                source,
                destination
            } if matches!(source, Register::A) && matches!(destination, Register::C)
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
}
