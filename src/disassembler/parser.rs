use crate::disassembler::{error::ParseError, instruction::Operand, Instruction, Register, IR};

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
pub fn parse_instruction(bytes: &[u8]) -> Result<(Instruction, usize), ParseError> {
    if bytes.is_empty() {
        return Err(ParseError::UnexpectedEOF);
    }

    let opcode = bytes[0];
    let ir = match opcode {
        // MOV r/m, r/e
        0b10001000..=0b10001011 => {
            let (dest, src, bytes_consumed) = parse_dw_mod_reg_rm_bytes(bytes)?;
            Ok((IR::Mov { dest, src }, bytes_consumed))
        }
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
        // ADD r/m, r/e
        0b00000000..=0b00000011 => {
            let (dest, src, bytes_consumed) = parse_dw_mod_reg_rm_bytes(bytes)?;
            Ok((IR::Add { dest, src }, bytes_consumed))
        }
        // SUB r/m, r/e
        0b00101000..=0b00101011 => {
            let (dest, src, bytes_consumed) = parse_dw_mod_reg_rm_bytes(bytes)?;
            Ok((IR::Sub { dest, src }, bytes_consumed))
        }
        // SSB r/m, r/e
        0b00011000..=0b00011011 => {
            let (dest, src, bytes_consumed) = parse_dw_mod_reg_rm_bytes(bytes)?;
            Ok((IR::Ssb { dest, src }, bytes_consumed))
        }
        // CMP r/m, r/e
        0b00111000..=0b00111011 => {
            let (dest, src, bytes_consumed) = parse_dw_mod_reg_rm_bytes(bytes)?;
            Ok((IR::Cmp { dest, src }, bytes_consumed))
        }
        // AND r/m, r/e
        0b00100000..=0b00100011 => {
            let (dest, src, bytes_consumed) = parse_dw_mod_reg_rm_bytes(bytes)?;
            Ok((IR::And { dest, src }, bytes_consumed))
        }
        // OR r/m, r/e
        0b00001000..=0b00001011 => {
            let (dest, src, bytes_consumed) = parse_dw_mod_reg_rm_bytes(bytes)?;
            Ok((IR::Or { dest, src }, bytes_consumed))
        }
        // XOR r/m, r/e
        0b00110000..=0b00110011 => {
            let (dest, src, bytes_consumed) = parse_dw_mod_reg_rm_bytes(bytes)?;
            Ok((IR::Xor { dest, src }, bytes_consumed))
        }
        // LEA
        0b10001101 => {
            let (dest, src, bytes_consumed) = parse_mod_reg_rm_bytes(&bytes[1..], true)?;
            Ok((IR::Lea { dest, src }, bytes_consumed + 1))
        }
        // LDS
        0b11000101 => {
            let (dest, src, bytes_consumed) = parse_mod_reg_rm_bytes(&bytes[1..], true)?;
            Ok((IR::Lds { dest, src }, bytes_consumed + 1))
        }
        // LES
        0b11000100 => {
            let (dest, src, bytes_consumed) = parse_mod_reg_rm_bytes(&bytes[1..], true)?;
            Ok((IR::Les { dest, src }, bytes_consumed + 1))
        }
        // Immediate with/to Register/Memory
        0b10000000..=0b10000011 => {
            let s = (opcode & 0b00000010) != 0;
            let w = (opcode & 0b00000001) != 0;
            let is_word_data = !s && w;
            let total_consumed = 3 + is_word_data as usize;
            if bytes.len() < total_consumed {
                return Err(ParseError::UnexpectedEOF);
            }

            let data = if is_word_data {
                Operand::LongImmediate(u16::from_le_bytes([bytes[2], bytes[3]]))
            } else {
                Operand::Immediate(u8::from_le_bytes([bytes[2]]).into())
            };

            let (_, rm, bytes_consumed) = parse_mod_reg_rm_bytes(&bytes[1..], w)?;
            // next byte is the immediate data, so we should have consumed 1 bytes only
            if bytes_consumed != 1 {
                return Err(ParseError::InvalidOpcode(bytes[1]));
            }

            // We need bits 5-2 from bytes 2
            let bits = (bytes[1] & 0b00111000) >> 3;
            match bits {
                // ADD Imm to r/m
                0b000 => Ok((
                    IR::Add {
                        dest: rm,
                        src: data,
                    },
                    total_consumed,
                )),
                // ADC Imm to r/m
                0b010 => {
                    unimplemented!()
                }
                // SUB Imm from r/m
                0b101 => Ok((
                    IR::Sub {
                        dest: rm,
                        src: data,
                    },
                    total_consumed,
                )),
                // SSB Imm from r/m
                0b011 => {
                    unimplemented!()
                }
                // CMP Imm with r/m
                0b111 => Ok((
                    IR::Cmp {
                        dest: rm,
                        src: data,
                    },
                    total_consumed,
                )),
                _ => Err(ParseError::InvalidOpcode(bytes[1])),
            }
        }
        _ => Err(ParseError::InvalidOpcode(opcode)),
    };

    ir.map(|(ir, bytes_consumed)| {
        (
            Instruction::new(ir, bytes[..bytes_consumed].to_vec()),
            bytes_consumed,
        )
    })
}

