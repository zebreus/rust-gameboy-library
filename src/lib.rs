#![warn(missing_docs)]
//! This crate will be a gameboy emulation library
//!

/// Contains [cpu::CpuState] and more.
pub mod cpu;

/// Contains the [memory::MemoryDevice] trait.
pub mod memory;

mod test_roms;
