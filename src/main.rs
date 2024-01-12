use std::fs;

use clap::{Parser, Subcommand};
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};

mod cli;
mod lexer;
mod token;
mod token_type;

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

/// Runs the interpreter interactively.
fn run_prompt() -> Result<()> {
    let mut rl = DefaultEditor::new()?;

    Ok(loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(mut line) => {
                line = line.trim().to_string();
                if line.is_empty() {
                    continue;
                }

                run(line.clone());
                let _ = rl.add_history_entry(line);
            }
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                break;
            }
            Err(error) => {
                eprintln!("Error: {error:?}");
                break;
            }
        }
    })
}

/// Runs `Chonk` code.
fn run(source: String) {
    let mut lexer = lexer::Lexer::new(source);
    let tokens = lexer.scan_tokens();

    // NOTE: For now, just print the tokens.
    for token in tokens.iter() {
        println!("{:#?}", token);
    }
}
