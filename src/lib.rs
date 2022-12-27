#![warn(missing_docs)]
//! This crate will be a gameboy emulation library
//!

// TODO: Figure out how to idiomatically structure a rust project
/// First experiments with cartridges and storage
pub mod cartridge;
/// Contains [cpu::CpuState] and more.
pub mod cpu;
/// Contains [debug_memory::DebugMemory].
pub mod debug_memory;
/// Contains the [memory::MemoryDevice] trait.
pub mod memory;
