#![feature(internal_output_capture)]

//! This crate is a library for disassembling and interpreting Minix 2 binaries compiled for the 8086 CPU.
//! It can be used as a virtual machine for Minix 2 binaries.
//!
//! ## Features
//! - Disassembler: read a Minix 2 binary and output the sequence of CPU instructions.
//! - Interpreter: execute a sequence of CPU instructions and simulate the behavior of the 8086 CPU, including the stack, registers, memory and minix2 system calls.
//!
//! ## Note
//! This crate is for education pruposes only. Only a partial implementation of the instruction set is provided.
//!
//! ## Usage
//!
//! Read minix binary from file:
//! ```ignore
//! use minix2_rs::minix::Program;
//!
//! let file = std::fs::File::open(&args[1]).unwrap();
//! let program = Program::from_file(file).unwrap();
//! ```
//!
//! Disassemble program and output assembly code to stdout:
//! ```ignore
//! use minix2_rs::disassembler::Disassemblable;
//!
//! let disassembled = program.disassemble().unwrap();
//! println!("{}", disassembled);
//! ```
//!
//! Interpret program in minix2 virtual machine environment:
//! ```ignore
//! use minix2_rs::interpreter::Interpretable;
//!
//! program.interpret();
//! ```

/// Minix specifications
///
/// See [Minix 2 Homepage](https://minix1.woodhull.com/current/)
pub mod minix;

/// 8086 CPU specifications
///
/// See [8086 16-BIT HMOS MICROPROCESSOR 8086/8086-2/8086-1](https://www.electro-tech-online.com/datasheets/8086_intel.pdf)
/// See [Intel® 64 and IA-32 Architectures Software Developer’s Manual Volume 2](https://www.intel.com/content/www/us/en/developer/articles/technical/intel-sdm.html)
pub mod x86;

/// Disassembler
pub mod disassembler;
/// Interpreter
pub mod interpreter;

mod utils;
