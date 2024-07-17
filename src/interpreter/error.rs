#[derive(Debug, PartialEq)]
pub enum OpcodeExecErrors {
    ExitCatch,
    UnimplementedSyscall(usize),
    UnimplementedInterrupt(usize),
    DivideError,
}

impl std::fmt::Display for OpcodeExecErrors {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            OpcodeExecErrors::ExitCatch => write!(f, "Exit catch"),
            OpcodeExecErrors::UnimplementedSyscall(n) => write!(f, "Unimplemented syscall {}", n),
            OpcodeExecErrors::UnimplementedInterrupt(n) => {
                write!(f, "Unimplemented interrupt {}", n)
            }
            OpcodeExecErrors::DivideError => write!(f, "Divide error"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum InterpreterError {
    InvalidArgs,
    CycleLimitExceeded,
    OpcodeExecutionError(OpcodeExecErrors),
}

impl std::fmt::Display for InterpreterError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            InterpreterError::InvalidArgs => write!(f, "Invalid arguments"),
            InterpreterError::CycleLimitExceeded => write!(f, "Cycle limit exceeded"),
            InterpreterError::OpcodeExecutionError(e) => write!(f, "Execution error: {}", e),
        }
    }
}
