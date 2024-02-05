use std::fs;

use crate::internal::{interpreter::Interpreter, parser::Parser};

/// Reads a source file and executes it.
pub fn run_file(path: &str) {
    let mut interpreter = Interpreter::new(false);
    let contents = fs::read_to_string(path).expect("Unable to read file");
    run(&contents, &mut interpreter);
}

/// Runs Chonk code.
pub fn run(input: &str, interpreter: &mut Interpreter) {
    let mut parser = Parser::new(input);

    match parser.parse() {
        Ok(statements) => {
            if let Err(error) = interpreter.interpret(&statements) {
                eprintln!("{error:?}");
            }
        }
        Err(error) => eprintln!("{error:?}"),
    }
}
