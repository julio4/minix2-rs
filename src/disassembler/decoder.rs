use super::parser;
use super::{error::DisassemblerError, DisassembledProgram};
use crate::{
    minix::Program,
    x86::{Instruction, IR},
};

pub trait Disassemblable {
    fn disassemble(&self) -> Result<DisassembledProgram, DisassemblerError>;
}

impl Disassemblable for Program {
    fn disassemble(&self) -> Result<DisassembledProgram, DisassemblerError> {
        let mut instructions = Vec::new();
        let mut text = self.text_segment.as_slice();

        let mut ip = 0;
        while !text.is_empty() {
            let (instruction, bytes_consumed) = match parser::parse_instruction(text, ip) {
                Ok((instruction, bytes_consumed)) => (instruction, bytes_consumed),
                Err(DisassemblerError::UnexpectedEOF) => {
                    (Instruction::new(IR::Undefined, text.to_vec()), text.len())
                }
                Err(e) => return Err(e),
            };
            ip += bytes_consumed;

            instructions.push(instruction);
            text = &text[bytes_consumed..];
        }

        Ok(DisassembledProgram::new(
            instructions,
            self.data_segment.data.clone(),
        ))
    }
}

pub fn decode(args: Vec<String>) -> Result<String, DisassemblerError> {
    if args.len() < 2 {
        return Err(DisassemblerError::InvalidArgs);
    }

    let file = std::fs::File::open(&args[1]).map_err(|_| DisassemblerError::InvalidArgs)?;
    let program = Program::from_file(file).map_err(|_| DisassemblerError::InvalidArgs)?;

    let disassembled = program.disassemble()?;
    Ok(disassembled.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::fs;

    fn assert_disassemble(file: &str) {
        let args = vec![
            "minix2_rs".to_string(),
            format!("./tests_data/{}.out", file),
        ];
        let result = decode(args).unwrap();

        let expected = fs::read_to_string(format!("./tests_data/{}.expected", file)).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_asem_1() {
        assert_disassemble("asem/1.s");
    }

    #[test]
    fn test_asem_2() {
        assert_disassemble("asem/2.s");
    }

    #[test]
    fn test_asem_3() {
        assert_disassemble("asem/3.s");
    }

    #[test]
    fn test_asem_4() {
        assert_disassemble("asem/4.s");
    }

    #[test]
    fn test_c_1() {
        assert_disassemble("1.c");
    }

    #[test]
    fn test_c_2() {
        assert_disassemble("2.c");
    }

    #[test]
    fn test_c_3() {
        assert_disassemble("3.c");
    }

    #[test]
    fn test_c_4() {
        assert_disassemble("4.c");
    }

    #[test]
    fn test_c_5() {
        assert_disassemble("5.c");
    }

    #[test]
    fn test_c_6() {
        assert_disassemble("6.c");
    }

    #[test]
    fn test_c_7() {
        assert_disassemble("7.c");
    }
}
