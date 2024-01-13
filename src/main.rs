use std::fs;

use clap::Parser;
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};

mod cli;
mod lexer;
mod token;
mod token_type;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let args = cli::Cli::parse();

    if args.is_empty() {
        // TODO: Print header information before prompt.
        run_prompt().unwrap();
    } else {
        match args.file {
            Some(file) => run_file(file),
            None => {}
        }
    }
}

/// Reads a source file and executes it.
fn run_file(path: String) {
    let contents = fs::read_to_string(path).expect("Unable to read file");
    run(contents);
}

// TODO: Refactor this function cuz it's getting too damn big.
/// Runs the interpreter interactively.
fn run_prompt() -> Result<()> {
    let mut running = true;
    let mut rl = DefaultEditor::new()?;

    let repl_template = format!(
        "\
        Welcome to Chonk {}.\n\
        Type \".help\" for more information.\
        ",
        VERSION
    );

    println!("{}", repl_template);

    // TODO: Create a template for `help` command.

    Ok(while running {
        let readline = rl.readline(">> ");
        match readline {
            Ok(mut line) => {
                line = line.trim().to_string();
                if line.is_empty() {
                    continue;
                }

                // Terminate program on exit command
                if line == ".exit" {
                    running = false;
                    continue;
                }

                run(line.clone());
                let _ = rl.add_history_entry(line);
            }
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                running = false;
            }
            Err(error) => {
                eprintln!("Error: {error:?}");
                running = false;
            }
        }
    })
}

/// Runs `Chonk` code.
fn run(source: String) {
    let tokens = lexer::scan_tokens(source);

    // NOTE: For now, just print the tokens.
    for token in tokens.iter() {
        println!("{:#?}", token);
    }
}

fn error(line: usize, message: &str) {
    report(line, "", message);
}

fn report(line: usize, location: &str, message: &str) {
    eprintln!("[line {}] Error{}: {}", line, location, message);
}
