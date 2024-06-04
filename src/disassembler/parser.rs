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
        // ADD Imm to accumulator
        0x4 | 0x5 => {
            let (data, dest, _, bytes_consumed) = parse_accumulator(bytes)?;
            Ok((IR::Add { dest, src: data }, bytes_consumed))
        }
        // OR r/m, r/e
        0x8..=0xB => {
            let (dest, src, bytes_consumed) = parse_dw_mod_reg_rm_bytes(bytes)?;
            Ok((IR::Or { dest, src }, bytes_consumed))
        }
        // OR Imm to accumulator
        0x0C | 0x0D => {
            let (data, dest, _, bytes_consumed) = parse_accumulator(bytes)?;
            Ok((IR::Or { dest, src: data }, bytes_consumed))
        }
        // SSB Imm from accumulator
        0x0E | 0x0F => {
            let (data, dest, _, bytes_consumed) = parse_accumulator(bytes)?;
            Ok((IR::Ssb { dest, src: data }, bytes_consumed))
        }
        // ADC r/m, r/e
        0x11 => {
            let (dest, src, bytes_consumed) = parse_dw_mod_reg_rm_bytes(bytes)?;
            Ok((IR::Adc { dest, src }, bytes_consumed))
        }
        // ADC Imm to accumulator
        0x14 | 0x15 => {
            let (data, dest, _, bytes_consumed) = parse_accumulator(bytes)?;
            Ok((IR::Adc { dest, src: data }, bytes_consumed))
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
        // AND Imm to accumulator
        0x24 | 0x25 => {
            let (data, dest, _, bytes_consumed) = parse_accumulator(bytes)?;
            Ok((IR::And { dest, src: data }, bytes_consumed))
        }
        // BAA
        0x27 => Ok((IR::Baa, 1)),
        // SUB r/m, r/e
        0x28..=0x2b => {
            let (dest, src, bytes_consumed) = parse_dw_mod_reg_rm_bytes(bytes)?;
            Ok((IR::Sub { dest, src }, bytes_consumed))
        }
        // SUB Imm from accumulator
        0x2d | 0x2c => {
            let (data, dest, _, bytes_consumed) = parse_accumulator(bytes)?;
            Ok((IR::Sub { dest, src: data }, bytes_consumed))
        }
        // DAS
        0x2F => Ok((IR::Das, 1)),
        // XOR r/m, r/e
        0x30..=0x33 => {
            let (dest, src, bytes_consumed) = parse_dw_mod_reg_rm_bytes(bytes)?;
            Ok((IR::Xor { dest, src }, bytes_consumed))
        }
        // XOR Imm to accumulator
        0x34 | 0x35 => {
            let (data, dest, _, bytes_consumed) = parse_accumulator(bytes)?;
            Ok((IR::Xor { dest, src: data }, bytes_consumed))
        }
        // AAA
        0x37 => Ok((IR::Aaa, 1)),
        // CMP r/m, r/e
        0x38..=0x3b => {
            let (dest, src, bytes_consumed) = parse_dw_mod_reg_rm_bytes(bytes)?;
            Ok((
                IR::Cmp {
                    dest,
                    src,
                    byte: opcode & 0x1 == 0,
                },
                bytes_consumed,
            ))
        }
        // CMP Imm with accumulator
        0x3c | 0x3d => {
            let (data, dest, _, bytes_consumed) = parse_accumulator(bytes)?;
            Ok((
                IR::Cmp {
                    dest,
                    src: data,
                    byte: opcode & 0x1 == 0,
                },
                bytes_consumed,
            ))
        }
        // AAS
        0x3F => Ok((IR::Aas, 1)),
        // INC with reg
        0x40..=0x47 => {
            let reg = Register::from(opcode & 0x7, true);
            Ok((
                IR::Inc {
                    dest: Operand::Register(reg),
                },
                1,
            ))
        }
        // DEC with reg
        0x48..=0x4F => {
            let reg = Register::from(opcode & 0x7, true);
            Ok((
                IR::Dec {
                    dest: Operand::Register(reg),
                },
                1,
            ))
        }
        // PUSH reg
        0x50..=0x57 => {
            let reg = Register::from(opcode & 0x7, true);
            Ok((
                IR::Push {
                    src: Operand::Register(reg),
                },
                1,
            ))
        }
        // POP register
        0x58..=0x5F => {
            let reg = Register::from(opcode & 0x7, true);
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
            let s = (opcode & 0x2) != 0;
            let w = (opcode & 0x1) != 0;
            let is_word_data = !s && w;
            let total_consumed = 3 + is_word_data as usize;
            if bytes.len() < total_consumed {
                return Err(ParseError::UnexpectedEOF);
            }

            let (_, rm, bytes_consumed) = parse_mod_reg_rm_bytes(&bytes[1..], w)?;
            let data = match opcode & 0x3 {
                // !s w: 16 bits immediate data
                0b01 => Operand::LongImmediate(u16::from_le_bytes([
                    bytes[bytes_consumed + 1],
                    bytes[bytes_consumed + 2],
                ])),
                // s w: byte sign extended
                0b11 => Operand::SignExtendedImmediate(
                    i8::from_le_bytes([bytes[bytes_consumed + 1]]).into(),
                ),
                _ => Operand::Immediate(u8::from_le_bytes([bytes[bytes_consumed + 1]]).into()),
            };

            // We need bits 5-2 from bytes 2
            let bits = (bytes[1] & 0x38) >> 3;
            match bits {
                // ADD Imm to r/m
                0b000 => Ok((
                    IR::Add {
                        dest: rm,
                        src: data,
                    },
                    total_consumed + bytes_consumed - 1,
                )),
                // OR Imm to r/m
                0b001 => Ok((
                    IR::Or {
                        dest: rm,
                        src: data,
                    },
                    total_consumed + bytes_consumed - 1,
                )),
                // ADC Imm to r/m
                0b010 => Ok((
                    IR::Adc {
                        dest: rm,
                        src: data,
                    },
                    total_consumed + bytes_consumed - 1,
                )),
                // SUB Imm from r/m
                0b101 => Ok((
                    IR::Sub {
                        dest: rm,
                        src: data,
                    },
                    total_consumed + bytes_consumed - 1,
                )),
                // SSB Imm from r/m
                0b011 => Ok((
                    IR::Ssb {
                        dest: rm,
                        src: data,
                    },
                    total_consumed + bytes_consumed - 1,
                )),
                // AND Imm with r/m
                0b100 => Ok((
                    IR::And {
                        dest: rm,
                        src: data,
                    },
                    total_consumed + bytes_consumed - 1,
                )),
                // XOR Imm to r/m
                0b110 => Ok((
                    IR::Xor {
                        dest: rm,
                        src: data,
                    },
                    total_consumed + bytes_consumed - 1,
                )),
                // CMP Imm with r/m
                0b111 => Ok((
                    IR::Cmp {
                        dest: rm,
                        src: data,
                        byte: !w,
                    },
                    total_consumed + bytes_consumed - 1,
                )),
                _ => Err(ParseError::InvalidOpcode(bytes[1])),
            }
        }
        // TEST r/m, r/e
        0x84 | 0x85 => {
            let (dest, src, bytes_consumed) = parse_dw_mod_reg_rm_bytes(bytes)?;
            Ok((
                IR::Test {
                    dest,
                    src,
                    byte: opcode == 0x84,
                },
                bytes_consumed,
            ))
        }
        // MOV r/m, r/e
        0x88..=0x8b => {
            let (dest, src, bytes_consumed) = parse_dw_mod_reg_rm_bytes(bytes)?;
            Ok((
                IR::Mov {
                    dest,
                    src,
                    byte: false,
                },
                bytes_consumed,
            ))
        }
        // LEA
        0x8D => {
            let (dest, src, bytes_consumed) = parse_mod_reg_rm_bytes(&bytes[1..], true)?;
            Ok((IR::Lea { dest, src }, bytes_consumed + 1))
        }
        // CBW
        0x98 => Ok((IR::Cbw, 1)),
        // CWD
        0x99 => Ok((IR::Cwd, 1)),
        // AAM and AAD
        0xD4 | 0xD5 => {
            if bytes.len() < 2 {
                return Err(ParseError::UnexpectedEOF);
            }
            if bytes[1] != 0xA {
                return Err(ParseError::InvalidOpcode(bytes[0]));
            }

            match bytes[0] & 0x1 {
                // AAM
                0 => Ok((IR::Aam, 2)),
                // AAD
                1 => Ok((IR::Aad, 2)),
                _ => Err(ParseError::InvalidOpcode(bytes[0])),
            }
        }
        // WAIT
        0x9B => Ok((IR::Wait, 1)),
        // TEST Imm with accumulator
        0xa8 | 0xa9 => {
            let (data, dest, w, bytes_consumed) = parse_accumulator(bytes)?;
            Ok((
                IR::Test {
                    dest,
                    src: data,
                    byte: !w,
                },
                bytes_consumed,
            ))
        }
        // MOV imm, reg
        0xB0..=0xBF => {
            let w = (opcode & 0x8) != 0;
            if bytes.len() < (2 + w as usize) {
                return Err(ParseError::UnexpectedEOF);
            }

            let reg = Operand::Register(Register::from(opcode & 0x7, w));
            let data = match w {
                true => Operand::LongImmediate(u16::from_le_bytes([bytes[1], bytes[2]])),
                false => Operand::Immediate(bytes[1]),
            };

            Ok((
                IR::Mov {
                    dest: reg,
                    src: data,
                    byte: !w,
                },
                2 + w as usize,
            ))
        }
        // MOV Mem to accumulator
        0xA0 | 0xA1 => {
            let w = opcode == 0xA1;
            if bytes.len() < 3 {
                return Err(ParseError::UnexpectedEOF);
            };

            let dest = match w {
                true => Operand::Register(Register::AX),
                false => Operand::Register(Register::AL),
            };
            let addr = u16::from_le_bytes([bytes[1], bytes[2]]);

            Ok((
                IR::Mov {
                    dest,
                    src: Operand::LongImmediate(addr),
                    byte: !w,
                },
                3,
            ))
        }
        // MOV accumulator to mem
        0xA2 | 0xA3 => {
            let w = opcode == 0xA3;
            if bytes.len() < 3 {
                return Err(ParseError::UnexpectedEOF);
            };

            let src = match w {
                true => Operand::Register(Register::AX),
                false => Operand::Register(Register::AL),
            };
            let addr = u16::from_le_bytes([bytes[1], bytes[2]]);

            Ok((
                IR::Mov {
                    dest: Operand::LongImmediate(addr),
                    src,
                    byte: !w,
                },
                3,
            ))
        }
        // MOV r/m, Seg
        0x8E => {
            let (dest, src, bytes_consumed) = parse_mod_reg_rm_bytes(&bytes[1..], false)?;
            Ok((
                IR::Mov {
                    dest,
                    src,
                    byte: true,
                },
                bytes_consumed + 1,
            ))
        }
        // MOV Seg, r/m
        0x8C => {
            let (src, dest, bytes_consumed) = parse_mod_reg_rm_bytes(&bytes[1..], false)?;
            Ok((
                IR::Mov {
                    dest,
                    src,
                    byte: true,
                },
                bytes_consumed + 1,
            ))
        }
        // POP r/m
        0x8F => {
            let (_, src, bytes_consumed) = parse_mod_reg_rm_bytes(&bytes[1..], true)?;
            Ok((IR::Pop { dest: src }, bytes_consumed + 1))
        }
        // XCHG r/m, reg
        0x86 | 0x87 => {
            let (src, dest, bytes_consumed) = parse_dw_mod_reg_rm_bytes(bytes)?;
            Ok((IR::Xchg { dest, src }, bytes_consumed))
        }
        // XCHG with accumulator
        0x90..=0x97 => {
            let reg = Register::from(opcode & 0x7, true);
            Ok((
                IR::Xchg {
                    dest: Operand::Register(reg),
                    src: Operand::Register(Register::AX),
                },
                1,
            ))
        }
        // RET Within Seg Adding Immed to SP
        0xC2 => {
            if bytes.len() < 3 {
                return Err(ParseError::UnexpectedEOF);
            }
            let dest = u16::from_le_bytes([bytes[1], bytes[2]]);
            Ok((
                IR::Ret {
                    dest: Some(Operand::LongImmediate(dest)),
                },
                3,
            ))
        }
        // RET segment / intersegment
        0xC3 | 0xCB => Ok((IR::Ret { dest: None }, 1)),
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
        // mov r/m, imm
        0xc6 | 0xc7 => {
            let w = opcode == 0xc7;
            if bytes.len() < (3 + w as usize) {
                return Err(ParseError::UnexpectedEOF);
            }

            let (_, rm, bytes_consumed) = parse_mod_reg_rm_bytes(&bytes[1..], w)?;
            let data = match w {
                true => Operand::LongImmediate(u16::from_le_bytes([
                    bytes[bytes_consumed + 1],
                    bytes[bytes_consumed + 2],
                ])),
                false => Operand::Immediate(u8::from_le_bytes([bytes[bytes_consumed + 1]]).into()),
            };

            Ok((
                IR::Mov {
                    dest: rm,
                    src: data,
                    byte: !w,
                },
                2 + bytes_consumed + w as usize,
            ))
        }
        // INT
        0xcc..=0xcd => {
            let specified = (opcode & 0x1) != 0;
            if bytes.len() < (1 + specified as usize) {
                return Err(ParseError::UnexpectedEOF);
            }

            let int_type = if specified { bytes[1] } else { 3 };

            Ok((IR::Int { int_type }, 1 + specified as usize))
        }
        // INTO
        0xCE => Ok((IR::Into, 1)),
        // IRET
        0xCF => Ok((IR::Iret, 1)),
        // Logic instructions
        0xD0..=0xD3 => {
            // TODO: v = 0 "count" = 1, v = 1 "count" = CL
            let _v = (opcode & 0x1) != 0;
            let w = (opcode & 0x1) != 0;
            if bytes.len() < 2 {
                return Err(ParseError::UnexpectedEOF);
            }

            let (_, rm, bytes_consumed) = parse_mod_reg_rm_bytes(&bytes[1..], w)?;
            let bits = (bytes[1] & 0x38) >> 3;
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

            let w = (opcode & 0x1) != 0;
            let dest = match w {
                true => Operand::Register(Register::AX),
                false => Operand::Register(Register::AL),
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
            let w = (opcode & 0x1) != 0;
            let dest = match w {
                true => Operand::Register(Register::AX),
                false => Operand::Register(Register::AL),
            };

            Ok((
                IR::In {
                    dest,
                    src: Operand::Register(Register::DX),
                },
                1,
            ))
        }
        // LOCK
        0xF0 => Ok((IR::Lock, 1)),
        // ESC to external device
        0xD8..=0xDF => {
            let (_, rm, bytes_consumed) = parse_mod_reg_rm_bytes(&bytes[1..], true)?;
            Ok((IR::Esc { dest: rm }, 1 + bytes_consumed))
        }
        // REP
        0xf2 | 0xf3 => {
            let z = opcode == 0xf3;
            if bytes.len() < 2 {
                return Err(ParseError::UnexpectedEOF);
            }
            let ir = parse_string_manipulation_ir_from(bytes[1])?;
            Ok((
                IR::Rep {
                    z,
                    string_ir: Box::new(ir),
                },
                2,
            ))
        }
        0xa4..=0xa7 | 0xaa..=0xaf => {
            let ir = parse_string_manipulation_ir_from(opcode)?;
            Ok((ir, 1))
        }
        // CMC
        0xF5 => Ok((IR::Cmc, 1)),
        // HLT
        0xF4 => Ok((IR::Hlt, 1)),
        0xF6..=0xF7 => {
            // 1111011w opcode
            // atleast 2 bytes
            if bytes.len() < 2 {
                return Err(ParseError::UnexpectedEOF);
            }
            let w = (opcode & 0x1) != 0;

            let (_, rm, bytes_consumed) = parse_mod_reg_rm_bytes(&bytes[1..], w)?;

            // We need bits 5-2 from bytes 2
            let bits = (bytes[1] & 0x38) >> 3;
            match bits {
                // NEG
                0b011 => Ok((IR::Neg { dest: rm }, bytes_consumed + 1)),
                // MUL
                0b100 => Ok((IR::Mul { dest: rm }, bytes_consumed + 1)),
                // IMUL
                0b101 => Ok((IR::Imul { dest: rm }, bytes_consumed + 1)),
                // DIV
                0b110 => Ok((IR::Div { dest: rm }, bytes_consumed + 1)),
                // IDIV
                0b111 => Ok((IR::Idiv { dest: rm }, bytes_consumed + 1)),
                // NOT
                0b010 => Ok((IR::Not { dest: rm }, bytes_consumed + 1)),
                // TEST Imm and r/m
                0b000 => {
                    let data = match w {
                        true => Operand::LongImmediate(u16::from_le_bytes([
                            bytes[bytes_consumed + 1],
                            bytes[bytes_consumed + 2],
                        ])),
                        false => Operand::Immediate(
                            u8::from_le_bytes([bytes[bytes_consumed + 1]]).into(),
                        ),
                    };

                    Ok((
                        IR::Test {
                            dest: rm,
                            src: data,
                            byte: !w,
                        },
                        2 + bytes_consumed + w as usize,
                    ))
                }
                _ => Err(ParseError::InvalidOpcode(bytes[1])),
            }
        }
        // CLC
        0xF8 => Ok((IR::Clc, 1)),
        // STC
        0xF9 => Ok((IR::Stc, 1)),
        // CLI
        0xFA => Ok((IR::Cli, 1)),
        // STI
        0xfb => Ok((IR::Sti, 1)),
        // CLD
        0xfc => Ok((IR::Cld, 1)),
        // STD
        0xfd => Ok((IR::Std, 1)),
        0xFF => {
            if bytes.len() < 2 {
                return Err(ParseError::UnexpectedEOF);
            }

            let (_, rm, bytes_consumed) = parse_mod_reg_rm_bytes(&bytes[1..], true)?;
            let bits = (bytes[1] & 0x38) >> 3;
            match bits {
                // INC r/m
                0b000 => Ok((IR::Inc { dest: rm }, bytes_consumed + 1)),
                // DEC r/m
                0b001 => Ok((IR::Dec { dest: rm }, bytes_consumed + 1)),
                // CALL indirect w/ segment
                // CALL intersegment
                0b010 | 0b011 => Ok((IR::Call { dest: rm }, bytes_consumed + 1)),
                // JMP indirect w/ segment (100)
                // JMP intersegment (101)
                0b100 | 0b101 => Ok((
                    IR::Jmp {
                        dest: rm,
                        short: false,
                    },
                    bytes_consumed + 1,
                )),
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
    let mod_ = (bytes[0] & 0xC0) >> 6;
    let rm = bytes[0] & 0x7;
    let reg = Register::from((bytes[0] & 0x38) >> 3, w);

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
    let w = (bytes[0] & 0x1) != 0;
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
    let d: bool = (bytes[0] & 0x2) != 0;
    let w = (bytes[0] & 0x1) != 0;
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

// Parse the rest of the bytes as:
// 76543210  76543210 76543210
// -------w    data   if w: data
// And return the operand (data, dest, w, bytes_consumed)
fn parse_accumulator(data: &[u8]) -> Result<(Operand, Operand, bool, usize), ParseError> {
    let w = (data[0] & 0x1) != 0;
    if data.len() < 2 + w as usize {
        return Err(ParseError::UnexpectedEOF);
    }

    let data = match w {
        true => Operand::LongImmediate(u16::from_le_bytes([data[1], data[2]])),
        false => Operand::Immediate(data[1]),
    };

    let dest = match w {
        true => Operand::Register(Register::AX),
        false => Operand::Register(Register::AL),
    };

    Ok((data, dest, w, 2 + w as usize))
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

fn parse_string_manipulation_ir_from(byte: u8) -> Result<IR, ParseError> {
    let word = (byte & 0x1) != 0;
    match byte >> 1 {
        // MOVS
        0x52 => Ok(IR::Movs { word }),
        // CMPS
        0x53 => Ok(IR::Cmps { word }),
        // STOS
        0x55 => Ok(IR::Stos { word }),
        // LODS
        0x56 => Ok(IR::Lods { word }),
        // SCAS
        0x57 => Ok(IR::Scas { word }),
        _ => Err(ParseError::InvalidOpcode(byte)),
    }
}
