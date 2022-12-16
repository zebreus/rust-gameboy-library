use super::Instruction;
use crate::{cpu::Cpu, memory_device::MemoryDevice};

/// Halt the execution until the next interrupt.
///
/// This is achieved by returning Halt instructions until a interrupt is pending
///
// TODO: The halt instruction on gameboy apparently has some weird bug that is not implemented for now.
// TODO: It also has slightly different behaviour than this, but I did not understand what exactly is different. See https://gbdev.io/pandocs/halt.html
pub struct Halt {}

impl Instruction for Halt {
    fn execute<T: MemoryDevice>(
        &self,
        cpu: &mut crate::cpu::CpuState,
        _memory: &mut T,
    ) -> super::InstructionEnum {
        let interrupt = cpu.get_pending_interrupt();
        match interrupt {
            Some(instruction) => instruction,
            None => (Self {}).into(),
        }
    }
    fn encode(&self) -> Vec<u8> {
        Vec::from([0b01110110])
    }
}

#[cfg(test)]
mod tests {
    use super::Halt;
    use crate::cpu::instruction::{Instruction, InstructionEnum};
    use crate::cpu::{Cpu, CpuState, Interrupt};
    use crate::debug_memory::DebugMemory;

    #[test]
    fn halt_works() {
        let mut cpu = CpuState::new();
        let mut memory = DebugMemory::new();

        cpu.write_interrupt_master_enable(false);

        let instruction = Halt {};

        let instruction = instruction.execute(&mut cpu, &mut memory);
        let instruction = instruction.execute(&mut cpu, &mut memory);
        let instruction = instruction.execute(&mut cpu, &mut memory);
        let instruction = instruction.execute(&mut cpu, &mut memory);
        let instruction = instruction.execute(&mut cpu, &mut memory);
        let instruction = instruction.execute(&mut cpu, &mut memory);

        assert!(matches!(instruction, InstructionEnum::Halt(Halt {})));

        cpu.write_interrupt_master_enable(true);
        cpu.write_interrupt_enable(Interrupt::VBlank, true);
        cpu.write_interrupt_flag(Interrupt::VBlank, true);

        let instruction = instruction.execute(&mut cpu, &mut memory);

        assert!(matches!(
            instruction,
            InstructionEnum::InterruptServiceRoutine(_)
        ));

        assert_eq!(cpu.read_interrupt_master_enable(), true);
    }
}
