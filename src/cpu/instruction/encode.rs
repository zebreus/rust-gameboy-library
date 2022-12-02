use super::Instruction;

pub fn encode(instruction: Instruction) -> Vec<u8> {
    match instruction {
        Instruction::LoadFromRegisterToRegister {
            source,
            destination,
        } => {
            let base_code = 0b01000000 & 0b11000000u8;
            let source_code = (source.id() << 3) & 0b00111000u8;
            let destination_code = destination.id() & 0b00000111u8;
            let opcode = base_code | source_code | destination_code;
            Vec::from([opcode])
        }
        Instruction::LoadImmediateToRegister {
            destination,
            value,
            phase,
        } => {
            let base_code = 0b00000110 & 0b11000111u8;
            let destination_code = (destination.id() << 3) & 0b00111000u8;
            let opcode = base_code | destination_code;
            match phase {
                0 => Vec::from([opcode]),
                1 => Vec::from([opcode, value]),
                _ => Vec::new(),
            }
        }
        Instruction::LoadFromHlToRegister {
            destination,
            phase: _,
        } => {
            let base_code = 0b01000110 & 0b11000111u8;
            let destination_code = (destination.id() << 3) & 0b00111000u8;
            let opcode = base_code | destination_code;
            Vec::from([opcode])
        }
        Instruction::LoadAccumulatorToHlAndIncrement { phase: _ } => Vec::from([]),
        Instruction::None => Vec::new(),
    }
}

#[cfg(test)]
mod tests {
    use crate::cpu::instruction::decode::decode;
    use crate::cpu::Cpu;
    use crate::cpu::CpuState;
    use crate::cpu::Register;
    use crate::debug_memory::DebugMemory;

    #[test]
    fn load_instruction_works() {
        let mut cpu = CpuState::new();
        cpu.write_register(Register::A, 100);

        let mut memory = DebugMemory::new();
        let load_a_to_c = 0b01000010u8;
        let instruction = decode(load_a_to_c);

        let value_c_before = cpu.read_register(Register::C);
        assert_eq!(value_c_before, 0);

        instruction.execute(&mut cpu, &mut memory);
        let value_c_after = cpu.read_register(Register::C);

        assert_eq!(value_c_after, 100);
    }
}
