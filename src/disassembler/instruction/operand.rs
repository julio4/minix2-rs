use crate::disassembler::{error::ParseError, Displacement, Memory, Register};

#[derive(Debug, PartialEq)]
pub enum Operand {
    Register(Register),
    Immediate(u8),
    LongImmediate(u16),
    Memory(Memory),
    Displacement(Displacement),
    // LongDisplacement(i16),
}

impl Operand {
    /// Parse a ModRM byte and return the corresponding operand.
    /// This can consume additional bytes from the instruction stream.
    pub fn parse_modrm(
        mod_: u8,
        rm: u8,
        bytes: &[u8],
        w: bool,
    ) -> Result<(Operand, usize), ParseError> {
        match mod_ {
            0b11 => Ok((Operand::Register(Register::from(rm, w)), 0)),
            0b00 => {
                // Special case *: EA = disp-high;disp-low
                if rm == 0b110 {
                    // parse next 2 bytes
                    if bytes.len() < 3 {
                        return Err(ParseError::UnexpectedEOF);
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
                    return Err(ParseError::UnexpectedEOF);
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
                    return Err(ParseError::UnexpectedEOF);
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
            _ => Err(ParseError::InvalidModRM),
        }
    }
}

impl std::fmt::Display for Operand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operand::Register(r) => write!(f, "{}", r),
            Operand::Immediate(i) => write!(f, "{:x}", i),
            Operand::LongImmediate(i) => write!(f, "{:04x}", i),
            Operand::Memory(mem) => write!(f, "{}", mem),
            Operand::Displacement(d) => match d {
                Displacement::Short(d) => write!(f, "{:02x}", d),
                Displacement::Long(d) => write!(f, "{:04x}", d),
            },
        }
    }
}
