use std::{env, fs, process::ExitCode};

mod ast;
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

    let mut parser = parser::Parser::new(&src);

    match parser.parse() {
        Ok(program) => println!("{program:#?}"),
        Err(err) => {
            eprintln!("Error parsing source file: {:?}", err);
            return ExitCode::FAILURE;
        }
    }

    ExitCode::SUCCESS
}
