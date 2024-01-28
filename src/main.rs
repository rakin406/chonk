use std::fs;
use std::path::Path;

use clap::Parser;
use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;

mod cli;
mod internal;

use internal::*;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let args = cli::Cli::parse();
    let interpreter = interpreter::Interpreter::default();

    if args.is_empty() {
        // TODO: Create a template for `help` command.
        println!(
            "\
            Welcome to Chonk {}.\n\
            Type \".help\" for more information.\
            ",
            VERSION
        );
        run_prompt();
    } else if let Some(file) = args.file {
        run_file(file);
    }
}

/// Reads a source file and executes it.
fn run_file(path: String) {
    let contents = fs::read_to_string(path).expect("Unable to read file");
    run(contents);
}

// TODO: Create a separate repl.rs which contains this function and other repl
// related functions. The problem is that this function needs the `run()`
// function which is defined in this file. Hmm...
/// Runs the interpreter interactively.
fn run_prompt() {
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

                run(line);
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
fn run(input: String) {
    // I know this looks weird :/
    let mut lexer = lexer::Lexer::new(input);
    let tokens = lexer.scan_tokens();

    // NOTE: This snippet is purely for printing the tokens.
    // Print the tokens
    // for token in tokens.iter() {
    //     println!("{:#?}", token);
    // }

    let mut parser = parser::Parser::new(tokens);

    // Check for parser error
    match parser.parse_program() {
        Ok(stmts) => INTERPRETER.interpret(stmts),
        Err(error) => eprintln!("{error:?}"),
    }
}
