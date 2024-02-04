use std::path::Path;

use rustyline::error::ReadlineError;
use rustyline::highlight::MatchingBracketHighlighter;
use rustyline::validate::MatchingBracketValidator;
use rustyline::{Completer, Helper, Highlighter, Hinter, Validator};
use rustyline::{Editor, Result};

use crate::internal::interpreter::Interpreter;
use crate::runner;

#[derive(Completer, Helper, Highlighter, Hinter, Validator)]
struct InputValidator {
    #[rustyline(Validator)]
    brackets: MatchingBracketValidator,
    #[rustyline(Highlighter)]
    highlighter: MatchingBracketHighlighter,
}

/// Runs the interpreter interactively.
pub fn start() -> Result<()> {
    let mut interpreter = Interpreter::default();

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
                    ".clear" => interpreter = Interpreter::default(),
                    ".exit" => running = false,
                    ".help" => todo!(),
                    code => runner::run(code, &mut interpreter),
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
