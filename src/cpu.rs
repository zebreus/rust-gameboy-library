use num_enum::{IntoPrimitive, TryFromPrimitive};

use crate::memory_device::MemoryDevice;

/// Instructions can be executed to modify cpu state and memory
pub mod instruction;

use self::instruction::decode;
use self::instruction::InstructionEnum;
use self::instruction::InterruptServiceRoutine;

/// The CpuState stores the internal state of the gameboy processor.
///
/// This is basically just a data container, the actual CPU functionality is handled by [Instruction](instruction::Instruction).
pub struct CpuState {
    program_counter: u16,
    stack_pointer: u16,
    registers: [u8; 8],

    interrupt_enable: u8,
    interrupt_flags: u8,
    interrupt_master_enable: bool,
}

impl CpuState {
    /// Initialize a new CPU state.
    ///
    /// The program counter is set to the start of the ROM.
    /// The stack pointer is set to 0xFFFE.
    /// The registers are set to 0.
    ///
    /// ```
    /// use rust_gameboy_library::cpu::CpuState;
    ///
    /// let cpuState = CpuState::new();
    /// ```
    pub fn new() -> Self {
        Self {
            program_counter: 0, // 0x0100
            stack_pointer: 0xFFFE,
            registers: [0x00, 0x13, 0x00, 0xD8, 0x01, 0x4d, 0xB0, 0x01],

            interrupt_enable: 0,
            interrupt_flags: 0,
            interrupt_master_enable: false,
        }
    }
    /// Load the next opcode
    ///
    /// Also increments the program counter
    pub fn load_opcode<T: MemoryDevice>(&mut self, memory: &T) -> u8 {
        let opcode = memory.read(self.advance_program_counter());
        return opcode;
    }

    /// Load the next [Instruction](self::instruction::Instruction)
    ///
    // TODO: Link to ISR instruction
    /// Returns a ISR, if there are pending interrupts and the [IME][self::Cpu::read_interrupt_master_enable] is set.
    ///
    /// Also increments the program counter
    pub fn load_instruction<T: MemoryDevice>(&mut self, memory: &T) -> InstructionEnum {
        let pending_interrupt = self.get_pending_interrupt();
        match pending_interrupt {
            Some(interrupt) => interrupt,
            None => {
                let opcode = self.load_opcode(memory);
                decode(opcode)
            }
        }
    }
}

