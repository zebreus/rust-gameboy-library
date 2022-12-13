use super::Instruction;
use crate::{cpu::Register, memory_device::MemoryDevice};

/// Copy data from one register to another one.
pub struct LoadFromRegisterToRegister {
    /// The source register
    pub source: Register,
    /// The destination register
    pub destination: Register,
}

impl Instruction for LoadFromRegisterToRegister {
    fn execute<T: MemoryDevice>(
        &self,
        cpu: &mut crate::cpu::CpuState,
        memory: &mut T,
    ) -> super::InstructionEnum {
        cpu.registers[self.destination as usize] = cpu.registers[self.source as usize];
        return cpu.load_instruction(memory);
    }
    fn encode(&self) -> Vec<u8> {
        let base_code = 0b01000000 & 0b11000000u8;
        let source_code = (self.source.id() << 3) & 0b00111000u8;
        let destination_code = self.destination.id() & 0b00000111u8;
        let opcode = base_code | source_code | destination_code;
        Vec::from([opcode])
    }
}

#[cfg(test)]
mod tests {
    use super::LoadFromRegisterToRegister;
    use crate::cpu::instruction::Instruction;
    use crate::cpu::{Cpu, CpuState, Register};
    use crate::debug_memory::DebugMemory;

    #[test]
    fn load_instruction_works() {
        let mut cpu = CpuState::new();
        cpu.write_register(Register::A, 100);

        let mut memory = DebugMemory::new();
        let instruction = LoadFromRegisterToRegister {
            source: Register::A,
            destination: Register::C,
        };

        let value_c_before = cpu.read_register(Register::C);
        assert_eq!(value_c_before, 0);

        instruction.execute(&mut cpu, &mut memory);
        let value_c_after = cpu.read_register(Register::C);

        assert_eq!(value_c_after, 100);
    }
}