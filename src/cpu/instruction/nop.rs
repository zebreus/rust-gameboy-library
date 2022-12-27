use super::Instruction;
use crate::memory::MemoryDevice;

/// Do nothing and load the next instruction.
#[doc(alias = "NOP")]
#[derive(Debug)]
pub struct Nop {}
impl Instruction for Nop {
    fn execute<T: MemoryDevice>(
        &self,
        cpu: &mut crate::cpu::CpuState,
        memory: &mut T,
    ) -> super::InstructionEnum {
        cpu.load_instruction(memory)
    }
    fn encode(&self) -> Vec<u8> {
        Vec::from([0b00000000])
    }
}

#[cfg(test)]
mod tests {
    use super::Nop;
    use crate::cpu::instruction::{Instruction, InstructionEnum};
    use crate::cpu::CpuState;
    use crate::memory::Memory;

    #[test]
    fn nop_works() {
        let mut cpu = CpuState::new();
        let mut memory = Memory::new_with_init(&[1, 1, 1, 1]);

        let instruction = Nop {};

        let instruction = instruction.execute(&mut cpu, &mut memory);

        assert!(!matches!(instruction, InstructionEnum::Nop(Nop {})));
    }
}
