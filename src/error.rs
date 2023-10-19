use std::fmt::Display;

#[derive(Debug)]
pub enum VmError {
    NoBytecodeToExecute,
    NoIndexInBytecode,
    NoValueInBytecode,
    NoValueInStack,
    InvalidOpcode,
}

impl Display for VmError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Self::NoBytecodeToExecute => "there is no bytecode to execute",
            Self::NoIndexInBytecode => "there is no index in bytecode",
            Self::NoValueInBytecode => "there is no value in bytecode",
            Self::NoValueInStack => "there is no value in stack",
            Self::InvalidOpcode => "there is an invalid opcode",
        };

        write!(f, "RUNTIME ERROR: {}", msg)
    }
}
