#[cfg(test)]
mod tests {
    use crate::{
        cartridge::Cartridge,
        cpu::{instruction::Instruction, Cpu, CpuState},
        memory::{Memory, MemoryDevice},
    };

    #[test]
    fn special_test() {
        let mut cartridge = Cartridge::load("test_roms/blargg/cpu_instrs/individual/01-special.gb");
        let mut cpu = CpuState::new();
        let mut memory = Memory::new_for_tests();
        cartridge.place_into_memory(&mut memory);
        cpu.write_program_counter(0x0100);
        let mut instruction = cpu.load_instruction(&mut memory);
        for _id in 1..10000000 {
            instruction = instruction.execute(&mut cpu, &mut memory);
            cartridge.process_writes(&mut memory);
        }
        assert_eq!(memory.printed_passed, 1);
    }

    // #[test]
    // fn stackpointer_operations_test() {
    //     let mut cartridge =
    //         Cartridge::load("test_roms/blargg/cpu_instrs/individual/03-op sp,hl.gb");
    //     let mut cpu = CpuState::new();
    //     let mut memory = Memory::new_for_tests();
    //     cartridge.place_into_memory(&mut memory);
    //     cpu.write_program_counter(0x0100);
    //     let mut instruction = cpu.load_instruction(&mut memory);
    //     for _id in 1..10000000 {
    //         instruction = instruction.execute(&mut cpu, &mut memory);
    //         cartridge.process_writes(&mut memory);
    //     }
    //     assert_eq!(memory.printed_passed, 1);
    // }

    #[test]
    fn load_immediate_to_register_test() {
        let mut cartridge =
            Cartridge::load("test_roms/blargg/cpu_instrs/individual/04-op r,imm.gb");
        let mut cpu = CpuState::new();
        let mut memory = Memory::new();
        cartridge.place_into_memory(&mut memory);
        cpu.write_program_counter(0x0100);
        let mut instruction = cpu.load_instruction(&mut memory);
        for _id in 1..10000000 {
            instruction = instruction.execute(&mut cpu, &mut memory);
            cartridge.process_writes(&mut memory);
        }
        assert_eq!(memory.printed_passed, 1);
    }

    #[test]
    fn fifth_instruction_test() {
        let mut cartridge = Cartridge::load("test_roms/blargg/cpu_instrs/individual/05-op rp.gb");
        let mut cpu = CpuState::new();
        let mut memory = Memory::new();
        cartridge.place_into_memory(&mut memory);
        cpu.write_program_counter(0x0100);
        let mut instruction = cpu.load_instruction(&mut memory);
        for _id in 1..10000000 {
            instruction = instruction.execute(&mut cpu, &mut memory);
            cartridge.process_writes(&mut memory);
        }
        assert_eq!(memory.printed_passed, 1);
    }

    #[test]
    fn ld_test() {
        let mut cartridge = Cartridge::load("test_roms/blargg/cpu_instrs/individual/06-ld r,r.gb");
        let mut cpu = CpuState::new();
        let mut memory = Memory::new_for_tests();
        cartridge.place_into_memory(&mut memory);
        cpu.write_program_counter(0x0100);
        let mut instruction = cpu.load_instruction(&mut memory);
        for _id in 1..1000000 {
            instruction = instruction.execute(&mut cpu, &mut memory);
            cartridge.process_writes(&mut memory);
        }
        assert_eq!(memory.printed_passed, 1);
    }

    // #[test]
    // fn branching_test() {
    //     let mut cartridge =
    //         Cartridge::load("test_roms/blargg/cpu_instrs/individual/07-jr,jp,call,ret,rst.gb");
    //     let mut cpu = CpuState::new();
    //     let mut memory = Memory::new_for_tests();
    //     cartridge.place_into_memory(&mut memory);
    //     cpu.write_program_counter(0x0100);
    //     let mut instruction = cpu.load_instruction(&mut memory);
    //     for _id in 1..100000000 {
    //         instruction = instruction.execute(&mut cpu, &mut memory);
    //         cartridge.process_writes(&mut memory);
    //         if (cpu.read_program_counter() >= 0xc000) {
    //             let x = 8;
    //         }
    //     }
    //     assert_eq!(memory.printed_passed, 1);
    // }

    #[test]
    fn misc_instructions_test() {
        let mut cartridge =
            Cartridge::load("test_roms/blargg/cpu_instrs/individual/08-misc instrs.gb");
        let mut cpu = CpuState::new();
        let mut memory = Memory::new_for_tests();
        cartridge.place_into_memory(&mut memory);
        cpu.write_program_counter(0x0100);
        let mut instruction = cpu.load_instruction(&mut memory);
        for _id in 1..1000000 {
            instruction = instruction.execute(&mut cpu, &mut memory);
            cartridge.process_writes(&mut memory);
        }
        assert_eq!(memory.printed_passed, 1);
    }

    #[test]
    fn register_operations_test() {
        let mut cartridge = Cartridge::load("test_roms/blargg/cpu_instrs/individual/09-op r,r.gb");
        let mut cpu = CpuState::new();
        let mut memory = Memory::new_for_tests();
        cartridge.place_into_memory(&mut memory);
        cpu.write_program_counter(0x0100);
        let mut instruction = cpu.load_instruction(&mut memory);
        for _id in 1..10000000 {
            instruction = instruction.execute(&mut cpu, &mut memory);
            cartridge.process_writes(&mut memory);
        }
        assert_eq!(memory.printed_passed, 1);
    }

    #[test]
    fn bitwise_operations_test() {
        let mut cartridge = Cartridge::load("test_roms/blargg/cpu_instrs/individual/10-bit ops.gb");
        let mut cpu = CpuState::new();
        let mut memory = Memory::new_for_tests();
        cartridge.place_into_memory(&mut memory);
        cpu.write_program_counter(0x0100);
        let mut instruction = cpu.load_instruction(&mut memory);
        for _id in 1..20000000 {
            instruction = instruction.execute(&mut cpu, &mut memory);
            cartridge.process_writes(&mut memory);
        }
        assert_eq!(memory.printed_passed, 1);
    }

    #[test]
    fn accumulator_with_hl_test() {
        let mut cartridge =
            Cartridge::load("test_roms/blargg/cpu_instrs/individual/11-op a,(hl).gb");
        let mut cpu = CpuState::new();
        let mut memory = Memory::new_for_tests();
        cartridge.place_into_memory(&mut memory);
        cpu.write_program_counter(0x0100);
        let mut instruction = cpu.load_instruction(&mut memory);
        for _id in 1..50000000 {
            instruction = instruction.execute(&mut cpu, &mut memory);
            cartridge.process_writes(&mut memory);
        }
        assert_eq!(memory.printed_passed, 1);
    }
}
