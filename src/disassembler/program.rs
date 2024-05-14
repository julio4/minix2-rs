use super::{parse_instruction, ParseError};
use crate::{disassembler::Instruction, text_segment::TextSegment};

/// The high-level representation of a program.
pub struct Program {
    pub instructions: Vec<Instruction>,
}

impl Program {
    pub fn new(instructions: Vec<Instruction>) -> Self {
        Program { instructions }
    }

    pub fn from_text_segment(segment: TextSegment) -> Result<Program, ParseError> {
        let mut instructions = Vec::new();
        let mut text = segment.text.as_slice();

        while !text.is_empty() {
            let (instruction, bytes_consumed) = parse_instruction(text)?;
            instructions.push(instruction);
            text = &text[bytes_consumed..];
        }

        Ok(Program::new(instructions))
    }
}
#[cfg(test)]
mod tests {
    use crate::disassembler::{instruction::Operand, register::Register};

    use super::*;

    #[test]
    fn test_new() {
        let instructions = vec![
            Instruction::Mov {
                dest: Operand::Register(Register::AX),
                src: Operand::Register(Register::AX),
            },
            Instruction::Mov {
                dest: Operand::Register(Register::AX),
                src: Operand::Register(Register::AX),
            },
            Instruction::Mov {
                dest: Operand::Register(Register::AX),
                src: Operand::Register(Register::AX),
            },
        ];
        let program = Program::new(instructions);
        assert_eq!(program.instructions.len(), 3);
    }

    #[test]
    fn test_from_text_segment() {
        let text_segment = TextSegment {
            text: vec![0xbb, 0x00, 0x00],
        };
        let program = Program::from_text_segment(text_segment).unwrap();
        assert_eq!(program.instructions.len(), 1);
        assert_eq!(
            program.instructions[0],
            Instruction::Mov {
                dest: Operand::Register(Register::BX),
                src: Operand::LongImmediate(0x0000),
            }
        );
    }
}
