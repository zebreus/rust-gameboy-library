use super::Instruction;
use crate::{cpu::Cpu, memory_device::MemoryDevice};

/// Disable interrupts now.
///
/// Sets IME to false.
///
/// Cancels pending [EnableInterrupts](super::EnableInterrupts).
/// In our case we dont need to do anything special for that, as that happens implicitly.
#[doc(alias = "DI")]
pub struct DisableInterrupts {}

impl Instruction for DisableInterrupts {
    fn execute<T: MemoryDevice>(
        &self,
        cpu: &mut crate::cpu::CpuState,
        memory: &mut T,
    ) -> super::InstructionEnum {
        cpu.write_interrupt_master_enable(false);
        return cpu.load_instruction(memory);
    }
    fn encode(&self) -> Vec<u8> {
        Vec::from([0b11110011])
    }
}

#[cfg(test)]
mod tests {
    use super::DisableInterrupts;
    use crate::cpu::instruction::Instruction;
    use crate::cpu::{Cpu, CpuState};
    use crate::debug_memory::DebugMemory;

    #[test]
    fn disable_interrupts_works() {
        let mut cpu = CpuState::new();
        let mut memory = DebugMemory::new();

        cpu.write_interrupt_master_enable(true);

        let instruction = DisableInterrupts {};

        instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cpu.read_interrupt_master_enable(), false);
    }
}
