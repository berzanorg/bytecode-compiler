use crate::error::VmError;

/// An enum that represents opcode type for the virtual machine.
#[derive(Clone, Copy)]
#[repr(u8)]
pub enum Opcode {
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

impl TryFrom<u8> for Opcode {
    /// Converts to a `Opcode` from `u8` value.
    type Error = VmError;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::PUSH),
            1 => Ok(Self::POP),
            2 => Ok(Self::STORE),
            3 => Ok(Self::LOAD),
            4 => Ok(Self::ADD),
            5 => Ok(Self::SUB),
            6 => Ok(Self::MUL),
            7 => Ok(Self::DIV),
            8 => Ok(Self::MOD),
            9 => Ok(Self::RET),
            _ => Err(VmError::InvalidOpcode),
        }
    }
}

impl From<Opcode> for u8 {
    /// Converts to a `u8` from `Opcode` value.
    fn from(value: Opcode) -> Self {
        value as u8
    }
}
