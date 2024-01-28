use std::collections::HashMap;

use super::error_reporter::{ErrorReporter, ErrorType};
use super::token::{Literal, Token};

#[derive(Default)]
pub struct Environment {
    store: HashMap<String, Literal>,
}

impl Environment {
    /// Returns the literal value bound to the name.
    pub fn get(&self, name: &Token) -> Literal {
        if let Some(value) = self.store.get(&name.lexeme) {
            return value.to_owned();
        }

        // NOTE: This looks ugly to me, maybe turn ErrorReporter into ErrorFormatter?
        // That way I could pass in the error string to panic!().
        self.token_error(
            name.to_owned(),
            &format!("Undefined variable \"{}\"", name.lexeme),
        );
        panic!();
    }

    /// Binds a new name to a value. If the name exists, it assigns a new value
    /// to it.
    pub fn set(&mut self, name: String, value: Literal) {
        self.store.insert(name, value);
    }
}

impl ErrorReporter for Environment {
    const ERROR_TYPE: ErrorType = ErrorType::RuntimeError;
}
