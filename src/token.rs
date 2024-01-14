use std::any::Any;

use crate::token_type::TokenType;

#[derive(Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<Box<dyn Any>>,
    pub line: usize,
    pub column: i64,
}

impl Token {
    /// Creates a new `Token`.
    pub fn new(
        token_type: TokenType,
        lexeme: String,
        literal: Option<Box<dyn Any>>,
        line: usize,
        column: i64,
    ) -> Self {
        Self {
            token_type,
            lexeme,
            literal,
            line,
            column,
        }
    }
}
