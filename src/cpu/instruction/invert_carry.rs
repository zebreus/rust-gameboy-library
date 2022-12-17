use super::Instruction;
use crate::{
    cpu::{Cpu, Flag},
    memory_device::MemoryDevice,
};

#[doc(alias = "CCF")]
/// Invert the current value of the [Flag::Carry] flag.
///
/// ```
/// # use rust_gameboy_library::cpu::{CpuState, Cpu, Register, Flag};
/// # use rust_gameboy_library::cpu::instruction::InvertCarry;
/// # use rust_gameboy_library::cpu::instruction::Instruction;
/// # use rust_gameboy_library::debug_memory::DebugMemory;
/// #
/// # let mut cpu = CpuState::new();
/// # let mut memory = DebugMemory::new();
/// #
/// cpu.write_flag(Flag::Carry, false);
///
/// let instruction = InvertCarry {};
/// instruction.execute(&mut cpu, &mut memory);
///
/// assert_eq!(cpu.read_flag(Flag::Subtract), false);
/// assert_eq!(cpu.read_flag(Flag::HalfCarry), false);
/// assert_eq!(cpu.read_flag(Flag::Carry), true);
/// ```
pub struct InvertCarry {}

impl Instruction for InvertCarry {
    fn execute<T: MemoryDevice>(
        &self,
        cpu: &mut crate::cpu::CpuState,
        memory: &mut T,
    ) -> super::InstructionEnum {
        cpu.write_flag(Flag::Subtract, false);
        cpu.write_flag(Flag::HalfCarry, false);
        cpu.write_flag(Flag::Carry, !cpu.read_flag(Flag::Carry));

        return cpu.load_instruction(memory);
    }
    fn encode(&self) -> Vec<u8> {
        Vec::from([0b00111111])
    }
}

#[cfg(test)]
mod tests {
    use super::InvertCarry;
    use crate::cpu::instruction::Instruction;
    use crate::cpu::{Cpu, CpuState, Flag};
    use crate::debug_memory::DebugMemory;

    #[test]
    fn invert_carry_works() {
        let mut cpu = CpuState::new();
        let mut memory = DebugMemory::new();

        cpu.write_flag(Flag::Carry, false);
        let instruction = InvertCarry {};
        instruction.execute(&mut cpu, &mut memory);
        assert_eq!(cpu.read_flag(Flag::Subtract), false);
        assert_eq!(cpu.read_flag(Flag::HalfCarry), false);
        assert_eq!(cpu.read_flag(Flag::Carry), true);
    }
}
