use super::phases::FivePhases;
use super::Instruction;
use crate::{cpu::Cpu, memory_device::MemoryDevice};

/// Loads the two bytes following the opcode of the instruction to a double register
pub struct LoadSpToImmediateAddress {
    /// The immediate value. Will only valid after the second phase.
    pub address: u16,
    /// The current phase of the instruction.
    pub phase: FivePhases,
}

impl Instruction for LoadSpToImmediateAddress {
    fn execute<T: MemoryDevice>(
        &self,
        cpu: &mut crate::cpu::CpuState,
        memory: &mut T,
    ) -> super::InstructionEnum {
        match self.phase {
            FivePhases::First => {
                let program_counter = cpu.read_program_counter();
                let value_lsb = memory.read(program_counter);

                Self {
                    phase: FivePhases::Second,
                    address: value_lsb as u16,
                }
                .into()
            }
            FivePhases::Second => {
                let program_counter = cpu.read_program_counter();
                let value_msb = memory.read(program_counter) as u16;

                Self {
                    phase: FivePhases::Third,
                    address: self.address | ((value_msb) << 8),
                }
                .into()
            }
            FivePhases::Third => {
                let data = cpu.read_stack_pointer().to_le_bytes()[0];
                memory.write(self.address + 1, data);

                Self {
                    phase: FivePhases::Fourth,
                    address: self.address,
                }
                .into()
            }
            FivePhases::Fourth => {
                let data = cpu.read_stack_pointer().to_le_bytes()[1];
                memory.write(self.address, data);

                Self {
                    phase: FivePhases::Fifth,
                    address: self.address,
                }
                .into()
            }
            FivePhases::Fifth => {
                return cpu.load_instruction(memory);
            }
        }
    }
    fn encode(&self) -> Vec<u8> {
        match self.phase {
            FivePhases::First => Vec::from([0b00001000]),
            FivePhases::Second => Vec::from([0b00001000, self.address.to_le_bytes()[0]]),
            _ => Vec::from([
                0b00001000,
                self.address.to_le_bytes()[0],
                self.address.to_le_bytes()[1],
            ]),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LoadSpToImmediateAddress;
    use crate::cpu::instruction::phases::FivePhases;
    use crate::cpu::instruction::{Instruction, InstructionEnum};
    use crate::cpu::{Cpu, CpuState};
    use crate::debug_memory::DebugMemory;
    use crate::memory_device::MemoryDevice;

    #[test]
    fn load_sp_to_immediate_address_works() {
        // Write 42 to A and then copy A to C
        let mut cpu = CpuState::new();
        let mut memory = DebugMemory::new_with_init(&[0x34, 0x12]);
        cpu.write_stack_pointer(0x5678);

        let instruction = LoadSpToImmediateAddress {
            address: 0,
            phase: FivePhases::First,
        };

        let instruction = instruction.execute(&mut cpu, &mut memory);
        let instruction = instruction.execute(&mut cpu, &mut memory);

        assert!(matches!(
            instruction,
            InstructionEnum::LoadSpToImmediateAddress(LoadSpToImmediateAddress {
                phase: FivePhases::Third,
                address: 0x1234
            })
        ));

        let instruction = instruction.execute(&mut cpu, &mut memory);
        let instruction = instruction.execute(&mut cpu, &mut memory);

        assert!(matches!(
            instruction,
            InstructionEnum::LoadSpToImmediateAddress(LoadSpToImmediateAddress {
                phase: FivePhases::Fifth,
                address: 0x1234
            })
        ));

        assert_eq!(memory.read(0x1234), 0x56);
        assert_eq!(memory.read(0x1235), 0x78);
    }

    #[test]
    fn encode_load_sp_to_immediate_address() {
        // Write 42 to A and then copy A to C
        let mut cpu = CpuState::new();
        let mut memory = DebugMemory::new_with_init(&[0x34, 0x12]);

        let instruction = LoadSpToImmediateAddress {
            address: 0,
            phase: FivePhases::First,
        };

        let instruction = instruction.execute(&mut cpu, &mut memory);
        let instruction = instruction.execute(&mut cpu, &mut memory);

        let encoded = instruction.encode();

        assert_eq!(encoded[0], 0b00001000);
        assert_eq!(encoded[1], 0x34);
        assert_eq!(encoded[2], 0x12);
    }
}
