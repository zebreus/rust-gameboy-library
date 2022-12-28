use super::Instruction;
use crate::{cpu::Register, memory::MemoryDevice};

/// Copy data from one register to another one.
///
/// Cannot be used with [Register::F]
#[doc(alias = "LD")]
#[doc(alias = "LD R,R")]
#[derive(Debug)]
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
        if (matches!(self.source, Register::B)) && matches!(self.destination, Register::B) {
            println!("Encountered breakpoint instruction LD B,B");
        }
        cpu.registers[self.destination as usize] = cpu.registers[self.source as usize];
        return cpu.load_instruction(memory);
    }
    fn encode(&self) -> Vec<u8> {
        if matches!(self.destination, Register::F) {
            panic!(
                "Cannot encode load from register to register for destination register Register::F"
            )
        }
        if matches!(self.source, Register::F) {
            panic!("Cannot encode load from register to register for source register Register::F")
        }
        let base_code = 0b01000000 & 0b11000000u8;
        let source_code = self.source.id() & 0b00000111u8;
        let destination_code = (self.destination.id() << 3) & 0b00111000u8;
        let opcode = base_code | source_code | destination_code;
        Vec::from([opcode])
    }
}

#[cfg(test)]
mod tests {
    use super::LoadFromRegisterToRegister;
    use crate::cpu::instruction::Instruction;
    use crate::cpu::{Cpu, CpuState, Register};
    use crate::memory::Memory;

    #[test]
    fn load_instruction_works() {
        let mut cpu = CpuState::new();
        cpu.write_register(Register::C, 0);
        cpu.write_register(Register::A, 100);

        let mut memory = Memory::new_for_tests();
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
