use super::Instruction;
use crate::{cpu::Cpu, memory::MemoryDevice};

/// Powers down the CPU and screen until a button is pressed.
///
/// Our current implementation is basically identical to [Halt][super::Halt] but it uses [Cpu::get_pending_stop_wakeup()] instead of [Cpu::get_pending_interrupt()]
#[doc(alias = "STOP")]
pub struct Stop {}

impl Instruction for Stop {
    fn execute<T: MemoryDevice>(
        &self,
        cpu: &mut crate::cpu::CpuState,
        _memory: &mut T,
    ) -> super::InstructionEnum {
        let interrupt = cpu.get_pending_stop_wakeup();
        match interrupt {
            Some(instruction) => instruction,
            None => (Self {}).into(),
        }
    }
    fn encode(&self) -> Vec<u8> {
        Vec::from([0b00010000])
    }
}

#[cfg(test)]
mod tests {
    use super::Stop;
    use crate::cpu::instruction::{Instruction, InstructionEnum};
    use crate::cpu::{Cpu, CpuState, Interrupt};
    use crate::debug_memory::DebugMemory;

    #[test]
    fn stop_works() {
        let mut cpu = CpuState::new();
        let mut memory = DebugMemory::new();

        let instruction = Stop {};

        let instruction = instruction.execute(&mut cpu, &mut memory);
        let instruction = instruction.execute(&mut cpu, &mut memory);
        let instruction = instruction.execute(&mut cpu, &mut memory);
        let instruction = instruction.execute(&mut cpu, &mut memory);
        let instruction = instruction.execute(&mut cpu, &mut memory);
        let instruction = instruction.execute(&mut cpu, &mut memory);

        assert!(matches!(instruction, InstructionEnum::Stop(Stop {})));

        cpu.write_interrupt_enable(Interrupt::Joypad, true);
        cpu.write_interrupt_flag(Interrupt::Joypad, true);

        let instruction = instruction.execute(&mut cpu, &mut memory);

        assert!(matches!(
            instruction,
            InstructionEnum::InterruptServiceRoutine(_)
        ));
    }
}
