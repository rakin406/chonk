use std::collections::HashMap;

use super::token::{Literal, Token};

#[allow(dead_code)]
pub struct Environment {
    values: HashMap<String, Literal>,
}

#[allow(dead_code)]
impl Environment {
    /// Binds a new name to a value.
    pub fn define(&mut self, name: String, value: Literal) {
        self.values.insert(name, value);
    }

    /// Returns the literal value bound to the variable name.
    pub fn get(&self, name: Token) -> Literal {
        if let Some(value) = self.values.get(&name.lexeme) {
            return value.to_owned();
        }

        // NOTE: I should return a RuntimeError here instead of returning Null.
        // Because of laziness and keeping things simple, I am not focusing much
        // on errors so that the project is finished quickly. Once the language
        // is working, I will add more error support.

        Literal::Null
    }
}
