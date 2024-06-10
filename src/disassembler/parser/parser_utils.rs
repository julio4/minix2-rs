use crate::disassembler::error::DisassemblerError;
use crate::x86::{Displacement, Memory, Operand, Register, IR};

impl Operand {
    /// Parse a ModRM byte and return the corresponding operand.
    /// This can consume additional bytes from the instruction stream.
    pub fn parse_modrm(
        mod_: u8,
        rm: u8,
        bytes: &[u8],
        w: bool,
    ) -> Result<(Operand, usize), DisassemblerError> {
        match mod_ {
            0b11 => Ok((Operand::Register(Register::from(rm, w)), 0)),
            0b00 => {
                // Special case *: EA = disp-high;disp-low
                if rm == 0b110 {
                    // parse next 2 bytes
                    if bytes.len() < 3 {
                        return Err(DisassemblerError::UnexpectedEOF);
                    }
                    Ok((
                        Operand::Memory(Memory {
                            base: None,
                            index: None,
                            disp: Some(Displacement::Long(i16::from_le_bytes([
                                bytes[1], bytes[2],
                            ]))),
                        }),
                        2,
                    ))
                } else {
                    Ok((
                        Operand::Memory(Memory {
                            base: Register::get_base(rm),
                            index: Register::get_index(rm),
                            disp: None,
                        }),
                        0,
                    ))
                }
            }
            0b01 => {
                // parse next byte
                if bytes.len() < 2 {
                    return Err(DisassemblerError::UnexpectedEOF);
                }
                // sign extended to i16
                let disp = Displacement::Long(bytes[1] as i8 as i16);
                return Ok((
                    Operand::Memory(Memory {
                        base: Register::get_base(rm),
                        index: Register::get_index(rm),
                        disp: Some(disp),
                    }),
                    1,
                ));
            }
            0b10 => {
                // parse next 2 bytes
                if bytes.len() < 3 {
                    return Err(DisassemblerError::UnexpectedEOF);
                }
                return Ok((
                    Operand::Memory(Memory {
                        base: Register::get_base(rm),
                        index: Register::get_index(rm),
                        disp: Some(Displacement::Long(i16::from_le_bytes([bytes[1], bytes[2]]))),
                    }),
                    2,
                ));
            }
            _ => Err(DisassemblerError::InvalidModRM),
        }
    }
}

/// Parse the given byte as:
/// 76  543 210
/// mod reg r/m
/// And return the operands (dest, src, bytes_consumed)
/// Warning: This will consume FROM the given byte slice (be sure that bytes[0] is the modrm byte)
pub fn parse_mod_reg_rm_bytes(
    bytes: &[u8],
    w: bool,
) -> Result<(Operand, Operand, usize), DisassemblerError> {
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
pub fn _parse_w_mod_reg_rm_bytes(
    bytes: &[u8],
) -> Result<(Operand, Operand, usize), DisassemblerError> {
    if bytes.len() < 2 {
        return Err(DisassemblerError::UnexpectedEOF);
    }
    let w = (bytes[0] & 0x1) != 0;
    let (reg, rm, bytes_consumed) = parse_mod_reg_rm_bytes(&bytes[1..], w)?;
    Ok((reg, rm, bytes_consumed + 1))
}

/// Parse the given two bytes as:
/// 76543210  76  543 210
/// ------dw  mod reg r/m
/// And return the operands (dest, src, bytes_consumed)
pub fn parse_dw_mod_reg_rm_bytes(
    bytes: &[u8],
) -> Result<(Operand, Operand, usize), DisassemblerError> {
    if bytes.len() < 2 {
        return Err(DisassemblerError::UnexpectedEOF);
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
pub fn parse_disp_bytes(bytes: &[u8], ip: usize) -> Result<(Operand, usize), DisassemblerError> {
    if bytes.len() < 2 {
        return Err(DisassemblerError::UnexpectedEOF);
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
pub fn parse_accumulator(
    data: &[u8],
) -> Result<(Operand, Operand, bool, usize), DisassemblerError> {
    let w = (data[0] & 0x1) != 0;
    if data.len() < 2 + w as usize {
        return Err(DisassemblerError::UnexpectedEOF);
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
pub fn parse_word_disp_bytes(
    bytes: &[u8],
    ip: usize,
) -> Result<(Operand, usize), DisassemblerError> {
    if bytes.len() < 3 {
        return Err(DisassemblerError::UnexpectedEOF);
    }
    Ok((
        Operand::Displacement(Displacement::Long(
            i16::from_le_bytes([bytes[1], bytes[2]]) + ip as i16 + 3,
        )),
        3,
    ))
}

pub fn parse_string_manipulation_ir_from(byte: u8) -> Result<IR, DisassemblerError> {
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
        _ => Err(DisassemblerError::InvalidOpcode(byte)),
    }
}
