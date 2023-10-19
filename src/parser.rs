use crate::{error::ParseError, lexer::Token, value::Value};

/// It represent expressions in virtual machine's assembly language.
#[derive(Debug, PartialEq)]
pub enum Expression {
    PUSH(Value),
    POP,
    STORE(u8),
    LOAD(u8),
    ADD,
    SUB,
    MUL,
    DIV,
    MOD,
    RET,
}

/// Parses tokens into expressions.
pub fn parse(tokens: Vec<Token>) -> Result<Vec<Expression>, ParseError> {
    let mut expressions = vec![];

    let mut tokens_iter = tokens.into_iter();

    while let Some(token) = tokens_iter.next() {
        match token {
            Token::Number(number_string) => return Err(ParseError::OpcodeRequired(number_string)),
            Token::Opcode(opcode_string) => match opcode_string {
                "PUSH" => {
                    let next_token = tokens_iter
                        .next()
                        .ok_or(ParseError::ValueRequired("PUSH"))?;
                    let value: Value = match next_token {
                        Token::Number(number_string) => number_string
                            .parse()
                            .map_err(|_| ParseError::MistakenValue(number_string))?,
                        _ => return Err(ParseError::ValueRequired("PUSH")),
                    };
                    expressions.push(Expression::PUSH(value))
                }
                "POP" => expressions.push(Expression::POP),
                "STORE" => {
                    let next_token = tokens_iter
                        .next()
                        .ok_or(ParseError::IndexRequired("STORE"))?;
                    let index: u8 = match next_token {
                        Token::Number(number_string) => number_string
                            .parse()
                            .map_err(|_| ParseError::MistakenIndex(number_string))?,
                        _ => return Err(ParseError::IndexRequired("STORE")),
                    };
                    expressions.push(Expression::STORE(index))
                }
                "LOAD" => {
                    let next_token = tokens_iter
                        .next()
                        .ok_or(ParseError::IndexRequired("LOAD"))?;
                    let index: u8 = match next_token {
                        Token::Number(number_string) => number_string
                            .parse()
                            .map_err(|_| ParseError::MistakenIndex(number_string))?,
                        _ => return Err(ParseError::IndexRequired("LOAD")),
                    };
                    expressions.push(Expression::LOAD(index))
                }
                "ADD" => expressions.push(Expression::ADD),
                "SUB" => expressions.push(Expression::SUB),
                "MUL" => expressions.push(Expression::MUL),
                "DIV" => expressions.push(Expression::DIV),
                "MOD" => expressions.push(Expression::MOD),
                "RET" => expressions.push(Expression::RET),
                _ => return Err(ParseError::MistakenOpcode(opcode_string)),
            },
        }
    }

    Ok(expressions)
}

#[test]
fn test_parsing() {
    let tokens = vec![
        Token::Opcode("PUSH"),
        Token::Number("10"),
        Token::Opcode("PUSH"),
        Token::Number("40"),
        Token::Opcode("ADD"),
        Token::Opcode("STORE"),
        Token::Number("0"),
        Token::Opcode("PUSH"),
        Token::Number("6"),
        Token::Opcode("PUSH"),
        Token::Number("-2"),
        Token::Opcode("SUB"),
        Token::Opcode("STORE"),
        Token::Number("1"),
        Token::Opcode("PUSH"),
        Token::Number("10"),
        Token::Opcode("PUSH"),
        Token::Number("20"),
        Token::Opcode("DIV"),
        Token::Opcode("LOAD"),
        Token::Number("0"),
        Token::Opcode("LOAD"),
        Token::Number("1"),
        Token::Opcode("MUL"),
        Token::Opcode("RET"),
    ];

    let expressions = parse(tokens).unwrap();

    assert_eq!(
        &expressions,
        &[
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
        ]
    )
}
