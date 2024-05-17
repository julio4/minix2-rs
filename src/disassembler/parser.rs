use crate::disassembler::{
    error::ParseError, instruction::Operand, Displacement, Instruction, Register, IR,
};

/// Parses the given byte slice and returns the parsed instruction along with the number of bytes consumed.
///
/// # Arguments
///
/// * `bytes` - The byte slice containing the instruction to parse.
/// * `ip` - The instruction pointer (address) of the instruction.
///
/// # Returns
///
/// Returns a `Result` containing a tuple with the parsed `Instruction` and the number of bytes consumed.
/// If parsing fails, a `ParseError` is returned.
pub fn parse_instruction(bytes: &[u8], ip: usize) -> Result<(Instruction, usize), ParseError> {
    if bytes.is_empty() {
        return Err(ParseError::UnexpectedEOF);
    }

    let opcode = bytes[0];
    let ir: Result<(IR, usize), ParseError> = match opcode {
        // ADD r/m, r/e
        0x0..=0x3 => {
            let (dest, src, bytes_consumed) = parse_dw_mod_reg_rm_bytes(bytes)?;
            Ok((IR::Add { dest, src }, bytes_consumed))
        }
        // OR r/m, r/e
        0x8..=0xB => {
            let (dest, src, bytes_consumed) = parse_dw_mod_reg_rm_bytes(bytes)?;
            Ok((IR::Or { dest, src }, bytes_consumed))
        }
        // SSB r/m, r/e
        0x18..=0x1b => {
            let (dest, src, bytes_consumed) = parse_dw_mod_reg_rm_bytes(bytes)?;
            Ok((IR::Ssb { dest, src }, bytes_consumed))
        }
        // AND r/m, r/e
        0x20..=0x23 => {
            let (dest, src, bytes_consumed) = parse_dw_mod_reg_rm_bytes(bytes)?;
            Ok((IR::And { dest, src }, bytes_consumed))
        }
        // SUB r/m, r/e
        0x28..=0x2b => {
            let (dest, src, bytes_consumed) = parse_dw_mod_reg_rm_bytes(bytes)?;
            Ok((IR::Sub { dest, src }, bytes_consumed))
        }
        // XOR r/m, r/e
        0x30..=0x33 => {
            let (dest, src, bytes_consumed) = parse_dw_mod_reg_rm_bytes(bytes)?;
            Ok((IR::Xor { dest, src }, bytes_consumed))
        }
        // CMP r/m, r/e
        0x38..=0x3b => {
            let (dest, src, bytes_consumed) = parse_dw_mod_reg_rm_bytes(bytes)?;
            Ok((IR::Cmp { dest, src }, bytes_consumed))
        }
        // DEC with reg
        0x48..=0x4F => {
            let reg = Register::from(opcode & 0b00000111, true);
            Ok((
                IR::Dec {
                    dest: Operand::Register(reg),
                },
                1,
            ))
        }
        // PUSH reg
        0x50..=0x57 => {
            let reg = Register::from(opcode & 0b00000111, true);
            Ok((
                IR::Push {
                    src: Operand::Register(reg),
                },
                1,
            ))
        }
        // POP register
        0x58..=0x5F => {
            let reg = Register::from(opcode & 0b00000111, true);
            Ok((
                IR::Pop {
                    dest: Operand::Register(reg),
                },
                1,
            ))
        }
        // JO
        0x70 => {
            let (dest, bytes_consumed) = parse_disp_bytes(bytes, ip)?;
            Ok((IR::Jo { dest }, bytes_consumed))
        }
        // JNO
        0x71 => {
            let (dest, bytes_consumed) = parse_disp_bytes(bytes, ip)?;
            Ok((IR::Jno { dest }, bytes_consumed))
        }
        // JB/JNAE
        0x72 => {
            let (dest, bytes_consumed) = parse_disp_bytes(bytes, ip)?;
            Ok((IR::Jb { dest }, bytes_consumed))
        }
        // JNB/JAE
        0x73 => {
            let (dest, bytes_consumed) = parse_disp_bytes(bytes, ip)?;
            Ok((IR::Jnb { dest }, bytes_consumed))
        }
        // JE/JZ
        0x74 => {
            let (dest, bytes_consumed) = parse_disp_bytes(bytes, ip)?;
            Ok((IR::Je { dest }, bytes_consumed))
        }
        // JNE/JNZ
        0x75 => {
            let (dest, bytes_consumed) = parse_disp_bytes(bytes, ip)?;
            Ok((IR::Jne { dest }, bytes_consumed))
        }
        // JBE/JNA
        0x76 => {
            let (dest, bytes_consumed) = parse_disp_bytes(bytes, ip)?;
            Ok((IR::Jbe { dest }, bytes_consumed))
        }
        // JNBE/JA
        0x77 => {
            let (dest, bytes_consumed) = parse_disp_bytes(bytes, ip)?;
            Ok((IR::Jnbe { dest }, bytes_consumed))
        }
        // JS
        0x78 => {
            let (dest, bytes_consumed) = parse_disp_bytes(bytes, ip)?;
            Ok((IR::Js { dest }, bytes_consumed))
        }
        // JNS
        0x79 => {
            let (dest, bytes_consumed) = parse_disp_bytes(bytes, ip)?;
            Ok((IR::Jns { dest }, bytes_consumed))
        }
        // JP/JPE
        0x7A => {
            let (dest, bytes_consumed) = parse_disp_bytes(bytes, ip)?;
            Ok((IR::Jp { dest }, bytes_consumed))
        }
        // JNP/JPO
        0x7B => {
            let (dest, bytes_consumed) = parse_disp_bytes(bytes, ip)?;
            Ok((IR::Jnp { dest }, bytes_consumed))
        }
        // JL/JNGE
        0x7C => {
            let (dest, bytes_consumed) = parse_disp_bytes(bytes, ip)?;
            Ok((IR::Jl { dest }, bytes_consumed))
        }
        // JNL/JGE
        0x7D => {
            let (dest, bytes_consumed) = parse_disp_bytes(bytes, ip)?;
            Ok((IR::Jnl { dest }, bytes_consumed))
        }
        // JLE/JNG
        0x7E => {
            let (dest, bytes_consumed) = parse_disp_bytes(bytes, ip)?;
            Ok((IR::Jle { dest }, bytes_consumed))
        }
        // JNLE/JG
        0x7F => {
            let (dest, bytes_consumed) = parse_disp_bytes(bytes, ip)?;
            Ok((IR::Jnle { dest }, bytes_consumed))
        }
        // Immediate with/to Register/Memory
        0x80..=0x83 => {
            let s = (opcode & 0b00000010) != 0;
            let w = (opcode & 0b00000001) != 0;
            let is_word_data = !s && w;
            let total_consumed = 3 + is_word_data as usize;
            if bytes.len() < total_consumed {
                return Err(ParseError::UnexpectedEOF);
            }

            let (_, rm, bytes_consumed) = parse_mod_reg_rm_bytes(&bytes[1..], w)?;
            let data = if is_word_data {
                Operand::LongImmediate(u16::from_le_bytes([
                    bytes[bytes_consumed + 1],
                    bytes[bytes_consumed + 2],
                ]))
            } else {
                Operand::Immediate(u8::from_le_bytes([bytes[bytes_consumed + 1]]).into())
            };

            // We need bits 5-2 from bytes 2
            let bits = (bytes[1] & 0b00111000) >> 3;
            match bits {
                // ADD Imm to r/m
                0b000 => Ok((
                    IR::Add {
                        dest: rm,
                        src: data,
                    },
                    total_consumed + bytes_consumed - 1,
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
                    total_consumed + bytes_consumed - 1,
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
                    total_consumed + bytes_consumed - 1,
                )),
                _ => Err(ParseError::InvalidOpcode(bytes[1])),
            }
        }
        // MOV r/m, r/e
        0x88..=0x8b => {
            let (dest, src, bytes_consumed) = parse_dw_mod_reg_rm_bytes(bytes)?;
            Ok((IR::Mov { dest, src }, bytes_consumed))
        }
        // LEA
        0x8D => {
            let (dest, src, bytes_consumed) = parse_mod_reg_rm_bytes(&bytes[1..], true)?;
            Ok((IR::Lea { dest, src }, bytes_consumed + 1))
        }
        // MOV imm, reg
        0xb0..=0xbf => {
            let w = (opcode & 0b00001000) != 0;
            if bytes.len() < (2 + w as usize) {
                return Err(ParseError::UnexpectedEOF);
            }

            let reg = Operand::Register(Register::from(opcode & 0b00000111, w));
            let imm = if w {
                Operand::LongImmediate(u16::from_le_bytes([bytes[1], bytes[2]]))
            } else {
                Operand::Immediate(bytes[1])
            };

            Ok((
                IR::Mov {
                    dest: reg,
                    src: imm,
                },
                2 + w as usize,
            ))
        }
        // LES
        0xC4 => {
            let (dest, src, bytes_consumed) = parse_mod_reg_rm_bytes(&bytes[1..], true)?;
            Ok((IR::Les { dest, src }, bytes_consumed + 1))
        }
        // LDS
        0xC5 => {
            let (dest, src, bytes_consumed) = parse_mod_reg_rm_bytes(&bytes[1..], true)?;
            Ok((IR::Lds { dest, src }, bytes_consumed + 1))
        }
        // INT
        0xcc..=0xcd => {
            let specified = (opcode & 0b00000001) != 0;
            if bytes.len() < (1 + specified as usize) {
                return Err(ParseError::UnexpectedEOF);
            }

            let int_type = if specified { bytes[1] } else { 3 };

            Ok((IR::Int { int_type }, 1 + specified as usize))
        }
        // Logic instructions
        0xD0..=0xD3 => {
            // TODO: v = 0 "count" = 1, v = 1 "count" = CL
            let _v = (opcode & 0b00000010) != 0;
            let w = (opcode & 0b00000001) != 0;
            if bytes.len() < 2 {
                return Err(ParseError::UnexpectedEOF);
            }

            let (_, rm, bytes_consumed) = parse_mod_reg_rm_bytes(&bytes[1..], w)?;
            let bits = (bytes[1] & 0b00111000) >> 3;
            match bits {
                // SHL/SAL
                0b100 => Ok((
                    IR::Shl {
                        dest: rm,
                        src: Operand::Immediate(1),
                    },
                    bytes_consumed + 1,
                )),
                // SHR
                0b101 => Ok((
                    IR::Shr {
                        dest: rm,
                        src: Operand::Immediate(1),
                    },
                    bytes_consumed + 1,
                )),
                // SAR
                0b111 => Ok((
                    IR::Sar {
                        dest: rm,
                        src: Operand::Immediate(1),
                    },
                    bytes_consumed + 1,
                )),
                // ROL
                0b000 => Ok((
                    IR::Rol {
                        dest: rm,
                        src: Operand::Immediate(1),
                    },
                    bytes_consumed + 1,
                )),
                // ROR
                0b001 => Ok((
                    IR::Ror {
                        dest: rm,
                        src: Operand::Immediate(1),
                    },
                    bytes_consumed + 1,
                )),
                // RCL
                0b010 => Ok((
                    IR::Rcl {
                        dest: rm,
                        src: Operand::Immediate(1),
                    },
                    bytes_consumed + 1,
                )),
                // RCR
                0b011 => Ok((
                    IR::Rcr {
                        dest: rm,
                        src: Operand::Immediate(1),
                    },
                    bytes_consumed + 1,
                )),
                _ => Err(ParseError::InvalidOpcode(bytes[1])),
            }
        }
        // LOOPNZ/LOOPNE
        0xE0 => {
            let (dest, bytes_consumed) = parse_disp_bytes(bytes, ip)?;
            Ok((IR::Loopnz { dest }, bytes_consumed))
        }
        // LOOPZ/LOOPE
        0xE1 => {
            let (dest, bytes_consumed) = parse_disp_bytes(bytes, ip)?;
            Ok((IR::Loopz { dest }, bytes_consumed))
        }
        // LOOP
        0xE2 => {
            let (dest, bytes_consumed) = parse_disp_bytes(bytes, ip)?;
            Ok((IR::Loop { dest }, bytes_consumed))
        }
        // JCXZ
        0xE3 => {
            let (dest, bytes_consumed) = parse_disp_bytes(bytes, ip)?;
            Ok((IR::Jcxz { dest }, bytes_consumed))
        }
        // IN fixed port
        0xE4 | 0xE5 => {
            if bytes.len() < 2 {
                return Err(ParseError::UnexpectedEOF);
            }

            let w = (opcode & 0b00000001) != 0;
            let dest = if w {
                Operand::Register(Register::AX)
            } else {
                Operand::Register(Register::AL)
            };
            let port = Operand::Immediate(bytes[1]);

            Ok((IR::In { dest, src: port }, 2))
        }
        // CALL direct w/ segment
        0xE8 => {
            let (dest, bytes_consumed) = parse_word_disp_bytes(bytes, ip)?;
            Ok((IR::Call { dest }, bytes_consumed))
        }
        // JMP direct with segment
        0xE9 => {
            let (dest, bytes_consumed) = parse_word_disp_bytes(bytes, ip)?;
            Ok((IR::Jmp { dest, short: false }, bytes_consumed))
        }
        // JMP direct with short segment
        0xEB => {
            let (dest, bytes_consumed) = parse_disp_bytes(bytes, ip)?;
            Ok((IR::Jmp { dest, short: true }, bytes_consumed))
        }
        // IN variable port
        0xEC => {
            let w = (opcode & 0b00000001) != 0;
            let dest = if w {
                Operand::Register(Register::AX)
            } else {
                Operand::Register(Register::AL)
            };

            Ok((
                IR::In {
                    dest,
                    src: Operand::Register(Register::DX),
                },
                1,
            ))
        }
        // HLT
        0xF4 => Ok((IR::Hlt, 1)),
        0xF6..=0xF7 => {
            // 1111011w opcode
            // atleast 2 bytes
            if bytes.len() < 2 {
                return Err(ParseError::UnexpectedEOF);
            }
            let w = (opcode & 0b00000001) != 0;

            let (_, rm, bytes_consumed) = parse_mod_reg_rm_bytes(&bytes[1..], w)?;

            // We need bits 5-2 from bytes 2

            let bits = (bytes[1] & 0b00111000) >> 3;
            match bits {
                // NEG
                0b011 => Ok((IR::Neg { dest: rm }, bytes_consumed + 1)),
                // MUL
                0b100 => {
                    unimplemented!()
                }
                // IMUL
                0b101 => {
                    unimplemented!()
                }
                // DIV
                0b110 => {
                    unimplemented!()
                }
                // IDIV
                0b111 => {
                    unimplemented!()
                }
                // NOT
                0b010 => {
                    unimplemented!()
                }
                // TEST Imm and r/m
                0b000 => {
                    // next byte is data, so we should have consumed 1 bytes only
                    // also, we should have atleast 3 bytes (4 if word data)
                    if bytes_consumed != 1 || bytes.len() < (3 + w as usize) {
                        return Err(ParseError::InvalidOpcode(bytes[1]));
                    }
                    let data = if w {
                        Operand::LongImmediate(u16::from_le_bytes([bytes[2], bytes[3]]))
                    } else {
                        Operand::Immediate(u8::from_le_bytes([bytes[2]]).into())
                    };

                    Ok((
                        IR::Test {
                            dest: rm,
                            src: data,
                        },
                        3 + w as usize,
                    ))
                }
                _ => Err(ParseError::InvalidOpcode(bytes[1])),
            }
        }
        // RET segment / intersegment
        0xC3 | 0xCB => Ok((IR::Ret, 1)),
        0xFF => {
            if bytes.len() < 2 {
                return Err(ParseError::UnexpectedEOF);
            }

            let (_, rm, bytes_consumed) = parse_mod_reg_rm_bytes(&bytes[1..], true)?;
            let bits = (bytes[1] & 0b00111000) >> 3;
            match bits {
                // CALL indirect w/ segment
                // CALL intersegment
                0b010 | 0b011 => Ok((IR::Call { dest: rm }, bytes_consumed + 1)),
                // JMP indirect w/ segment
                0b100 => {
                    unimplemented!()
                }
                // JMP intersegment
                0b101 => {
                    unimplemented!()
                }
                // PUSH r/m
                0b110 => Ok((IR::Push { src: rm }, bytes_consumed + 1)),
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
/// And return the operands (dest, src, bytes_consumed)
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
/// And return the operands (dest, src, bytes_consumed)
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
/// And return the operands (dest, src, bytes_consumed)
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

/// Parse the given two bytes as:
/// 76543210  76543210
/// --------    disp
/// And return the operand (dest, bytes_consumed)
fn parse_disp_bytes(bytes: &[u8], ip: usize) -> Result<(Operand, usize), ParseError> {
    if bytes.len() < 2 {
        return Err(ParseError::UnexpectedEOF);
    }
    Ok((
        Operand::Displacement(Displacement::Long((bytes[1] as i8) as i16 + ip as i16 + 2)),
        2,
    ))
}

/// Parse the given three bytes as:
/// 76543210  76543210 76543210
/// --------  disp-low disp-high
/// And return the operand (dest, bytes_consumed)
fn parse_word_disp_bytes(bytes: &[u8], ip: usize) -> Result<(Operand, usize), ParseError> {
    if bytes.len() < 3 {
        return Err(ParseError::UnexpectedEOF);
    }
    Ok((
        Operand::Displacement(Displacement::Long(
            i16::from_le_bytes([bytes[1], bytes[2]]) + ip as i16 + 3,
        )),
        3,
    ))
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
        assert_eq!(parse_instruction(&bytes, 0), Ok(expected_result));

        // 16 bits
        let bytes = [0xbb, 0xFF, 0x00];
        let expected_result = (
            Instruction::new(
                IR::Mov {
                    dest: Operand::Register(Register::BX),
                    src: Operand::LongImmediate(0x00FF),
                },
                bytes.to_vec(),
            ),
            bytes.len(),
        );
        assert_eq!(parse_instruction(&bytes, 0), Ok(expected_result));
    }

    #[test]
    fn test_parse_instruction_int() {
        // INT 0x03
        let bytes = [0xcc];
        let expected_result = (
            Instruction::new(IR::Int { int_type: 3 }, bytes.to_vec()),
            bytes.len(),
        );
        assert_eq!(parse_instruction(&bytes, 0), Ok(expected_result));

        // INT 0x01
        let bytes = [0xcd, 0x01];
        let expected_result = (
            Instruction::new(IR::Int { int_type: 1 }, bytes.to_vec()),
            bytes.len(),
        );
        assert_eq!(parse_instruction(&bytes, 0), Ok(expected_result));
    }

    #[test]
    fn test_parse_instruction_add() {
        // r/m and reg
        let bytes = [0x00, 0x00];
        let expected_result = (
            Instruction::new(
                IR::Add {
                    dest: Operand::Memory(Memory::new(
                        Some(Register::BX),
                        Some(Register::SI),
                        None,
                    )),
                    src: Operand::Register(Register::AL),
                },
                bytes.to_vec(),
            ),
            bytes.len(),
        );
        assert_eq!(parse_instruction(&bytes, 0), Ok(expected_result));

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
        assert_eq!(parse_instruction(&bytes, 0), Ok(expected_result));
    }

    #[test]
    fn test_parse_instruction_sub() {
        // r/m and reg
        let bytes = [0x28, 0x00];
        let expected_result = (
            Instruction::new(
                IR::Sub {
                    dest: Operand::Memory(Memory::new(
                        Some(Register::BX),
                        Some(Register::SI),
                        None,
                    )),
                    src: Operand::Register(Register::AL),
                },
                bytes.to_vec(),
            ),
            bytes.len(),
        );
        assert_eq!(parse_instruction(&bytes, 0), Ok(expected_result));

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
        assert_eq!(parse_instruction(&bytes, 0), Ok(expected_result));
    }

    #[test]
    fn test_parse_instruction_ssb() {
        let bytes = [0x18, 0x00];
        let expected_result = (
            Instruction::new(
                IR::Ssb {
                    dest: Operand::Memory(Memory::new(
                        Some(Register::BX),
                        Some(Register::SI),
                        None,
                    )),
                    src: Operand::Register(Register::AL),
                },
                bytes.to_vec(),
            ),
            bytes.len(),
        );
        assert_eq!(parse_instruction(&bytes, 0), Ok(expected_result));
    }

    #[test]
    fn test_parse_instruction_cmp() {
        // r/m and reg
        let bytes = [0x38, 0x00];
        let expected_result = (
            Instruction::new(
                IR::Cmp {
                    dest: Operand::Memory(Memory::new(
                        Some(Register::BX),
                        Some(Register::SI),
                        None,
                    )),
                    src: Operand::Register(Register::AL),
                },
                bytes.to_vec(),
            ),
            bytes.len(),
        );
        assert_eq!(parse_instruction(&bytes, 0), Ok(expected_result));

        // Imm with r/m
        let bytes = [0x83, 0x7c, 0x02, 0x00];
        let expected_result = (
            Instruction::new(
                IR::Cmp {
                    dest: Operand::Memory(Memory::new(
                        Some(Register::SI),
                        None,
                        Some(Displacement::Long(0x2)),
                    )),
                    src: Operand::Immediate(0x00),
                },
                bytes.to_vec(),
            ),
            bytes.len(),
        );
        assert_eq!(parse_instruction(&bytes, 0), Ok(expected_result));
    }

    #[test]
    fn test_parse_instruction_and() {
        let bytes = [0x20, 0x00];
        let expected_result = (
            Instruction::new(
                IR::And {
                    dest: Operand::Memory(Memory::new(
                        Some(Register::BX),
                        Some(Register::SI),
                        None,
                    )),
                    src: Operand::Register(Register::AL),
                },
                bytes.to_vec(),
            ),
            bytes.len(),
        );
        assert_eq!(parse_instruction(&bytes, 0), Ok(expected_result));
    }

    #[test]
    fn test_parse_instruction_or() {
        let bytes = [0x8, 0x00];
        let expected_result = (
            Instruction::new(
                IR::Or {
                    dest: Operand::Memory(Memory::new(
                        Some(Register::BX),
                        Some(Register::SI),
                        None,
                    )),
                    src: Operand::Register(Register::AL),
                },
                bytes.to_vec(),
            ),
            bytes.len(),
        );
        assert_eq!(parse_instruction(&bytes, 0), Ok(expected_result));
    }

    #[test]
    fn test_parse_instruction_xor() {
        let bytes = [0x30, 0x00];
        let expected_result = (
            Instruction::new(
                IR::Xor {
                    dest: Operand::Memory(Memory::new(
                        Some(Register::BX),
                        Some(Register::SI),
                        None,
                    )),
                    src: Operand::Register(Register::AL),
                },
                bytes.to_vec(),
            ),
            bytes.len(),
        );
        assert_eq!(parse_instruction(&bytes, 0), Ok(expected_result));
    }

    #[test]
    fn test_parse_instruction_lea() {
        let bytes = [0x8D, 0x57, 0x02];
        let expected_result = (
            Instruction::new(
                IR::Lea {
                    dest: Operand::Register(Register::DX),
                    src: Operand::Memory(Memory::new(
                        Some(Register::BX),
                        None,
                        Some(Displacement::Long(0x2)),
                    )),
                },
                bytes.to_vec(),
            ),
            bytes.len(),
        );
        assert_eq!(parse_instruction(&bytes, 0), Ok(expected_result));
    }

    #[test]
    fn test_parse_instruction_lds() {
        let bytes = [0xC5, 0x57, 0x02];
        let expected_result = (
            Instruction::new(
                IR::Lds {
                    dest: Operand::Register(Register::DX),
                    src: Operand::Memory(Memory::new(
                        Some(Register::BX),
                        None,
                        Some(Displacement::Long(0x2)),
                    )),
                },
                bytes.to_vec(),
            ),
            bytes.len(),
        );
        assert_eq!(parse_instruction(&bytes, 0), Ok(expected_result));
    }

    #[test]
    fn test_parse_instruction_les() {
        let bytes = [0xC4, 0x57, 0x02];
        let expected_result = (
            Instruction::new(
                IR::Les {
                    dest: Operand::Register(Register::DX),
                    src: Operand::Memory(Memory::new(
                        Some(Register::BX),
                        None,
                        Some(Displacement::Long(0x2)),
                    )),
                },
                bytes.to_vec(),
            ),
            bytes.len(),
        );
        assert_eq!(parse_instruction(&bytes, 0), Ok(expected_result));
    }

    #[test]
    fn test_parse_instruction_jmp() {
        // short segment
        let bytes = [0xeb, 0x0f];
        let expected_result = (
            Instruction::new(
                IR::Jmp {
                    dest: Operand::Displacement(Displacement::Long(0x0f + 2)),
                    short: true,
                },
                bytes.to_vec(),
            ),
            bytes.len(),
        );
        assert_eq!(parse_instruction(&bytes, 0), Ok(expected_result));
        // segment
        let bytes = [0xe9, 0x57, 0x02];
        let expected_result = (
            Instruction::new(
                IR::Jmp {
                    dest: Operand::Displacement(Displacement::Long(0x0257 + 3)),
                    short: false,
                },
                bytes.to_vec(),
            ),
            bytes.len(),
        );
        assert_eq!(parse_instruction(&bytes, 0), Ok(expected_result));
    }

    #[test]
    fn test_parse_instruction_jnb() {
        let bytes = [0x73, 0x0f];
        let expected_result = (
            Instruction::new(
                IR::Jnb {
                    dest: Operand::Displacement(Displacement::Long(0x0f + 2)),
                },
                bytes.to_vec(),
            ),
            bytes.len(),
        );
        assert_eq!(parse_instruction(&bytes, 0), Ok(expected_result));
    }

    #[test]
    fn test_parse_instruction_test() {
        // Imm and r/m
        let bytes = [0xf6, 0xc3, 0x01];
        let expected_result = (
            Instruction::new(
                IR::Test {
                    dest: Operand::Register(Register::BL),
                    src: Operand::Immediate(0x01),
                },
                bytes.to_vec(),
            ),
            bytes.len(),
        );
        assert_eq!(parse_instruction(&bytes, 0), Ok(expected_result));
    }

    #[test]
    fn test_parse_instruction_push() {
        // reg
        let bytes = [0x50];
        let expected_result = (
            Instruction::new(
                IR::Push {
                    src: Operand::Register(Register::AX),
                },
                bytes.to_vec(),
            ),
            bytes.len(),
        );
        assert_eq!(parse_instruction(&bytes, 0), Ok(expected_result));

        // r/m
        let bytes = [0xff, 0x76, 0x04];
        let expected_result = (
            Instruction::new(
                IR::Push {
                    src: Operand::Memory(Memory::new(
                        Some(Register::BP),
                        None,
                        Some(Displacement::Long(0x4)),
                    )),
                },
                bytes.to_vec(),
            ),
            bytes.len(),
        );
        assert_eq!(parse_instruction(&bytes, 0), Ok(expected_result));
    }

    #[test]
    fn test_parse_instruction_call() {
        // direct w/ segment
        let bytes = [0xe8, 0x57, 0x02];
        let expected_result = (
            Instruction::new(
                IR::Call {
                    dest: Operand::Displacement(Displacement::Long(0x0257 + 3)),
                },
                bytes.to_vec(),
            ),
            bytes.len(),
        );
        assert_eq!(parse_instruction(&bytes, 0), Ok(expected_result));

        // indirect w/ segment
        // intersegment
        let bytes = [0xff, 0xd3];
        let expected_result = (
            Instruction::new(
                IR::Call {
                    dest: Operand::Register(Register::BX),
                },
                bytes.to_vec(),
            ),
            bytes.len(),
        );
        assert_eq!(parse_instruction(&bytes, 0), Ok(expected_result));
    }

    #[test]
    fn test_parse_instruction_dec() {
        // reg
        let bytes = [0x48];
        let expected_result = (
            Instruction::new(
                IR::Dec {
                    dest: Operand::Register(Register::AX),
                },
                bytes.to_vec(),
            ),
            bytes.len(),
        );
        assert_eq!(parse_instruction(&bytes, 0), Ok(expected_result));
    }

    #[test]
    fn test_parse_instruction_hlt() {
        let bytes = [0xf4];
        let expected_result = (Instruction::new(IR::Hlt, bytes.to_vec()), bytes.len());
        assert_eq!(parse_instruction(&bytes, 0), Ok(expected_result));
    }

    #[test]
    fn test_parse_instruction_shl() {
        let bytes = [0xd1, 0xe3];
        let expected_result = (
            Instruction::new(
                IR::Shl {
                    dest: Operand::Register(Register::BX),
                    src: Operand::Immediate(1),
                },
                bytes.to_vec(),
            ),
            bytes.len(),
        );
        assert_eq!(parse_instruction(&bytes, 0), Ok(expected_result));
    }

    #[test]
    fn test_parse_instruction_shr() {
        let bytes = [0xd1, 0xeb];
        let expected_result = (
            Instruction::new(
                IR::Shr {
                    dest: Operand::Register(Register::BX),
                    src: Operand::Immediate(1),
                },
                bytes.to_vec(),
            ),
            bytes.len(),
        );
        assert_eq!(parse_instruction(&bytes, 0), Ok(expected_result));
    }

    #[test]
    fn test_parse_instruction_sar() {
        let bytes = [0xd1, 0xff];
        let expected_result = (
            Instruction::new(
                IR::Sar {
                    dest: Operand::Register(Register::DI),
                    src: Operand::Immediate(1),
                },
                bytes.to_vec(),
            ),
            bytes.len(),
        );
        assert_eq!(parse_instruction(&bytes, 0), Ok(expected_result));
    }

    #[test]
    fn test_parse_instruction_rol() {
        let bytes = [0xd1, 0xc3];
        let expected_result = (
            Instruction::new(
                IR::Rol {
                    dest: Operand::Register(Register::BX),
                    src: Operand::Immediate(1),
                },
                bytes.to_vec(),
            ),
            bytes.len(),
        );
        assert_eq!(parse_instruction(&bytes, 0), Ok(expected_result));
    }

    #[test]
    fn test_parse_instruction_ror() {
        let bytes = [0xd1, 0xcb];
        let expected_result = (
            Instruction::new(
                IR::Ror {
                    dest: Operand::Register(Register::BX),
                    src: Operand::Immediate(1),
                },
                bytes.to_vec(),
            ),
            bytes.len(),
        );
        assert_eq!(parse_instruction(&bytes, 0), Ok(expected_result));
    }

    #[test]
    fn test_parse_instruction_rcl() {
        let bytes = [0xd1, 0xd3];
        let expected_result = (
            Instruction::new(
                IR::Rcl {
                    dest: Operand::Register(Register::BX),
                    src: Operand::Immediate(1),
                },
                bytes.to_vec(),
            ),
            bytes.len(),
        );
        assert_eq!(parse_instruction(&bytes, 0), Ok(expected_result));
    }

    #[test]
    fn test_parse_instruction_rcr() {
        let bytes = [0xd1, 0xdb];
        let expected_result = (
            Instruction::new(
                IR::Rcr {
                    dest: Operand::Register(Register::BX),
                    src: Operand::Immediate(1),
                },
                bytes.to_vec(),
            ),
            bytes.len(),
        );
        assert_eq!(parse_instruction(&bytes, 0), Ok(expected_result));
    }

    #[test]
    fn test_parse_instruction_pop() {
        // reg
        let bytes = [0x5b];
        let expected_result = (
            Instruction::new(
                IR::Pop {
                    dest: Operand::Register(Register::BX),
                },
                bytes.to_vec(),
            ),
            bytes.len(),
        );
        assert_eq!(parse_instruction(&bytes, 0), Ok(expected_result));
    }

    #[test]
    fn test_parse_instruction_in() {
        // variable port
        let bytes = [0xec];
        let expected_result = (
            Instruction::new(
                IR::In {
                    dest: Operand::Register(Register::AL),
                    src: Operand::Register(Register::DX),
                },
                bytes.to_vec(),
            ),
            bytes.len(),
        );
        assert_eq!(parse_instruction(&bytes, 0), Ok(expected_result));

        // fixed port
        let bytes = [0xe4, 0x01];
        let expected_result = (
            Instruction::new(
                IR::In {
                    dest: Operand::Register(Register::AL),
                    src: Operand::Immediate(0x01),
                },
                bytes.to_vec(),
            ),
            bytes.len(),
        );
        assert_eq!(parse_instruction(&bytes, 0), Ok(expected_result));
    }

    #[test]
    fn test_parse_instruction_neg() {
        let bytes = [0xf7, 0xda];
        let expected_result = (
            Instruction::new(
                IR::Neg {
                    dest: Operand::Register(Register::DX),
                },
                bytes.to_vec(),
            ),
            bytes.len(),
        );
        assert_eq!(parse_instruction(&bytes, 0), Ok(expected_result));
    }

    #[test]
    fn test_parse_instruction_invalid_opcode() {
        // Test parsing an invalid opcode
        let bytes = [0xFA];
        assert_eq!(
            parse_instruction(&bytes, 0),
            Err(ParseError::InvalidOpcode(0xFA))
        );
    }

    #[test]
    fn test_parse_instruction_unexpected_eof() {
        let bytes = [0b10110000];
        assert_eq!(parse_instruction(&bytes, 0), Err(ParseError::UnexpectedEOF));
    }
}
