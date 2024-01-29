use std::collections::HashMap;

use super::runtime_error::RuntimeError;
use super::token::{Literal, Token};

#[derive(Default, Clone)]
pub struct Environment {
    store: HashMap<String, Literal>,
    outer: Option<Box<Environment>>,
}

impl Environment {
    /// Creates a new outer scope.
    pub fn new_outer(outer: Box<Environment>) -> Self {
        Self {
            outer: Some(outer),
            ..Default::default()
        }
    }

    /// Returns the literal value bound to the name.
    pub fn get(&self, name: &Token) -> Result<Literal, RuntimeError> {
        if let Some(value) = self.store.get(&name.lexeme) {
            return Ok(value.to_owned());
        }

        if let Some(outer_env) = &self.outer {
            return outer_env.get(name);
        }

        Err(RuntimeError::new(
            name.to_owned(),
            &format!("Undefined variable \"{}\"", name.lexeme),
        ))
    }

    /// Binds a new name to a value. If the name exists, it assigns a new value
    /// to it.
    pub fn set(&mut self, name: String, value: Literal) {
        self.store.insert(name, value);
    }
}
