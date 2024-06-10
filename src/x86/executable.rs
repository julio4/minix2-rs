use super::Instruction;

pub struct Executable {
    pub instructions: Vec<Instruction>,
}

impl Executable {
    pub fn new(instructions: Vec<Instruction>) -> Self {
        Executable { instructions }
    }
}

impl std::fmt::Display for Executable {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut bytes_count = 0;
        for instruction in &self.instructions {
            write!(f, "{:04x}: {}\n", bytes_count, instruction)?;
            bytes_count += instruction.raw.len();
        }
        Ok(())
    }
}
