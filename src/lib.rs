//! # Minix2-rs
//! Minix2-rs is a library for disassembling and interpreting Minix 2.0 binaries compiled for the 8086 CPU.
//!
//! ## Features
//! - Disassembling x86 instructions (90% complete)
//! - Interpreting x86 instructions (WIP)

/// Minix specifications
pub mod minix;

// 8086 CPU specifications
pub mod x86;

pub mod disassembler;
pub mod interpreter;

/// Utility functions
pub mod utils;
