use super::{phases::ThreePhases, Instruction};
use crate::{
    cpu::{Cpu, DoubleRegister},
    memory::MemoryDevice,
};

/// Stores the byte following the opcode to the address specified in [HL](DoubleRegister::HL).
#[doc(alias = "LD")]
#[doc(alias = "LD (HL),n")]
#[derive(Debug)]
pub struct LoadImmediateToHl {
    /// The immediate value. Only valid after the first phase.
    pub value: u8,
    /// The current phase of the instruction.
    pub phase: ThreePhases,
}

impl Instruction for LoadImmediateToHl {
    fn execute<T: MemoryDevice>(
        &self,
        cpu: &mut crate::cpu::CpuState,
        memory: &mut T,
    ) -> super::InstructionEnum {
        match self.phase {
            ThreePhases::First => {
                let program_counter = cpu.advance_program_counter();
                let data = memory.read(program_counter);

                Self {
                    value: data,
                    phase: ThreePhases::Second,
                }
                .into()
            }
            ThreePhases::Second => {
                let address = cpu.read_double_register(DoubleRegister::HL);
                memory.write(address, self.value);

                Self {
                    value: self.value,
                    phase: ThreePhases::Third,
                }
                .into()
            }
            ThreePhases::Third => cpu.load_instruction(memory),
        }
    }
    fn encode(&self) -> Vec<u8> {
        match self.phase {
            ThreePhases::First => Vec::from([0b00110110]),
            _ => Vec::from([0b00110110, self.value]),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LoadImmediateToHl;
    use crate::cpu::instruction::phases::ThreePhases;
    use crate::cpu::instruction::{Instruction, InstructionEnum};
    use crate::cpu::{Cpu, CpuState, DoubleRegister};
    use crate::memory::MemoryController;
    use crate::memory::MemoryDevice;
    #[test]
    fn load_immediate_to_hl_works() {
        let mut cpu = CpuState::new();
        let mut memory = MemoryController::new_with_init(&[42]);
        cpu.write_double_register(DoubleRegister::HL, 0x0003);

        let instruction = LoadImmediateToHl {
            value: 0,
            phase: ThreePhases::First,
        };

        let instruction = instruction.execute(&mut cpu, &mut memory);

        assert!(matches!(
            instruction,
            InstructionEnum::LoadImmediateToHl(LoadImmediateToHl {
                value: 42,
                phase: ThreePhases::Second
            })
        ));

        instruction.execute(&mut cpu, &mut memory);

        assert_eq!(memory.read(3), 42);
    }
}
