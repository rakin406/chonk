use std::fmt;

use super::token::Token;

pub struct RuntimeError {
    token: Token,
    message: String,
}

impl fmt::Debug for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[line {}] RuntimeError: {}",
            self.token.line, self.message
        )
    }
}

impl RuntimeError {
    /// Creates a new `RuntimeError`.
    pub fn new(token: Token, message: &str) -> Self {
        Self {
            token,
            message: message.to_string(),
        }
    }
}
