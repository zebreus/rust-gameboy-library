use super::Instruction;
use crate::{
    cpu::{Cpu, DoubleRegister},
    memory::MemoryDevice,
};

/// Jumps to the address stored in [DoubleRegister::HL].
#[doc(alias = "JP")]
#[derive(Debug)]
pub struct JumpToHl {}

impl Instruction for JumpToHl {
    fn execute<T: MemoryDevice>(
        &self,
        cpu: &mut crate::cpu::CpuState,
        memory: &mut T,
    ) -> super::InstructionEnum {
        let address = cpu.read_double_register(DoubleRegister::HL);
        cpu.write_program_counter(address);

        return cpu.load_instruction(memory);
    }
    fn encode(&self) -> Vec<u8> {
        Vec::from([0b11101001])
    }
}

#[cfg(test)]
mod tests {
    use super::Instruction;
    use super::JumpToHl;
    use crate::cpu::{Cpu, CpuState, DoubleRegister};
    use crate::memory::Memory;

    #[test]
    fn jump_to_hl_works() {
        let mut cpu = CpuState::new();
        let mut memory = Memory::new_for_tests();
        cpu.write_double_register(DoubleRegister::HL, 0x1234);

        let instruction = JumpToHl {};

        instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cpu.read_program_counter(), 0x1234);
    }
}
