use std::{env::args, ops::Deref};

use compiler::compile;
use error::UserError;
use lexer::tokenize;
use virtual_machine::VirtualMachine;

use crate::parser::parse;

mod compiler;
mod error;
mod lexer;
mod opcode;
mod parser;
mod value;
mod virtual_machine;

fn main() {
    let mut args = args();
    args.next();
    let args: Vec<String> = args.collect();
    let args: Vec<&str> = args.iter().map(|arg| arg.as_str()).collect();

    match args.deref() {
        ["run", file_path] => {
            let file_content = match std::fs::read_to_string(file_path) {
                Ok(content) => content,
                Err(_) => return eprintln!("{}", UserError::FileNotFound(file_path)),
            };

            let bytecode = if file_path.ends_with(".bin") {
                file_content.into_bytes()
            } else {
                let tokens = tokenize(&file_content);
                let expressions = match parse(tokens) {
                    Ok(expressions) => expressions,
                    Err(error) => return eprintln!("{error}"),
                };
                compile(expressions)
            };

            let mut virtual_machine = VirtualMachine::new(bytecode);

            match virtual_machine.run() {
                Ok(result) => {
                    println!("PROGRAM RESULT: {:#?}", result)
                }
                Err(error) => return eprintln!("{error}"),
            };
        }
        ["compile", file_path] => {
            let file_content = match std::fs::read_to_string(file_path) {
                Ok(content) => content,
                Err(_) => return eprintln!("{}", UserError::FileNotFound(&file_path)),
            };

            let bytecode = if file_path.ends_with(".bin") {
                return eprintln!("this file is already compiled");
            } else {
                let tokens = tokenize(&file_content);
                let expressions = match parse(tokens) {
                    Ok(expressions) => expressions,
                    Err(error) => return eprintln!("{error}"),
                };
                compile(expressions)
            };

            let file_name = std::path::Path::new(file_path)
                .file_name()
                .unwrap()
                .to_str()
                .unwrap();

            std::fs::write(format!("{file_name}.bin"), bytecode).unwrap();

            println!("program is compiled and `{file_name}.bin` is created")
        }

        ["run"] | ["compile"] => {
            let error = UserError::NoFilenameGiven;
            eprintln!("{error}");
        }
        _ => {
            eprintln!("This is my compiler.");
            eprintln!("Visit the repo for more info: <todo>");

            eprintln!("COMMANDS:");
            eprintln!("run <file>      runs the program");
            eprintln!(
                "compile <file>      compiles the program and creates a bytecode executable file"
            );
        }
    }
}
