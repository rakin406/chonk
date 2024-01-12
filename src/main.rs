use std::fs;

use clap::Parser;
use rustyline::error::ReadlineError;
use rustyline::{DefaultEditor, Result};

mod lexer;
mod token;
mod token_type;

#[derive(Parser)]
#[command(version)]
struct Args {
    /// Path of the script file to run
    #[arg(required(false))]
    file: String,
}

fn main() -> Result<()> {
    let args = Args::try_parse();
    match args {
        Ok(value) => Ok(run_file(value.file)),
        // TODO: Print header information before prompt.
        Err(_) => run_prompt(),
    }
}

/// Read a source file and execute it.
fn run_file(path: String) {
    let contents = fs::read_to_string(path).expect("Unable to read file");
    run(contents);
}

/// Run the interpreter interactively.
fn run_prompt() -> Result<()> {
    let mut rl = DefaultEditor::new()?;

    Ok(loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                if line.trim().is_empty() {
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

/// Run `chonk` code.
fn run(source: String) {
    let mut lexer = lexer::Lexer::new(source);
    let tokens = lexer.scan_tokens();

    // NOTE: For now, just print the tokens.
    for token in tokens.iter() {
        println!("{:#?}", token);
    }
}
