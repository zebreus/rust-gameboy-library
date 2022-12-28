#[cfg(test)]
mod tests {
    use crate::{
        cartridge::Cartridge,
        cpu::{instruction::Instruction, Cpu, CpuState},
        memory::Memory,
    };

    #[test]
    fn ld_test() {
        let mut cartridge = Cartridge::load("test_roms/blargg/cpu_instrs/individual/06-ld r,r.gb");
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
}
