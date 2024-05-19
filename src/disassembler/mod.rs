pub mod displacement;
pub mod error;
pub mod instruction;
pub mod memory;
pub mod parser;
pub mod program;
pub mod register;

pub use displacement::Displacement;
pub use instruction::{Instruction, IR};
pub use memory::Memory;
pub use program::Program;
pub use register::Register;

use std::fs::File;
use std::io::Read;

use self::error::ParseError;

pub fn minix2_disassemble(args: Vec<String>) -> Result<String, ParseError> {
    let file = File::open(&args[1]).expect("File not found");
    let binary = file
        .bytes()
        .map(|b| b.expect("Error reading binary file"))
        .collect::<Vec<u8>>();

    // Parse header
    let header = super::Header::parse(&binary)?;

    // Parse text segment
    let text_segment = super::TextSegment::parse(&binary, header.text)?;

    // Parse instructions from text segment
    let program = Program::from_text_segment(text_segment);

    // return mmvm -d disassembly output
    Ok(format!("{}", program))
}
