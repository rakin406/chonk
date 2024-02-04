use std::fs;
use std::path::Path;

use clap::Parser;
use rustyline::error::ReadlineError;
use rustyline::highlight::MatchingBracketHighlighter;
use rustyline::validate::MatchingBracketValidator;
use rustyline::Editor;
use rustyline::{Completer, Helper, Highlighter, Hinter, Validator};

mod internal;
use internal::{interpreter, parser};

const VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Parser)]
#[command(version)]
struct Args {
    /// Path of the script file to run
    file: Option<String>,
}

#[derive(Completer, Helper, Highlighter, Hinter, Validator)]
struct InputValidator {
    #[rustyline(Validator)]
    brackets: MatchingBracketValidator,
    #[rustyline(Highlighter)]
    highlighter: MatchingBracketHighlighter,
}

fn main() -> rustyline::Result<()> {
    let args = Args::parse();
    let mut interpreter = interpreter::Interpreter::default();

    if args.file.is_none() {
        // TODO: Create a template for `help` command.
        println!(
            "\
            Welcome to Chonk {}.\n\
            Type \".help\" for more information.\
            ",
            VERSION
        );
        run_prompt(&mut interpreter)?;
    } else if let Some(file) = args.file {
        run_file(&mut interpreter, &file);
    }

    Ok(())
}

/// Reads a source file and executes it.
fn run_file(interpreter: &mut interpreter::Interpreter, path: &str) {
    let contents = fs::read_to_string(path).expect("Unable to read file");
    run(interpreter, &contents);
}

/// Runs the interpreter interactively.
fn run_prompt(interpreter: &mut interpreter::Interpreter) -> rustyline::Result<()> {
    let helper = InputValidator {
        brackets: MatchingBracketValidator::new(),
        highlighter: MatchingBracketHighlighter::new(),
    };
    let mut rl = Editor::new()?;
    rl.set_helper(Some(helper));

    let mut history_path = String::new();
    if let Some(path) = home::home_dir() {
        history_path = format!("{}/.chonk_history", path.to_str().unwrap());
    }

    // Load REPL history
    if Path::new(&history_path).try_exists().is_ok() {
        rl.load_history(&history_path)?;
    }

    let mut running = true;
    while running {
        let readline = rl.readline(">> ");
        match readline {
            Ok(mut line) => {
                line = line.trim().to_string();
                if line.is_empty() {
                    continue;
                }

                // Save REPL history
                rl.add_history_entry(&line)?;
                rl.save_history(&history_path)?;

                // Commands
                match line.as_str() {
                    ".clear" => *interpreter = interpreter::Interpreter::default(),
                    ".exit" => running = false,
                    ".help" => todo!(),
                    code => run(interpreter, code),
                }
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

    Ok(())
}

/// Runs Chonk code.
fn run(interpreter: &mut interpreter::Interpreter, input: &str) {
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
