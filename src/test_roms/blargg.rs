mod instructions;

#[cfg(test)]
use crate::{
    cartridge::Cartridge,
    cpu::{instruction::Instruction, Cpu, CpuState},
    memory::Memory,
};

#[cfg(test)]
fn test_blargg_rom(path: &str, cycles: usize) {
    let mut cartridge = Cartridge::load(path);
    let mut cpu = CpuState::new();
    let mut memory = Memory::new();
    cartridge.place_into_memory(&mut memory);
    cpu.write_program_counter(0x0100);
    let mut instruction = cpu.load_instruction(&mut memory);
    for _id in 1..cycles {
        instruction = instruction.execute(&mut cpu, &mut memory);
        cartridge.process_writes(&mut memory);
        memory.process_cycle();
    }
    assert_eq!(memory.printed_passed, 1);
}
