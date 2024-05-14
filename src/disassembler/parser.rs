use super::error::ParseError;
use super::instruction::{Operand, IR};
use super::register::Register;

/// Parses the given byte slice and returns the parsed instruction along with the number of bytes consumed.
///
/// # Arguments
///
/// * `bytes` - The byte slice containing the instruction to parse.
///
/// # Returns
///
/// Returns a `Result` containing a tuple with the parsed `Instruction` and the number of bytes consumed.
/// If parsing fails, a `ParseError` is returned.
pub fn parse_instruction(bytes: &[u8]) -> Result<(IR, usize), ParseError> {
    if bytes.is_empty() {
        return Err(ParseError::UnexpectedEOF);
    }

    let opcode = bytes[0];
    match opcode {
        // MOV imm, reg
        0b10110000..=0b10111111 => {
            let w = (opcode & 0b00001000) != 0;
            if bytes.len() < (2 + w as usize) {
                return Err(ParseError::UnexpectedEOF);
            }

            let reg = Operand::Register(Register::from(opcode & 0b00000111, w));
            let imm = if w {
                Operand::LongImmediate(u16::from_le_bytes([bytes[1], bytes[2]]))
            } else {
                Operand::Immediate(u8::from_le_bytes([bytes[1]]).into())
            };

            Ok((
                IR::Mov {
                    dest: reg,
                    src: imm,
                },
                2 + w as usize,
            ))
        }
        // INT
        0b11001100..=0b11001101 => {
            let specified = (opcode & 0b00000001) != 0;
            if bytes.len() < (1 + specified as usize) {
                return Err(ParseError::UnexpectedEOF);
            }

            let int_type = if specified { bytes[1] } else { 3 };

            Ok((IR::Int { int_type }, 1 + specified as usize))
        }
        _ => Err(ParseError::InvalidOpcode(opcode)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_instruction_mov_imm_reg() {
        // 8 bits
        let bytes = [0xb0, 0x00];
        let expected_result = (
            IR::Mov {
                dest: Operand::Register(Register::AL),
                src: Operand::Immediate(0x00),
            },
            2,
        );
        assert_eq!(parse_instruction(&bytes), Ok(expected_result));

        // 16 bits
        let bytes = [0xbb, 0x00, 0x00];
        let expected_result = (
            IR::Mov {
                dest: Operand::Register(Register::BX),
                src: Operand::LongImmediate(0x0000),
            },
            3,
        );
        assert_eq!(parse_instruction(&bytes), Ok(expected_result));
    }

    #[test]
    fn test_parse_instruction_mov_imm_reg_unexpected_eof() {
        // Test parsing MOV instruction with immediate value and register operand, but with insufficient bytes
        let bytes = [0b10110000];
        assert_eq!(parse_instruction(&bytes), Err(ParseError::UnexpectedEOF));
    }

    #[test]
    fn test_parse_instruction_invalid_opcode() {
        // Test parsing an invalid opcode
        let bytes = [0xFF];
        assert_eq!(
            parse_instruction(&bytes),
            Err(ParseError::InvalidOpcode(0xFF))
        );
    }
}
