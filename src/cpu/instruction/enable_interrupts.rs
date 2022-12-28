use super::Instruction;
use crate::{cpu::Cpu, memory::MemoryDevice};

/// Enable interrupts after the next instruction has finished.
///
/// Sets IME to true.
#[doc(alias = "EI")]
#[derive(Debug)]
pub struct EnableInterrupts {}

impl Instruction for EnableInterrupts {
    fn execute<T: MemoryDevice>(
        &self,
        cpu: &mut crate::cpu::CpuState,
        memory: &mut T,
    ) -> super::InstructionEnum {
        // Interrupts are only enabled after the next instruction
        let next_instruction = cpu.load_instruction(memory);
        cpu.write_interrupt_master_enable(true);
        return next_instruction;
    }
    fn encode(&self) -> Vec<u8> {
        Vec::from([0b11111011])
    }
}

#[cfg(test)]
mod tests {
    use super::EnableInterrupts;
    use crate::cpu::instruction::Instruction;
    use crate::cpu::{Cpu, CpuState};
    use crate::memory::Memory;

    #[test]
    fn enable_interrupts_works() {
        let mut cpu = CpuState::new();
        let mut memory = Memory::new_for_tests();

        cpu.write_interrupt_master_enable(false);

        let instruction = EnableInterrupts {};

        instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cpu.read_interrupt_master_enable(), true);
    }
}