/// Trait for something that can be used as a gameboy cpu state.
pub trait Cpu {
    /// Get the address of the current instruction. Increment the program counter
    ///
    /// Returns the unincremented value of the program counter.
    fn advance_program_counter(&mut self) -> u16;
    /// Get the address of the current instruction.
    fn read_program_counter(&self) -> u16;
    /// Set the address of the current instruction
    fn write_program_counter(&mut self, value: u16);
    /// Get the current stack pointer
    fn read_stack_pointer(&self) -> u16;
    /// Set the current stack pointer
    fn write_stack_pointer(&mut self, value: u16);
    /// Get the content of a register.
    fn read_register(&self, register: Register) -> u8;
    /// Write a value to a register
    ///
    /// ```
    /// # use rust_gameboy_library::cpu::{CpuState, Cpu, Register};
    /// # let mut cpu = CpuState::new();
    /// #
    /// cpu.write_register(Register::A, 5);
    /// ```
    fn write_register(&mut self, register: Register, value: u8) -> ();
    /// Read the value from two registers at once.
    ///
    /// The documentation for [DoubleRegister] contains more information on the values.
    fn read_double_register(&self, register: DoubleRegister) -> u16;
    /// Read the value from two registers at once.
    ///
    /// The documentation for [DoubleRegister] contains more information on the values.
    fn write_double_register(&mut self, register: DoubleRegister, value: u16) -> ();
    /// Read the value of a flag
    fn read_flag(&self, flag: Flag) -> bool;
    /// Write the value of a flag
    fn write_flag(&mut self, flag: Flag, value: bool);
    /// Check if a condition is currently satisfied
    fn check_condition(&self, condition_code: ConditionCode) -> bool;
    /// Set the IME. This is the only way to write the IME.
    fn write_interrupt_master_enable(&mut self, value: bool);
    /// Check if the IME is enabled. This is the only way to read the IME.
    fn read_interrupt_master_enable(&mut self) -> bool;
    /// Set the interrupt enable flag for a specific interrupt.
    ///
    /// This is equivalent to modifying the IE register at memory address 0xffff
    fn write_interrupt_enable(&mut self, interrupt: Interrupt, value: bool);
    /// Read if a interrupt is enabled.
    ///
    /// This is equivalent to reading the IE register at memory address 0xffff
    fn read_interrupt_enable(&self, interrupt: Interrupt) -> bool;
    /// Get the complete IE
    fn read_interrupt_enable_register(&self) -> u8;
    /// Get the complete IF
    fn read_interrupt_flag_register(&self) -> u8;
    /// Set the interrupt flag for a specific interrupt.
    ///
    /// This is equivalent to modifying the IF register at memory address 0xff0f
    fn write_interrupt_flag(&mut self, interrupt: Interrupt, value: bool);
    /// Read if a interrupt is requested
    ///
    /// This is equivalent to reading the IE register at memory address 0xffff
    fn read_interrupt_flag(&self, interrupt: Interrupt) -> bool;
    /// Get the instruction of a pending interrupt if there is one.
    fn get_pending_interrupt(&mut self) -> Option<InstructionEnum> {
        if !self.read_interrupt_master_enable() {
            return None;
        }

        let triggered_interrupts =
            self.read_interrupt_enable_register() & self.read_interrupt_flag_register();

        if triggered_interrupts == 0 {
            return None;
        }

        if (triggered_interrupts & (Interrupt::VBlank as u8)) != 0 {
            self.write_interrupt_flag(Interrupt::VBlank, false);
            return Some(InterruptServiceRoutine::create(0x0040).into());
        }

        if (triggered_interrupts & (Interrupt::LcdStat as u8)) != 0 {
            self.write_interrupt_flag(Interrupt::LcdStat, false);
            return Some(InterruptServiceRoutine::create(0x0048).into());
        }

        if (triggered_interrupts & (Interrupt::Timer as u8)) != 0 {
            self.write_interrupt_flag(Interrupt::Timer, false);
            return Some(InterruptServiceRoutine::create(0x0050).into());
        }

        if (triggered_interrupts & (Interrupt::Serial as u8)) != 0 {
            self.write_interrupt_flag(Interrupt::Serial, false);
            return Some(InterruptServiceRoutine::create(0x0058).into());
        }

        if (triggered_interrupts & (Interrupt::Joypad as u8)) != 0 {
            self.write_interrupt_flag(Interrupt::Joypad, false);
            return Some(InterruptServiceRoutine::create(0x0060).into());
        }

        return None;
    }
}

impl Cpu for CpuState {
    fn advance_program_counter(&mut self) -> u16 {
        let result = self.program_counter;
        self.program_counter += 1;
        return result;
    }
    fn read_program_counter(&self) -> u16 {
        self.program_counter
    }
    fn write_program_counter(&mut self, value: u16) {
        self.program_counter = value;
    }
    fn read_stack_pointer(&self) -> u16 {
        let result = self.stack_pointer;
        return result;
    }
    fn write_stack_pointer(&mut self, value: u16) {
        self.stack_pointer = value;
    }

    fn read_register(&self, register: Register) -> u8 {
        let index = register as usize;
        return self.registers[index];
    }
    fn write_register(&mut self, register: Register, value: u8) -> () {
        let index = register as usize;

        if let Register::F = register {
            // You cannot write bit 0-3 on the flags register
            self.registers[index] = value & 0b11110000;
        }
        self.registers[index] = value;
    }
    fn read_double_register(&self, register: DoubleRegister) -> u16 {
        let registers = register.id();
        let lsb = self.read_register(registers.lsb);
        let msb = self.read_register(registers.msb);
        let value: u16 = u16::from_le_bytes([lsb, msb]);
        return value;
    }
    fn write_double_register(&mut self, register: DoubleRegister, value: u16) -> () {
        let registers = register.id();
        let [lsb, msb] = u16::to_le_bytes(value);
        self.write_register(registers.msb, msb);
        self.write_register(registers.lsb, lsb);
    }
    fn read_flag(&self, flag: Flag) -> bool {
        let flags_register = self.read_register(Register::F);
        (flags_register & (flag as u8)) == (flag as u8)
    }
    fn write_flag(&mut self, flag: Flag, value: bool) {
        let flags_register = self.read_register(Register::F);
        let flags_register = if value {
            flags_register | (flag as u8)
        } else {
            flags_register & (!(flag as u8))
        };
        self.write_register(Register::F, flags_register);
    }
    fn check_condition(&self, condition_code: ConditionCode) -> bool {
        match condition_code {
            ConditionCode::ZeroFlagUnset => self.read_flag(Flag::Zero) == false,
            ConditionCode::ZeroFlagSet => self.read_flag(Flag::Zero) == true,
            ConditionCode::CarryFlagUnset => self.read_flag(Flag::Carry) == false,
            ConditionCode::CarryFlagSet => self.read_flag(Flag::Carry) == true,
        }
    }
    fn write_interrupt_master_enable(&mut self, value: bool) {
        self.interrupt_master_enable = value;
    }
    fn read_interrupt_master_enable(&mut self) -> bool {
        self.interrupt_master_enable
    }
    fn write_interrupt_enable(&mut self, interrupt: Interrupt, value: bool) {
        self.interrupt_enable = if value {
            self.interrupt_enable | (interrupt as u8)
        } else {
            self.interrupt_enable & !(interrupt as u8)
        }
    }
    fn read_interrupt_enable(&self, interrupt: Interrupt) -> bool {
        self.interrupt_enable & (interrupt as u8) != 0
    }

