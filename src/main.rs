use std::fs;

use clap::Parser;
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
    loop {
        let line = rl.readline(">> ")?;
        if line.trim().is_empty() {
            continue;
        }

        run(line);
    }
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
