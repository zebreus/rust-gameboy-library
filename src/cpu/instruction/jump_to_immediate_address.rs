use super::phases::FourPhases;
use super::Instruction;
use crate::{cpu::Cpu, memory::MemoryDevice};

/// Jumps to the address specified in the two bytes following the opcode
#[doc(alias = "JP")]
#[derive(Debug)]
pub struct JumpToImmediateAddress {
    /// The immediate address. Will only valid after the second phase.
    pub address: u16,
    /// The current phase of the instruction.
    pub phase: FourPhases,
}

impl Instruction for JumpToImmediateAddress {
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
                let address_msb = memory.read(program_counter);

                Self {
                    phase: FourPhases::Third,
                    address: u16::from_le_bytes([self.address as u8, address_msb]),
                }
                .into()
            }
            FourPhases::Third => {
                cpu.write_program_counter(self.address);

                Self {
                    phase: FourPhases::Fourth,
                    address: self.address,
                }
                .into()
            }
            FourPhases::Fourth => {
                return cpu.load_instruction(memory);
            }
        }
    }
    fn encode(&self) -> Vec<u8> {
        match self.phase {
            FourPhases::First => Vec::from([0b11000011]),
            FourPhases::Second => Vec::from([0b11000011, self.address.to_le_bytes()[0]]),
            _ => Vec::from([
                0b11000011,
                self.address.to_le_bytes()[0],
                self.address.to_le_bytes()[1],
            ]),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::JumpToImmediateAddress;
    use crate::cpu::instruction::phases::FourPhases;
    use crate::cpu::instruction::{Instruction, InstructionEnum};
    use crate::cpu::{Cpu, CpuState};
    use crate::memory::MemoryController;

    #[test]
    fn jump_by_immediate_address_works() {
        // Write 42 to A and then copy A to C
        let mut cpu = CpuState::new();
        let mut memory = MemoryController::new_with_init(&[0x34, 0x12]);

        let instruction = JumpToImmediateAddress {
            address: 0,
            phase: FourPhases::First,
        };

        let instruction = instruction.execute(&mut cpu, &mut memory);
        let instruction = instruction.execute(&mut cpu, &mut memory);

        assert!(matches!(
            instruction,
            InstructionEnum::JumpToImmediateAddress(JumpToImmediateAddress {
                phase: FourPhases::Third,
                address: 0x1234
            })
        ));

        let instruction = instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cpu.read_program_counter(), 0x1234);

        assert!(matches!(
            instruction,
            InstructionEnum::JumpToImmediateAddress(JumpToImmediateAddress {
                phase: FourPhases::Fourth,
                address: 0x1234
            })
        ));

        assert_eq!(cpu.read_program_counter(), 0x1234);
    }

    #[test]
    fn encode_jump_by_immediate_address() {
        // Write 42 to A and then copy A to C
        let mut cpu = CpuState::new();
        let mut memory = MemoryController::new_with_init(&[0x34, 0x12]);

        let instruction = JumpToImmediateAddress {
            address: 0,
            phase: FourPhases::First,
        };

        let instruction = instruction.execute(&mut cpu, &mut memory);
        let instruction = instruction.execute(&mut cpu, &mut memory);

        let encoded = instruction.encode();

        assert_eq!(encoded[0], 0b11000011);
        assert_eq!(encoded[1], 0x34);
        assert_eq!(encoded[2], 0x12);
    }
}
