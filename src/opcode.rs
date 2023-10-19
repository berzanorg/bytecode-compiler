/// An enum that represents opcode type for the virtual machine.
#[derive(Clone, Copy)]
#[repr(u8)]
pub enum Opcode {
    INVALID = 0,
    PUSH,
    POP,
    STORE,
    LOAD,
    ADD,
    SUB,
    MUL,
    DIV,
    MOD,
    RET,
}

impl From<u8> for Opcode {
    /// Converts to a `Opcode` from `u8` value.
    fn from(value: u8) -> Self {
        match value {
            1 => Self::PUSH,
            2 => Self::POP,
            3 => Self::STORE,
            4 => Self::LOAD,
            5 => Self::ADD,
            6 => Self::SUB,
            7 => Self::MUL,
            8 => Self::DIV,
            9 => Self::MOD,
            10 => Self::RET,
            _ => Self::INVALID,
        }
    }
}

impl From<Opcode> for u8 {
    /// Converts to a `u8` from `Opcode` value.
    fn from(value: Opcode) -> Self {
        value as u8
    }
}
