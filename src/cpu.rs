use crate::memory_device::MemoryDevice;

struct CpuState {
    acc: u8, // Not sure if this is a register
    program_counter: u16,
    stack_pointer: u16,
    registers: [u8; 8],
    flags: u8,
}

trait Cpu {
    fn process_cycle(&self, memory: &mut dyn MemoryDevice) -> ();
}

impl Cpu for CpuState {
    fn process_cycle(&self, memory: &mut dyn MemoryDevice) -> () {
        memory.read(7);
    }
}
