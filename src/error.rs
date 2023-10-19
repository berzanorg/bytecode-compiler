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

#[derive(Debug)]
pub enum ParseError<'a> {
    MistakenOpcode(&'a str),
    OpcodeRequired(&'a str),
    ValueRequired(&'a str),
    IndexRequired(&'a str),
    MistakenValue(&'a str),
    MistakenIndex(&'a str),
}

impl<'a> Display for ParseError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MistakenOpcode(mistaken_opcode) => write!(
                f,
                "PARSING ERROR: `{mistaken_opcode}` is not a valid opcode"
            ),
            ParseError::OpcodeRequired(number_string) => write!(
                f,
                "PARSING ERROR: an opcode is required before `{number_string}`"
            ),
            ParseError::ValueRequired(opcode_string) => write!(
                f,
                "PARSING ERROR: a value is required after `{opcode_string}`"
            ),
            ParseError::IndexRequired(opcode_string) => write!(
                f,
                "PARSING ERROR: an index is required after `{opcode_string}`"
            ),
            ParseError::MistakenValue(value_string) => {
                write!(f, "PARSING ERROR: `{value_string}` is not a valid value")
            }
            ParseError::MistakenIndex(index_string) => {
                write!(f, "PARSING ERROR: `{index_string}` is not a valid index")
            }
        }
    }
}
