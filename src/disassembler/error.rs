#[derive(Debug, PartialEq)]
pub enum DisassemblerError {
    InvalidArgs,
    InvalidOpcode(u8),
    UnexpectedEOF,
    InvalidModRM,
}

impl std::fmt::Display for DisassemblerError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            DisassemblerError::InvalidArgs => write!(f, "Invalid arguments"),
            DisassemblerError::InvalidOpcode(opcode) => {
                write!(f, "Invalid opcode: {:#04x}", opcode)
            }
            DisassemblerError::UnexpectedEOF => write!(f, "Unexpected end of file"),
            DisassemblerError::InvalidModRM => write!(f, "Invalid ModRM byte"),
        }
    }
}
