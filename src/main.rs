use std::{env, fs, process::ExitCode};

mod ast;
mod codegen;
mod lexer;
mod parser;

fn main() -> ExitCode {
    let mut args = env::args();
    let prog_name = args.next().unwrap_or("bettr".to_string());

    let Some(src_path) = args.next() else {
        eprintln!("Usage: {} <source file>", prog_name);
        return ExitCode::FAILURE;
    };

    let src = match fs::read_to_string(&src_path) {
        Ok(src) => src,
        Err(err) => {
            eprintln!("Error reading source file: {}", err);
            return ExitCode::FAILURE;
        }
    };

    let mut parser = match parser::Parser::new(&src) {
        Ok(parser) => parser,
        Err(err) => {
            eprintln!("Error initializing parser: {:?}", err);
            return ExitCode::FAILURE;
        }
    };

    let asm = match parser.parse() {
        Ok(mut program) => {
            program.normalize();
            codegen::generate(&program)
        }
        Err(err) => {
            eprintln!("Error parsing source file: {:?}", err);
            return ExitCode::FAILURE;
        }
    };

    let asm_path = src_path.replace(".bt", ".s");
    match fs::write(&asm_path, asm) {
        Ok(_) => {
            println!("Assembly code written to {}", asm_path);
        }
        Err(err) => {
            eprintln!("Error writing assembly file: {}", err);
            return ExitCode::FAILURE;
        }
    }

    ExitCode::SUCCESS
}
