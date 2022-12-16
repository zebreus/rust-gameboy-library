use super::Instruction;
use crate::{
    cpu::{Cpu, Flag, Register},
    memory_device::MemoryDevice,
};

/// Add a value of the operand register to the accumulator.
pub struct AddRegister {
    /// The operand register
    pub operand: Register,
}

impl Instruction for AddRegister {
    fn execute<T: MemoryDevice>(
        &self,
        cpu: &mut crate::cpu::CpuState,
        memory: &mut T,
    ) -> super::InstructionEnum {
        let accumulator_value = cpu.read_register(Register::A);
        let operand_value = cpu.read_register(self.operand);
        let result = accumulator_value.wrapping_add(operand_value);

        // set flags
        let zero_flag = result == 0;
        let subtract_flag = false;
        let half_carry_flag =
            (accumulator_value ^ operand_value ^ result) & 0b00010000 == 0b00010000;
        let carry_flag = result < accumulator_value;

        cpu.write_flag(Flag::Zero, zero_flag);
        cpu.write_flag(Flag::Subtract, subtract_flag);
        cpu.write_flag(Flag::HalfCarry, half_carry_flag);
        cpu.write_flag(Flag::Carry, carry_flag);

        cpu.write_register(Register::A, result);
        return cpu.load_instruction(memory);
    }
    fn encode(&self) -> Vec<u8> {
        let base_code = 0b10000000 & 0b11111000u8;
        let operand_code = self.operand.id() & 0b00000111u8;
        let opcode = base_code | operand_code;
        Vec::from([opcode])
    }
}

#[cfg(test)]
mod tests {
    use super::AddRegister;
    use crate::cpu::instruction::Instruction;
    use crate::cpu::{Cpu, CpuState, Flag, Register};
    use crate::debug_memory::DebugMemory;

    fn execute_add_instruction(value_a: u8, value_b: u8) -> CpuState {
        let mut cpu = CpuState::new();
        cpu.write_register(Register::A, value_a);
        cpu.write_register(Register::B, value_b);

        let mut memory = DebugMemory::new();
        let instruction = AddRegister {
            operand: Register::B,
        };
        instruction.execute(&mut cpu, &mut memory);

        return cpu;
    }

    #[test]
    fn load_instruction_works() {
        let mut cpu = CpuState::new();
        cpu.write_register(Register::A, 100);
        cpu.write_register(Register::B, 100);

        let mut memory = DebugMemory::new();
        let instruction = AddRegister {
            operand: Register::B,
        };

        instruction.execute(&mut cpu, &mut memory);

        assert_eq!(cpu.read_register(Register::A), 200);
    }

    #[test]
    fn zero_flag_works() {
        let cpu = execute_add_instruction(100, 100);

        assert_eq!(cpu.read_flag(Flag::Zero), false);
    }

    #[test]
    fn half_carry_flag_works() {
        // Tuple of (value_a, value_b, expected_result)
        let test_cases = vec![
            (0b00001111, 0b00100000, false),
            (0b00001111, 0b00000001, true),
            (0b00001111, 0b00010001, true),
            (0b00001111, 0b00010000, false),
            (0b00010001, 0b00001111, true),
            (0b00010001, 0b00000010, false),
            (0b00010001, 0b00010000, false),
            (0b00010001, 0b00011111, true),
        ];
        for test_case in test_cases {
            let cpu = execute_add_instruction(test_case.0, test_case.1);
            assert_eq!(cpu.read_flag(Flag::HalfCarry), test_case.2);
            assert_eq!(cpu.read_flag(Flag::Subtract), false);
        }
    }

    #[test]
    fn carry_flag_works() {
        let cpu = execute_add_instruction(0b11111111, 0b00000001);
        assert_eq!(cpu.read_flag(Flag::Carry), true);

        let cpu = execute_add_instruction(0b01111111, 0b00000001);
        assert_eq!(cpu.read_flag(Flag::Carry), false);
    }
}
