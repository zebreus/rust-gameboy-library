mod instruction_timing;
mod instructions;
mod memory_timing;

#[cfg(test)]
use crate::{
    cpu::{instruction::Instruction, Cpu, CpuState},
    memory::{cartridge::Cartridge, serial::serial_connection::LineBasedConnection, Memory},
};

#[cfg(test)]
fn test_blargg_rom(path: &str, cycles: usize) {
    let mut passed = 0;

    let cartridge = Cartridge::load(path);
    let mut cpu = CpuState::new();
    let mut closure = |line: &String| {
        if line.contains("Passed") {
            passed += 1;
        }
        println!("Serial: {}", line)
    };

    let mut memory = Memory::new_with_connections(Some(LineBasedConnection::new(&mut closure)));
    cartridge.place_into_memory(&mut memory.memory);
    memory.cartridge = cartridge;
    cpu.write_program_counter(0x0100);
    let mut instruction = cpu.load_instruction(&mut memory);
    for _id in 1..cycles {
        instruction = instruction.execute(&mut cpu, &mut memory);
        memory.process_cycle();
    }

    assert_eq!(passed, 1);
}