/// Parse the given byte as:
/// 76  543 210
/// mod reg r/m
/// And return the corresponding operands
/// Warning: This will consume FROM the given byte slice (be sure that bytes[0] is the modrm byte)
fn parse_mod_reg_rm_bytes(bytes: &[u8], w: bool) -> Result<(Operand, Operand, usize), ParseError> {
    let mod_ = (bytes[0] & 0b11000000) >> 6;
    let rm = bytes[0] & 0b00000111;
    let reg = Register::from((bytes[0] & 0b00111000) >> 3, w);

    let reg = Operand::Register(reg);
    let (rm, bytes_consumed) = Operand::parse_modrm(mod_, rm, bytes, w)?;
    Ok((reg, rm, bytes_consumed + 1))
}

/// See `parse_mod_reg_rm_bytes`, but with first w byte:
/// 76543210  76  543 210
/// -------w  mod reg r/m
fn _parse_w_mod_reg_rm_bytes(bytes: &[u8]) -> Result<(Operand, Operand, usize), ParseError> {
    if bytes.len() < 2 {
        return Err(ParseError::UnexpectedEOF);
    }
    let w = (bytes[0] & 0b00000001) != 0;
    let (reg, rm, bytes_consumed) = parse_mod_reg_rm_bytes(&bytes[1..], w)?;
    Ok((reg, rm, bytes_consumed + 1))
}

