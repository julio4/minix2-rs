pub mod error;
pub mod instruction;
pub mod memory;
pub mod parser;
pub mod program;
pub mod register;

pub use error::ParseError;
pub use instruction::IR;
pub use parser::parse_instruction;
