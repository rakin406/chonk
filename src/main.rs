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
    let args = Args::parse();
    println!("The filename is {}", args.file);
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
