use crate::x86::{Instruction, IR};

pub struct DisassembledProgram {
    pub instructions: Vec<Instruction>,
}

impl DisassembledProgram {
    pub fn new(instructions: Vec<Instruction>, data: Vec<u8>) -> Self {
        DisassembledProgram { instructions }
    }
}

impl From<Vec<IR>> for DisassembledProgram {
    fn from(ir: Vec<IR>) -> Self {
        let instructions = ir.into_iter().map(|ir| ir.into()).collect();
        DisassembledProgram::new(instructions, vec![])
    }
}

impl std::fmt::Display for DisassembledProgram {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut bytes_count = 0;
        for instruction in &self.instructions {
            write!(f, "{:04x}: {}\n", bytes_count, instruction)?;
            bytes_count += instruction.raw.len();
        }
        Ok(())
    }
}
