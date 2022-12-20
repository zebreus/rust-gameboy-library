use super::{decode_cb, Instruction};
use crate::{cpu::Cpu, memory_device::MemoryDevice};

/// Decode next opcode with [decode_cb](super::decode_cb)
///
/// Most bit operations have opcodes prefixed by `0xCB`.
#[doc(alias = "CB")]
pub struct PrefixCb {}
impl Instruction for PrefixCb {
    fn execute<T: MemoryDevice>(
        &self,
        cpu: &mut crate::cpu::CpuState,
        memory: &mut T,
    ) -> super::InstructionEnum {
        let program_counter = cpu.advance_program_counter();
        let opcode = memory.read(program_counter);
        let instruction = decode_cb(opcode);
        return instruction;
    }
    fn encode(&self) -> Vec<u8> {
        Vec::from([0xCB])
    }
}

#[cfg(test)]
mod tests {
    use super::PrefixCb;
    use crate::cpu::instruction::Instruction;
    use crate::cpu::CpuState;
    use crate::debug_memory::DebugMemory;

    #[test]
    fn instruction_works() {
        let mut cpu = CpuState::new();
        let mut memory = DebugMemory::new_with_init(&[1, 1, 1, 1]);

        let instruction = PrefixCb {};

        instruction.execute(&mut cpu, &mut memory);
    }
}
