use crate::{
    disassembler::{error::ParseError, parser, Instruction, IR},
    text_segment::TextSegment,
};

/// The high-level representation of a program.
pub struct Program {
    pub instructions: Vec<Instruction>,
}

impl Program {
    pub fn new(instructions: Vec<Instruction>) -> Self {
        Program { instructions }
    }

    pub fn from_text_segment(segment: TextSegment) -> Program {
        let mut instructions = Vec::new();
        let mut text = segment.text.as_slice();

        let mut ip = 0;
        while !text.is_empty() {
            let (instruction, bytes_consumed) = match parser::parse_instruction(text, ip) {
                Ok((instruction, bytes_consumed)) => (instruction, bytes_consumed),
                Err(ParseError::UnexpectedEOF) => {
                    (Instruction::new(IR::Undefined, text.to_vec()), text.len())
                }
                Err(err) => {
                    eprintln!("Error: {}", err);
                    break;
                }
            };
            // DEBUG:
            println!("{:04x}: {}", ip, instruction);
            ip += bytes_consumed;

            instructions.push(instruction);
            text = &text[bytes_consumed..];
        }

        Program::new(instructions)
    }
}

impl std::fmt::Display for Program {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut bytes_count = 0;
        for instruction in &self.instructions {
            write!(f, "{:04x}: {}\n", bytes_count, instruction)?;
            bytes_count += instruction.raw.len();
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::disassembler::{instruction::Operand, Register, IR};

    #[test]
    fn test_from_text_segment() {
        let text_segment = TextSegment {
            text: vec![0xbb, 0xFF, 0x00],
        };
        let program = Program::from_text_segment(text_segment);
        assert_eq!(program.instructions.len(), 1);
        assert_eq!(
            program.instructions[0],
            Instruction::new(
                IR::Mov {
                    dest: Operand::Register(Register::BX),
                    src: Operand::LongImmediate(0x00FF)
                },
                vec![0xbb, 0xFF, 0x00]
            )
        );
    }
}
