#![warn(missing_docs)]
//! This crate will be a gameboy emulation library
//!

// TODO: Figure out how to idiomatically structure a rust project
/// Contains [CpuState] and more.
pub mod cpu;
/// Contains [DebugMemory].
pub mod debug_memory;
/// Contains the [MemoryDevice] trait.
pub mod memory_device;
