#[derive(Debug, PartialEq)]
pub enum MinixError {
    InvalidFile,
    InvalidSize,
    CorruptedData,
}

impl std::fmt::Display for MinixError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            MinixError::InvalidFile => write!(f, "Invalid file"),
            MinixError::InvalidSize => write!(f, "Invalid size"),
            MinixError::CorruptedData => write!(f, "Corrupted data"),
        }
    }
}
