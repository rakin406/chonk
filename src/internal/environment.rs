use std::collections::HashMap;

use super::error_reporter::{ErrorReporter, ErrorType};
use super::token::{Literal, Token};

#[derive(Default)]
pub struct Environment {
    store: HashMap<String, Literal>,
    outer: Option<Box<Environment>>,
}

impl Environment {
    /// Creates a new outer scope.
    pub fn new_outer(&mut self, outer: Box<Environment>) {
        self.outer = Some(outer);
    }

    /// Returns the literal value bound to the name.
    pub fn get(&self, name: &Token) -> Literal {
        if let Some(value) = self.store.get(&name.lexeme) {
            return value.to_owned();
        }

        if let Some(outer_env) = &self.outer {
            return outer_env.get(name);
        }

        self.token_error(
            name.to_owned(),
            &format!("Undefined variable \"{}\"", name.lexeme),
        );
        Literal::Null
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
