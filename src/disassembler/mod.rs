pub mod error;
pub mod instruction;
pub mod memory;
pub mod parser;
pub mod program;
pub mod register;

pub use instruction::{Instruction, IR};
pub use memory::Memory;
pub use program::Program;
pub use register::Register;
