use std::error::Error;
use std::fmt;

#[derive(Debug, Clone)]
pub struct InvalidInstructionError {
    opcode: i64,
    position: usize,
}

impl InvalidInstructionError {
    pub fn new(opcode: i64, position: usize) -> Self {
        Self { opcode, position }
    }
}

impl Error for InvalidInstructionError {}

impl fmt::Display for InvalidInstructionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Invalid instruction {} at position {}",
            self.opcode, self.position
        )
    }
}

#[derive(Debug, Clone)]
pub struct InvalidAddressError {
    position: usize,
}

impl InvalidAddressError {
    pub fn new(position: usize) -> Self {
        Self { position }
    }
}

impl Error for InvalidAddressError {}

impl fmt::Display for InvalidAddressError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Invalid address {}", self.position)
    }
}

#[derive(Debug)]
pub enum ProgramError {
    Address(InvalidAddressError),
    Instruction(InvalidInstructionError),
}

impl From<InvalidInstructionError> for ProgramError {
    fn from(e: InvalidInstructionError) -> Self {
        ProgramError::Instruction(e)
    }
}

impl From<InvalidAddressError> for ProgramError {
    fn from(e: InvalidAddressError) -> Self {
        ProgramError::Address(e)
    }
}

impl fmt::Display for ProgramError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ProgramError::Address(e) => write!(f, "{e}"),
            ProgramError::Instruction(e) => write!(f, "{e}"),
        }
    }
}

impl Error for ProgramError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ProgramError::Address(e) => Some(e),
            ProgramError::Instruction(e) => Some(e),
        }
    }
}
