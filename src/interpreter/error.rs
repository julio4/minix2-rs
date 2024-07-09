#[derive(Debug, PartialEq)]
pub enum InterpreterError {
    InvalidArgs,
    CycleLimitExceeded,
}

impl std::fmt::Display for InterpreterError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            InterpreterError::InvalidArgs => write!(f, "Invalid arguments"),
            InterpreterError::CycleLimitExceeded => write!(f, "Cycle limit exceeded"),
        }
    }
}
