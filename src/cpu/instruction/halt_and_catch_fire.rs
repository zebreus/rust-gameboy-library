use super::Instruction;
use crate::memory::MemoryDevice;

/// Illegal instruction. Lock up cpu.
#[doc(alias = "HCF")]
pub struct HaltAndCatchFire {
    /// The opcode that triggered this.
    pub opcode: u8,
}
impl Instruction for HaltAndCatchFire {
    fn execute<T: MemoryDevice>(
        &self,
        _cpu: &mut crate::cpu::CpuState,
        _memory: &mut T,
    ) -> super::InstructionEnum {
        #[cfg(test)]
        println!(
            "Encountered illegal opcode {:#010b}. Entering endless loop.",
            self.opcode
        );
        return Self {
            opcode: self.opcode,
        }
        .into();
    }
    fn encode(&self) -> Vec<u8> {
        Vec::from([self.opcode])
    }
}

#[cfg(test)]
mod tests {
    use super::HaltAndCatchFire;
    use crate::cpu::instruction::{Instruction, InstructionEnum};
    use crate::cpu::CpuState;
    use crate::debug_memory::DebugMemory;

    #[test]
    fn instruction_works() {
        let mut cpu = CpuState::new();
        let mut memory = DebugMemory::new_with_init(&[1, 1, 1, 1]);

        let instruction = HaltAndCatchFire { opcode: 0b11110100 };

        let instruction = instruction.execute(&mut cpu, &mut memory);
        let instruction = instruction.execute(&mut cpu, &mut memory);
        let instruction = instruction.execute(&mut cpu, &mut memory);
        let instruction = instruction.execute(&mut cpu, &mut memory);
        let instruction = instruction.execute(&mut cpu, &mut memory);
        let instruction = instruction.execute(&mut cpu, &mut memory);

        assert!(matches!(
            instruction,
            InstructionEnum::HaltAndCatchFire(HaltAndCatchFire { opcode: 0b11110100 })
        ));
    }
}
