use crate::{error::VmError, opcode::Opcode, value::Value};

const REGISTER_SIZE: usize = u8::MAX as usize;

/// A struct that represents a virtual machine instance.
pub struct VirtualMachine {
    stack: Vec<Value>,
    register: [Value; REGISTER_SIZE],
    bytecode: Vec<u8>,
    program_counter: usize,
}

impl VirtualMachine {
    /// Creates a new instance of virtual machine.
    pub fn new(bytecode: Vec<u8>) -> Self {
        Self {
            stack: vec![],
            register: [0u64; REGISTER_SIZE],
            bytecode,
            program_counter: 0,
        }
    }

    pub fn get_opcode_from_bytecode(&mut self) -> Option<Opcode> {
        let opcode = self
            .bytecode
            .get(self.program_counter)
            .map(|byte| byte.clone().into());

        self.program_counter += 1;

        opcode
    }

    pub fn get_value_from_bytecode(&mut self) -> Result<Value, VmError> {
        let value = self
            .bytecode
            .get(self.program_counter..self.program_counter + 8)
            .map(|bytes| {
                bytes
                    .try_into()
                    .ok()
                    .map(|bytes| Value::from_le_bytes(bytes))
            })
            .flatten();

        self.program_counter += 8;

        value.ok_or(VmError::NoValueInBytecode)
    }

    pub fn get_index_from_bytecode(&mut self) -> Result<u8, VmError> {
        let index = self
            .bytecode
            .get(self.program_counter)
            .map(|byte| byte.clone());

        self.program_counter += 1;

        index.ok_or(VmError::NoIndexInBytecode)
    }

    pub fn run(&mut self) -> Result<&[Value], VmError> {
        while let Some(opcode) = self.get_opcode_from_bytecode() {
            match opcode {
                Opcode::INVALID => return Err(VmError::InvalidOpcode),
                Opcode::PUSH => {
                    let value = self.get_value_from_bytecode()?;
                    self.stack.push(value);
                }
                Opcode::POP => {
                    self.stack.pop().ok_or(VmError::NoValueInStack)?;
                }
                Opcode::STORE => {
                    let value = self.stack.pop().ok_or(VmError::NoValueInStack)?;
                    let index = self.get_index_from_bytecode()?;
                    self.register[index as usize] = value;
                }
                Opcode::LOAD => {
                    let index = self.get_index_from_bytecode()?;
                    let value = self.register[index as usize];
                    self.stack.push(value);
                }
                Opcode::ADD => {
                    let value_1 = self.stack.pop().ok_or(VmError::NoValueInStack)?;
                    let value_2 = self.stack.pop().ok_or(VmError::NoValueInStack)?;
                    let value_3 = value_1 + value_2;
                    self.stack.push(value_3);
                }

                Opcode::SUB => {
                    let value_1 = self.stack.pop().ok_or(VmError::NoValueInStack)?;
                    let value_2 = self.stack.pop().ok_or(VmError::NoValueInStack)?;
                    let value_3 = value_1 - value_2;
                    self.stack.push(value_3)
                }

                Opcode::MUL => {
                    let value_1 = self.stack.pop().ok_or(VmError::NoValueInStack)?;
                    let value_2 = self.stack.pop().ok_or(VmError::NoValueInStack)?;
                    let value_3 = value_1 * value_2;
                    self.stack.push(value_3);
                }
                Opcode::DIV => {
                    let value_1 = self.stack.pop().ok_or(VmError::NoValueInStack)?;
                    let value_2 = self.stack.pop().ok_or(VmError::NoValueInStack)?;
                    let value_3 = value_1 / value_2;
                    self.stack.push(value_3);
                }
                Opcode::MOD => {
                    let value_1 = self.stack.pop().ok_or(VmError::NoValueInStack)?;
                    let value_2 = self.stack.pop().ok_or(VmError::NoValueInStack)?;
                    let value_3 = value_1 % value_2;
                    self.stack.push(value_3);
                }
                Opcode::RET => {
                    return Ok(&self.stack);
                }
            }
        }

        Err(VmError::NoBytecodeToExecute)
    }
}

#[test]
fn test_bytecode() {
    let mut bytecode: Vec<u8> = vec![];

    bytecode.push(Opcode::PUSH.into());
    bytecode.extend_from_slice(&100_u64.to_le_bytes());
    bytecode.push(Opcode::PUSH.into());
    bytecode.extend_from_slice(&200_u64.to_le_bytes());
    bytecode.push(Opcode::ADD.into());
    bytecode.push(Opcode::STORE.into());
    bytecode.push(0);

    bytecode.push(Opcode::PUSH.into());
    bytecode.extend_from_slice(&20_u64.to_le_bytes());
    bytecode.push(Opcode::PUSH.into());
    bytecode.extend_from_slice(&40_u64.to_le_bytes());
    bytecode.push(Opcode::ADD.into());
    bytecode.push(Opcode::STORE.into());
    bytecode.push(1);

    bytecode.push(Opcode::LOAD.into());
    bytecode.push(0);
    bytecode.push(Opcode::LOAD.into());
    bytecode.push(1);
    bytecode.push(Opcode::ADD.into());

    bytecode.push(Opcode::RET.into());

    let mut virtual_machine = VirtualMachine::new(bytecode);

    let result = virtual_machine.run().unwrap();

    assert_eq!(result, &[360_u64])
}