/// Parse the given two bytes as:
/// 76543210  76  543 210
/// ------dw  mod reg r/m
/// And return the corresponding operands
fn parse_dw_mod_reg_rm_bytes(bytes: &[u8]) -> Result<(Operand, Operand, usize), ParseError> {
    if bytes.len() < 2 {
        return Err(ParseError::UnexpectedEOF);
    }
    let d: bool = (bytes[0] & 0b00000010) != 0;
    let w = (bytes[0] & 0b00000001) != 0;
    let (reg, rm, bytes_consumed) = parse_mod_reg_rm_bytes(&bytes[1..], w)?;

    let (dest, src) = if d { (reg, rm) } else { (rm, reg) };
    Ok((dest, src, bytes_consumed + 1))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::disassembler::Memory;

    #[test]
    fn test_parse_instruction_mov_imm_reg() {
        // 8 bits
        let bytes = [0xb0, 0x00];
        let expected_result = (
            Instruction::new(
                IR::Mov {
                    dest: Operand::Register(Register::AL),
                    src: Operand::Immediate(0x00),
                },
                bytes.to_vec(),
            ),
            2,
        );
        assert_eq!(parse_instruction(&bytes), Ok(expected_result));

        // 16 bits
        let bytes = [0xbb, 0x00, 0x00];
        let expected_result = (
            Instruction::new(
                IR::Mov {
                    dest: Operand::Register(Register::BX),
                    src: Operand::LongImmediate(0x0000),
                },
                bytes.to_vec(),
            ),
            bytes.len(),
        );
        assert_eq!(parse_instruction(&bytes), Ok(expected_result));
    }

    #[test]
    fn test_parse_instruction_int() {
        // INT 0x03
        let bytes = [0xcc];
        let expected_result = (
            Instruction::new(IR::Int { int_type: 3 }, bytes.to_vec()),
            bytes.len(),
        );
        assert_eq!(parse_instruction(&bytes), Ok(expected_result));

        // INT 0x01
        let bytes = [0xcd, 0x01];
        let expected_result = (
            Instruction::new(IR::Int { int_type: 1 }, bytes.to_vec()),
            bytes.len(),
        );
        assert_eq!(parse_instruction(&bytes), Ok(expected_result));
    }

    #[test]
    fn test_parse_instruction_add() {
        // r/m and reg
        let bytes = [0x00, 0x00];
        let expected_result = (
            Instruction::new(
                IR::Add {
                    dest: Operand::Memory(Memory::new(Some(Register::BX), Some(Register::SI), 0)),
                    src: Operand::Register(Register::AL),
                },
                bytes.to_vec(),
            ),
            bytes.len(),
        );
        assert_eq!(parse_instruction(&bytes), Ok(expected_result));

        // Imm with r/m
        let bytes = [0x83, 0xc3, 0x14];
        let expected_result = (
            Instruction::new(
                IR::Add {
                    dest: Operand::Register(Register::BX),
                    src: Operand::Immediate(0x14),
                },
                bytes.to_vec(),
            ),
            bytes.len(),
        );
        assert_eq!(parse_instruction(&bytes), Ok(expected_result));
    }

    #[test]
    fn test_parse_instruction_sub() {
        // r/m and reg
        let bytes = [0x28, 0x00];
        let expected_result = (
            Instruction::new(
                IR::Sub {
                    dest: Operand::Memory(Memory::new(Some(Register::BX), Some(Register::SI), 0)),
                    src: Operand::Register(Register::AL),
                },
                bytes.to_vec(),
            ),
            bytes.len(),
        );
        assert_eq!(parse_instruction(&bytes), Ok(expected_result));

        // Imm from r/m
        let bytes = [0x83, 0xeb, 0x14];
        let expected_result = (
            Instruction::new(
                IR::Sub {
                    dest: Operand::Register(Register::BX),
                    src: Operand::Immediate(0x14),
                },
                bytes.to_vec(),
            ),
            bytes.len(),
        );
        assert_eq!(parse_instruction(&bytes), Ok(expected_result));
    }

    #[test]
    fn test_parse_instruction_ssb() {
        let bytes = [0x18, 0x00];
        let expected_result = (
            Instruction::new(
                IR::Ssb {
                    dest: Operand::Memory(Memory::new(Some(Register::BX), Some(Register::SI), 0)),
                    src: Operand::Register(Register::AL),
                },
                bytes.to_vec(),
            ),
            bytes.len(),
        );
        assert_eq!(parse_instruction(&bytes), Ok(expected_result));
    }

    #[test]
    fn test_parse_instruction_cmp() {
        // r/m and reg
        let bytes = [0x38, 0x00];
        let expected_result = (
            Instruction::new(
                IR::Cmp {
                    dest: Operand::Memory(Memory::new(Some(Register::BX), Some(Register::SI), 0)),
                    src: Operand::Register(Register::AL),
                },
                bytes.to_vec(),
            ),
            bytes.len(),
        );
        assert_eq!(parse_instruction(&bytes), Ok(expected_result));

        // Imm with r/m
        let bytes = [0x81, 0xfb, 0x14, 0x00];
        let expected_result = (
            Instruction::new(
                IR::Cmp {
                    dest: Operand::Register(Register::BX),
                    src: Operand::LongImmediate(0x0014),
                },
                bytes.to_vec(),
            ),
            bytes.len(),
        );
        assert_eq!(parse_instruction(&bytes), Ok(expected_result));
    }

    #[test]
    fn test_parse_instruction_and() {
        let bytes = [0x20, 0x00];
        let expected_result = (
            Instruction::new(
                IR::And {
                    dest: Operand::Memory(Memory::new(Some(Register::BX), Some(Register::SI), 0)),
                    src: Operand::Register(Register::AL),
                },
                bytes.to_vec(),
            ),
            bytes.len(),
        );
        assert_eq!(parse_instruction(&bytes), Ok(expected_result));
    }

    #[test]
    fn test_parse_instruction_or() {
        let bytes = [0x8, 0x00];
        let expected_result = (
            Instruction::new(
                IR::Or {
                    dest: Operand::Memory(Memory::new(Some(Register::BX), Some(Register::SI), 0)),
                    src: Operand::Register(Register::AL),
                },
                bytes.to_vec(),
            ),
            bytes.len(),
        );
        assert_eq!(parse_instruction(&bytes), Ok(expected_result));
    }

    #[test]
    fn test_parse_instruction_xor() {
        let bytes = [0x30, 0x00];
        let expected_result = (
            Instruction::new(
                IR::Xor {
                    dest: Operand::Memory(Memory::new(Some(Register::BX), Some(Register::SI), 0)),
                    src: Operand::Register(Register::AL),
                },
                bytes.to_vec(),
            ),
            bytes.len(),
        );
        assert_eq!(parse_instruction(&bytes), Ok(expected_result));
    }

    #[test]
    fn test_parse_instruction_lea() {
        let bytes = [0x8D, 0x57, 0x02];
        let expected_result = (
            Instruction::new(
                IR::Lea {
                    dest: Operand::Register(Register::DX),
                    src: Operand::Memory(Memory::new(Some(Register::BX), None, 2)),
                },
                bytes.to_vec(),
            ),
            bytes.len(),
        );
        assert_eq!(parse_instruction(&bytes), Ok(expected_result));
    }

    #[test]
    fn test_parse_instruction_lds() {
        let bytes = [0xC5, 0x57, 0x02];
        let expected_result = (
            Instruction::new(
                IR::Lds {
                    dest: Operand::Register(Register::DX),
                    src: Operand::Memory(Memory::new(Some(Register::BX), None, 2)),
                },
                bytes.to_vec(),
            ),
            bytes.len(),
        );
        assert_eq!(parse_instruction(&bytes), Ok(expected_result));
    }

    #[test]
    fn test_parse_instruction_les() {
        let bytes = [0xC4, 0x57, 0x02];
        let expected_result = (
            Instruction::new(
                IR::Les {
                    dest: Operand::Register(Register::DX),
                    src: Operand::Memory(Memory::new(Some(Register::BX), None, 2)),
                },
                bytes.to_vec(),
            ),
            bytes.len(),
        );
        assert_eq!(parse_instruction(&bytes), Ok(expected_result));
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

    #[test]
    fn test_parse_instruction_unexpected_eof() {
        let bytes = [0b10110000];
        assert_eq!(parse_instruction(&bytes), Err(ParseError::UnexpectedEOF));
    }
}
