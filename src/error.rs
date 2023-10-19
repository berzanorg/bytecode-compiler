use std::fmt::Display;

#[derive(Debug)]
pub enum VmError {
    RetOpcodeNotFound,
    NoIndexInBytecode,
    NoValueInBytecode,
    NoValueInStack,
    InvalidOpcode,
}

impl Display for VmError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let msg = match self {
            Self::RetOpcodeNotFound => "there is no `RET` opcode",
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

pub enum UserError<'a> {
    FileNotFound(&'a str),
    NoFilenameGiven,
}

impl<'a> Display for UserError<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UserError::FileNotFound(file_name) => {
                write!(f, "USER ERROR: `{file_name}` is not found")
            }
            UserError::NoFilenameGiven => {
                write!(f, "USER ERROR: no file name is given")
            }
        }
    }
}