    fn write_interrupt_flag(&mut self, interrupt: Interrupt, value: bool) {
        self.interrupt_flags = if value {
            self.interrupt_flags | (interrupt as u8)
        } else {
            self.interrupt_flags & !(interrupt as u8)
        }
    }
    fn read_interrupt_flag(&self, interrupt: Interrupt) -> bool {
        self.interrupt_flags & (interrupt as u8) != 0
    }
    fn read_interrupt_enable_register(&self) -> u8 {
        self.interrupt_enable
    }
    fn read_interrupt_flag_register(&self) -> u8 {
        self.interrupt_flags
    }
}

/// Register of the gameboy cpu
///
/// The gameboy processor has 8 general purpose 8bit registers.
/// The registers are named A, B, C, D, E, F, H, and L.
/// The registers are accessed by the `read_register`
///
/// The number this enum uses for each register corresponds to it's binary representation in opcodes.
/// The opcode for loading an immediate value to a register contains three bits a (`00aaa110`) which select the target register. They can be set to the value of a variant from this enum.
#[derive(TryFromPrimitive, Debug, IntoPrimitive, Clone, Copy)]
#[repr(u8)]
pub enum Register {
    /// A general purpose register.
    B = 0b000,
    /// A general purpose register.
    C = 0b001,
    /// A general purpose register.
    D = 0b010,
    /// A general purpose register.
    E = 0b011,
    /// A general purpose register. Part of [DoubleRegister::HL].
    H = 0b100,
    /// A general purpose register. Part of [DoubleRegister::HL].
    L = 0b101,
    /// The flags register.
    ///
    /// Inaccessible for most operations.
    ///
    /// Flags get set by many instructions.
    ///
    /// Contains flags set and used by various instructions.
    ///
    /// Table, when the bitorder is `0b76543210`:
    ///
    /// |  Bit 0   |   Bit 1  |   Bit 2  |   Bit 3  | Bit 4 |   Bit 5    |  Bit 6   | Bit 7 |
    /// |----------|----------|----------|----------|-------|------------|----------|-------|
    /// | readonly | readonly | readonly | readonly | carry | half-carry | negative | zero  |
    F = 0b110,
    /// A general purpose register. Acts as the accumulator for some instructions.
    A = 0b111,
}

impl Register {
    fn id(&self) -> u8 {
        *self as u8
    }
}

struct RegisterCombination {
    lsb: Register,
    msb: Register,
}

/// The gameboy has many instructions that combine two registers as a single 16bit value.
///
/// This enum represents the two registers that are combined.
#[derive(TryFromPrimitive, Debug, IntoPrimitive, Clone, Copy)]
#[repr(u8)]
pub enum DoubleRegister {
    /// A double register consisting of [Register::A] and [Register::F].
    ///
    /// Does not allow writing the bits 0-3 of F. See [Register::F] for details.
    AF = 0,
    /// A general purpose double register consisting of [Register::B] and [Register::C].
    BC = 1,
    /// A general purpose double register consisting of [Register::D] and [Register::E].
    DE = 2,
    /// A double register consisting of [Register::H] and [Register::L].
    ///
    /// Contains memory addresses for some operations.
    HL = 3,
}

impl DoubleRegister {
    fn id(&self) -> RegisterCombination {
        match self {
            DoubleRegister::AF => RegisterCombination {
                msb: Register::A,
                lsb: Register::F,
            },
            DoubleRegister::BC => RegisterCombination {
                msb: Register::B,
                lsb: Register::C,
            },
            DoubleRegister::DE => RegisterCombination {
                msb: Register::D,
                lsb: Register::E,
            },
            DoubleRegister::HL => RegisterCombination {
                msb: Register::H,
                lsb: Register::L,
            },
        }
    }

    fn numerical_id(&self) -> u8 {
        *self as u8
    }
}

