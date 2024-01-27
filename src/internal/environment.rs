use std::collections::HashMap;

use super::token::Literal;

#[derive(Default)]
pub struct Environment {
    values: HashMap<String, Literal>,
}

impl Environment {
    /// Binds a new name to a value.
    pub fn define(&mut self, name: String, value: Literal) {
        self.values.insert(name, value);
    }

    /// Assigns a new value to an existing name.
    pub fn assign(&mut self, name: String, value: Literal) {
        if self.values.contains_key(&name) {
            self.values.insert(name, value);
        }
    }

    /// Returns the literal value bound to the name.
    pub fn get(&self, name: String) -> Option<Literal> {
        if let Some(value) = self.values.get(&name) {
            return Some(value.to_owned());
        }

        // NOTE: I should return a RuntimeError here instead of returning None.
        // Because of laziness and keeping things simple, I am not focusing much
        // on errors so that the project is finished quickly. Once the language
        // is working, I will add more error support.

        None
    }
}
