use std::collections::HashMap;

use super::error_reporter::{ErrorReporter, ErrorType};
use super::token::{Literal, Token};

#[derive(Default)]
pub struct Environment {
    store: HashMap<String, Literal>,
}

impl Environment {
    /// Binds a new name to a value.
    pub fn define(&mut self, name: String, value: Literal) {
        self.store.insert(name, value);
    }

    /// Assigns a new value to an existing name.
    pub fn assign(&mut self, name: Token, value: Literal) {
        if self.store.contains_key(&name.lexeme) {
            self.store.insert(name.lexeme, value);
        } else {
            self.token_error(name, &format!("Undefined variable \"{}\"", name.lexeme));
            panic!();
        }
    }

    /// Returns the literal value bound to the name.
    pub fn get(&self, name: Token) -> Literal {
        if let Some(value) = self.store.get(&name.lexeme) {
            return value.to_owned();
        }

        // NOTE: This looks ugly to me, maybe turn ErrorReporter into ErrorFormatter?
        // That way I could pass in the error string to panic!().
        self.token_error(name, &format!("Undefined variable \"{}\"", name.lexeme));
        panic!();
    }
}

impl ErrorReporter for Environment {
    const ERROR_TYPE: ErrorType = ErrorType::RuntimeError;
}
