use std::fs;
use std::io::{self, Write};

use clap::Parser;

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

fn main() {
    let args = Args::try_parse();
    match args {
        Ok(value) => run_file(value.file),
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
fn run_prompt() {
    let mut running = true;

    while running {
        let mut line = String::new();
        print!(">> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut line).unwrap();

        if line.trim().is_empty() {
            running = false;
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
