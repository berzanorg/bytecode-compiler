/// It represents each part of the syntax.
#[derive(Debug, PartialEq)]
pub enum Token<'a> {
    Number(&'a str),
    Opcode(&'a str),
}

/// Converts source code into tokens.
pub fn tokenize<'a>(source_code: &'a str) -> Vec<Token<'a>> {
    let mut tokens = vec![];
    let mut current_index = 0;

    let mut number_start_index: Option<usize> = None;
    let mut opcode_start_index: Option<usize> = None;

    while let Some(&char) = source_code.as_bytes().get(current_index) {
        match char {
            b' ' | b'\n' => {
                if let Some(start_index) = opcode_start_index {
                    let opcode = &source_code[start_index..current_index];
                    tokens.push(Token::Opcode(opcode));
                    opcode_start_index = None;
                }

                if let Some(start_index) = number_start_index {
                    let number = &source_code[start_index..current_index];
                    tokens.push(Token::Number(number));
                    number_start_index = None;
                }
            }
            b'0'..=b'9' | b'-' => {
                if let None = number_start_index {
                    number_start_index = Some(current_index);
                }
            }
            _ => {
                if let None = opcode_start_index {
                    opcode_start_index = Some(current_index);
                }
            }
        };
        current_index += 1;
    }

    if let Some(start_index) = opcode_start_index {
        let opcode = &source_code[start_index..current_index];
        tokens.push(Token::Opcode(opcode));
    }

    if let Some(start_index) = number_start_index {
        let number = &source_code[start_index..current_index];
        tokens.push(Token::Number(number));
    }

    tokens
}

#[test]
fn test_tokenization() {
    let source_code = "
    PUSH 10
    PUSH 40
    ADD
    STORE 0
    PUSH 6
    PUSH -2
    SUB
    STORE 1
    PUSH 10
    PUSH 20
    DIV
    LOAD 0
    LOAD 1
    MUL
    RET
    ";

    let tokens = tokenize(source_code);

    assert_eq!(
        &tokens,
        &[
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
            Token::Opcode("RET")
        ],
    )
}