/// Condition codes that are used in conditional jump opcodes
#[derive(TryFromPrimitive, Debug, IntoPrimitive, Clone, Copy)]
#[repr(u8)]
pub enum ConditionCode {
    /// Jump if Z flag is reset.
    ///
    /// Named `NZ` in assembler
    ZeroFlagUnset = 0b00,
    /// Jump if Z flag is set.
    ///
    /// Named `Z` in assembler
    ZeroFlagSet = 0b01,
    /// Jump if C flag is reset.
    ///
    /// Named `NC` in assembler
    CarryFlagUnset = 0b10,
    /// Jump if C flag is set.
    ///
    /// Named `C` in assembler
    CarryFlagSet = 0b11,
}

/// Condition codes that are used in conditional jump opcodes
///
/// The value of every element is a byte with a single bit set to 1. The set bit corresponds to the flags bit in the flags register.
#[derive(TryFromPrimitive, Debug, IntoPrimitive, Clone, Copy)]
#[repr(u8)]
pub enum Flag {
    // TODO: Replace CP With a link to the CP instruction once that is implemented
    /// Set if zero was the result of the last math operation. Also set if values compared with the compare instruction CP match.
    Zero = 0b10000000,
    /// Set if the last math operation included a subtraction
    Subtract = 0b01000000,
    /// Set if a carry on the lower nibble occurred in the last math operation
    HalfCarry = 0b00100000,
    /// Set if a carry occurred in the last math operation
    Carry = 0b00010000,
}

/// Interrupt codes that can be used to enable and request interrupts from the CPU.
///
/// You can use them with the matching methods on the CPU.
///
// TODO: Link to ISR and get_pending_interrupt
///
/// See [https://gbdev.io/pandocs/Interrupts.html] for more details on how interrupts work.
///
/// There is also a useful [section in the gameboy cpu manual](http://marc.rawer.de/Gameboy/Docs/GBCPUman.pdf#page=32)
#[derive(TryFromPrimitive, Debug, IntoPrimitive, Clone, Copy)]
#[repr(u8)]
pub enum Interrupt {
    /// VBlank interrupt
    ///
    /// Interrupt handler at 0x0040
    VBlank = 0b00000001,
    /// LcdStat interrupt
    ///
    /// Interrupt handler at 0x0048
    LcdStat = 0b00000010,
    /// Timer interrupt
    ///
    /// Interrupt handler at 0x0050
    Timer = 0b00000100,
    /// Serial Interrupt
    ///
    /// Interrupt handler at 0x0058
    Serial = 0b00001000,
    /// Joypad interrupt
    ///
    /// Interrupt handler at 0x0060
    Joypad = 0b00010000,
}

#[cfg(test)]
mod tests {
    use super::instruction::{InstructionEnum, LoadFromRegisterToRegister};
    use super::Cpu;
    use super::{CpuState, DoubleRegister};
    use crate::cpu::Register;
    use crate::debug_memory::DebugMemory;

    #[test]
    fn read_double_register() {
        let mut cpu = CpuState::new();
        cpu.write_register(Register::B, 1);
        cpu.write_register(Register::C, 3);
        let double_value = cpu.read_double_register(DoubleRegister::BC);
        assert_eq!(double_value, 259)
    }

    #[test]
    fn write_double_register() {
        let mut cpu = CpuState::new();
        cpu.write_double_register(DoubleRegister::BC, 259);
        assert_eq!(cpu.read_register(Register::B), 1);
        assert_eq!(cpu.read_register(Register::C), 3);
    }

    #[test]
    fn write_read_double_register() {
        let mut cpu = CpuState::new();
        cpu.write_double_register(DoubleRegister::BC, 9874);
        assert_eq!(cpu.read_double_register(DoubleRegister::BC), 9874);
    }

    #[test]
    fn cpu_read_program_counter_works() {
        let mut cpu = CpuState::new();

        let memory = DebugMemory::new_with_init(&[0b01000010u8, 8]);
        let opcode = cpu.load_opcode(&memory);

        assert_eq!(opcode, 0b01000010u8);

        let opcode = cpu.load_opcode(&memory);
        assert_eq!(opcode, 8);
    }

    #[test]
    fn cpu_can_fetch_and_decode_instructions() {
        let mut cpu = CpuState::new();
        cpu.write_register(Register::A, 100);

        let memory = DebugMemory::new_with_init(&[0b01111001u8]);
        let instruction = cpu.load_instruction(&memory);
        assert!(matches!(
            instruction,
            InstructionEnum::LoadFromRegisterToRegister(LoadFromRegisterToRegister {
                source: Register::A,
                destination: Register::C
            })
        ))
    }
}
