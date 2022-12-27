use super::Instruction;
use crate::{
    cpu::{Cpu, Flag, Register},
    memory::MemoryDevice,
};

/// Invert the value of the [accumulator][Register::A].
///
/// Always sets [Flag::Subtract] and [Flag::HalfCarry]
///
/// ```
/// # use rust_gameboy_library::cpu::{CpuState, Cpu, Register, Flag};
/// # use rust_gameboy_library::cpu::instruction::Complement;
/// # use rust_gameboy_library::cpu::instruction::Instruction;
/// # use rust_gameboy_library::debug_memory::DebugMemory;
/// #
/// # let mut cpu = CpuState::new();
/// # let mut memory = DebugMemory::new();
/// #
/// cpu.write_register(Register::A, 0b01010101);
///
/// let instruction = Complement {};
///
/// instruction.execute(&mut cpu, &mut memory);
///
/// assert_eq!(cpu.read_register(Register::A), 0b10101010);
/// assert_eq!(cpu.read_flag(Flag::Subtract), true);
/// assert_eq!(cpu.read_flag(Flag::HalfCarry), true);
/// ```
///
/// | [Zero](Flag::Zero)  | [Subtract](Flag::Subtract) | [HalfCarry](Flag::HalfCarry) | [Carry](Flag::Carry) |
/// |---------------------|----------------------------|------------------------------|----------------------|
/// | unchanged           | true                       | true                         | unchanged            |
#[doc(alias = "CPL")]
pub struct Complement {}

impl Instruction for Complement {
    fn execute<T: MemoryDevice>(
        &self,
        cpu: &mut crate::cpu::CpuState,
        memory: &mut T,
    ) -> super::InstructionEnum {
        let original = cpu.read_register(Register::A);
        let complement = !original;
        cpu.write_register(Register::A, complement);

        cpu.write_flag(Flag::Subtract, true);
        cpu.write_flag(Flag::HalfCarry, true);

        return cpu.load_instruction(memory);
    }
    fn encode(&self) -> Vec<u8> {
        Vec::from([0b00101111])
    }
}

#[cfg(test)]
mod tests {
    use super::Complement;
    use crate::cpu::instruction::Instruction;
    use crate::cpu::{Cpu, CpuState, Flag, Register};
    use crate::debug_memory::DebugMemory;

    #[test]
    fn complement_works() {
        let mut cpu = CpuState::new();
        let mut memory = DebugMemory::new();

        cpu.write_register(Register::A, 0b01010101);

        let instruction = Complement {};

        instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cpu.read_register(Register::A), 0b10101010);
        assert_eq!(cpu.read_flag(Flag::Subtract), true);
        assert_eq!(cpu.read_flag(Flag::HalfCarry), true);
    }
}
