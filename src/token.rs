use std::any::Any;

use crate::token_type::TokenType;

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<Box<dyn Any>>,
    pub line: usize,
}

impl Token {
    /// Creates a new `Token`.
    pub fn new(
        token_type: TokenType,
        lexeme: String,
        literal: Option<Box<dyn Any>>,
        line: usize,
    ) -> Self {
        Self {
            token_type,
            lexeme,
            literal,
            line,
        }
    }

    /// Returns the `Token` as a `String` format.
    pub fn to_string(&self) -> String {
        format!("{:#?} {} {:#?}", self.token_type, self.lexeme, self.literal)
    }
}
