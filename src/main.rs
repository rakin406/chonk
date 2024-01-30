use std::fs;
use std::path::Path;

use clap::Parser;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

mod cli;
mod internal;

use internal::{interpreter, parser};

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let args = cli::Cli::parse();
    let mut interpreter = interpreter::Interpreter::default();

    if args.is_empty() {
        // TODO: Create a template for `help` command.
        println!(
            "\
            Welcome to Chonk {}.\n\
            Type \".help\" for more information.\
            ",
            VERSION
        );
        run_prompt(&mut interpreter);
    } else if let Some(file) = args.file {
        run_file(&mut interpreter, file);
    }
}

/// Reads a source file and executes it.
fn run_file(interpreter: &mut interpreter::Interpreter, path: String) {
    let contents = fs::read_to_string(path).expect("Unable to read file");
    run(interpreter, contents);
}

/// Runs the interpreter interactively.
fn run_prompt(interpreter: &mut interpreter::Interpreter) {
    let mut running = true;
    let mut rl = DefaultEditor::new().unwrap();

    let mut history_path = String::new();
    if let Some(path) = home::home_dir() {
        history_path = format!("{}/.chonk_history", path.to_str().unwrap());
    }

    // Load REPL history
    if Path::new(&history_path).try_exists().is_ok() {
        let _ = rl.load_history(&history_path);
    }

    while running {
        let readline = rl.readline(">> ");
        match readline {
            Ok(mut line) => {
                line = line.trim().to_string();
                if line.is_empty() {
                    continue;
                }

                // Save REPL history
                rl.add_history_entry(&line).unwrap();
                rl.save_history(&history_path).unwrap();

                // Terminate program on exit command
                if line == ".exit" {
                    running = false;
                    continue;
                }

                // This is to prevent the parser failing to find newline token
                line.push('\n');

                run(interpreter, line);
            }
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                running = false;
            }
            Err(error) => {
                eprintln!("Error: {error:?}");
                running = false;
            }
        }
    }
}

/// Runs `Chonk` code.
fn run(interpreter: &mut interpreter::Interpreter, input: String) {
    let mut parser = parser::Parser::new(input);

    match parser.parse() {
        Ok(program) => {
            if let Err(error) = interpreter.interpret(program) {
                eprintln!("{error:?}");
            }
        }
        Err(error) => eprintln!("{error:?}"),
    }
}
