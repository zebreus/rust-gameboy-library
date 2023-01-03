mod acceptance;

#[cfg(test)]
use crate::{
    cpu::{instruction::Instruction, Cpu, CpuState},
    memory::{cartridge::Cartridge, serial::serial_connection::LineBasedConnection, Memory},
};

#[cfg(test)]
fn test_mooneye_rom(path: &str, cycles: usize) {
    use crate::cpu::Register;

    let cartridge = Cartridge::load(path);
    let mut cpu = CpuState::new();
    let mut closure = |line: &String| println!("Serial: {}", line);

    let mut memory = Memory::new_with_connections(Some(LineBasedConnection::new(&mut closure)));
    cartridge.place_into_memory(&mut memory.memory);
    memory.cartridge = cartridge;
    cpu.write_program_counter(0x0100);
    let mut instruction = cpu.load_instruction(&mut memory);
    for _id in 1..cycles {
        instruction = instruction.execute(&mut cpu, &mut memory);
        memory.process_cycle();
        if (cpu.read_register(Register::B) == 3)
            && (cpu.read_register(Register::C) == 5)
            && (cpu.read_register(Register::D) == 8)
            && (cpu.read_register(Register::E) == 13)
            && (cpu.read_register(Register::H) == 21)
            && (cpu.read_register(Register::L) == 34)
        {
            break;
        }
    }

    assert_eq!(cpu.read_register(Register::B), 3);
    assert_eq!(cpu.read_register(Register::C), 5);
    assert_eq!(cpu.read_register(Register::D), 8);
    assert_eq!(cpu.read_register(Register::E), 13);
    assert_eq!(cpu.read_register(Register::H), 21);
    assert_eq!(cpu.read_register(Register::L), 34);
}
