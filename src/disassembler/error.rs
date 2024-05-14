#[derive(Debug, PartialEq)]
pub enum ParseError {
    InvalidSize,
    CorruptedData,
    InvalidOpcode(u8),
    UnexpectedEOF,
    InvalidModRM,
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ParseError::InvalidSize => write!(f, "Invalid size"),
            ParseError::CorruptedData => write!(f, "Corrupted data"),
            ParseError::InvalidOpcode(opcode) => write!(f, "Invalid opcode: {:#04x}", opcode),
            ParseError::UnexpectedEOF => write!(f, "Unexpected end of file"),
            ParseError::InvalidModRM => write!(f, "Invalid ModRM byte"),
        }
    }
}
