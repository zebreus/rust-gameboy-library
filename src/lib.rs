#![warn(missing_docs)]
//! This crate will be a gameboy emulation library
//!

// TODO: Figure out how to idiomatically structure a rust project
/// First experiments with cartridges and storage
pub mod cartridge;
/// Contains [cpu::CpuState] and more.
pub mod cpu;

/// Contains the [memory::MemoryDevice] trait.
pub mod memory;

/// Contains functions to access video ram
pub mod video;

mod test_roms;
