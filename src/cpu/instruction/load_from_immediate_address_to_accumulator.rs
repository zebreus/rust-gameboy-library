use super::{phases::FourPhases, Instruction};
use crate::{
    cpu::{Cpu, Register},
    memory::MemoryDevice,
};

/// Loads from memory at `the second byte following the opcode + the byte following the opcode` into the [accumulator](Register::A).
///
/// Reads from program counter: `opcode` `address lsb` `address msb`
#[doc(alias = "LD")]
#[doc(alias = "LD A,(nn)")]
pub struct LoadFromImmediateAddressToAccumulator {
    /// The memory address. Only valid after the second phase.
    pub address: u16,
    /// The current phase of the instruction.
    pub phase: FourPhases,
}

impl Instruction for LoadFromImmediateAddressToAccumulator {
    fn execute<T: MemoryDevice>(
        &self,
        cpu: &mut crate::cpu::CpuState,
        memory: &mut T,
    ) -> super::InstructionEnum {
        match self.phase {
            FourPhases::First => {
                let program_counter = cpu.advance_program_counter();
                let address_lsb = memory.read(program_counter);

                Self {
                    phase: FourPhases::Second,
                    address: address_lsb as u16,
                }
                .into()
            }
            FourPhases::Second => {
                let program_counter = cpu.advance_program_counter();
                let address_msb = memory.read(program_counter) as u16;

                Self {
                    phase: FourPhases::Third,
                    address: self.address | ((address_msb) << 8),
                }
                .into()
            }
            FourPhases::Third => {
                let data = memory.read(self.address);
                cpu.write_register(Register::A, data);

                Self {
                    phase: FourPhases::Fourth,
                    address: self.address,
                }
                .into()
            }
            FourPhases::Fourth => cpu.load_instruction(memory),
        }
    }
    fn encode(&self) -> Vec<u8> {
        match self.phase {
            FourPhases::First => Vec::from([0b11111010]),
            FourPhases::Second => Vec::from([0b11111010, self.address.to_le_bytes()[0]]),
            _ => Vec::from([
                0b11111010,
                self.address.to_le_bytes()[0],
                self.address.to_le_bytes()[1],
            ]),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LoadFromImmediateAddressToAccumulator;
    use crate::cpu::instruction::phases::FourPhases;
    use crate::cpu::instruction::{Instruction, InstructionEnum};
    use crate::cpu::{Cpu, CpuState, Register};
    use crate::memory::Memory;
    use crate::memory::MemoryDevice;

    #[test]
    fn load_from_immediate_address_to_accumulator_works() {
        // Write 42 to A and then copy A to C
        let mut cpu = CpuState::new();
        let mut memory = Memory::new_with_init(&[0x03, 0xFF]);
        memory.write(0xff03, 42);

        let instruction = LoadFromImmediateAddressToAccumulator {
            address: 0,
            phase: FourPhases::First,
        };

        let instruction = instruction.execute(&mut cpu, &mut memory);
        let instruction = instruction.execute(&mut cpu, &mut memory);

        assert!(matches!(
            instruction,
            InstructionEnum::LoadFromImmediateAddressToAccumulator(
                LoadFromImmediateAddressToAccumulator {
                    address: 0xFF03,
                    phase: FourPhases::Third,
                }
            )
        ));
        let instruction = instruction.execute(&mut cpu, &mut memory);

        assert!(matches!(
            instruction,
            InstructionEnum::LoadFromImmediateAddressToAccumulator(
                LoadFromImmediateAddressToAccumulator {
                    address: _,
                    phase: FourPhases::Fourth,
                }
            )
        ));

        instruction.execute(&mut cpu, &mut memory);

        assert_eq!(memory.read(0xFF03), 42);
        assert_eq!(cpu.read_register(Register::A), 42);
    }
}
