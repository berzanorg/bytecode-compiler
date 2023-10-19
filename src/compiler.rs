use crate::{opcode::Opcode, parser::Expression};

/// Compiles expressions to bytecode.
pub fn compile(expressions: Vec<Expression>) -> Vec<u8> {
    let mut bytecode: Vec<u8> = vec![];

    for expression in expressions {
        match expression {
            Expression::PUSH(value) => {
                bytecode.push(Opcode::PUSH.into());
                bytecode.extend_from_slice(&value.to_le_bytes());
            }
            Expression::POP => bytecode.push(Opcode::POP.into()),
            Expression::STORE(index) => {
                bytecode.push(Opcode::STORE.into());
                bytecode.push(index);
            }
            Expression::LOAD(index) => {
                bytecode.push(Opcode::LOAD.into());
                bytecode.push(index);
            }
            Expression::ADD => bytecode.push(Opcode::ADD.into()),
            Expression::SUB => bytecode.push(Opcode::SUB.into()),
            Expression::MUL => bytecode.push(Opcode::MUL.into()),
            Expression::DIV => bytecode.push(Opcode::DIV.into()),
            Expression::MOD => bytecode.push(Opcode::MOD.into()),
            Expression::RET => bytecode.push(Opcode::RET.into()),
        }
    }

    bytecode
}

#[test]
fn test_compiling() {
    let expressions = vec![
        Expression::PUSH(10),
        Expression::PUSH(40),
        Expression::ADD,
        Expression::STORE(0),
        Expression::PUSH(6),
        Expression::PUSH(-2),
        Expression::SUB,
        Expression::STORE(1),
        Expression::PUSH(10),
        Expression::PUSH(20),
        Expression::DIV,
        Expression::LOAD(0),
        Expression::LOAD(1),
        Expression::MUL,
        Expression::RET,
    ];

    let bytecode = compile(expressions);

    assert_eq!(
        &bytecode,
        &[
            0, 10, 0, 0, 0, 0, 0, 0, 0, 0, 40, 0, 0, 0, 0, 0, 0, 0, 4, 2, 0, 0, 6, 0, 0, 0, 0, 0,
            0, 0, 0, 254, 255, 255, 255, 255, 255, 255, 255, 5, 2, 1, 0, 10, 0, 0, 0, 0, 0, 0, 0,
            0, 20, 0, 0, 0, 0, 0, 0, 0, 7, 3, 0, 3, 1, 6, 9
        ]
    );
}
