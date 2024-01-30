#![allow(clippy::module_inception)]

use super::literal::Literal;
use super::token_type::TokenType;

#[derive(Debug, Clone)]
pub struct Token {
    pub ty: TokenType,
    pub lexeme: String,
    pub literal: Option<Literal>,
    pub line: usize,
}

impl Token {
    /// Creates a new `Token`.
    pub fn new(ty: TokenType, lexeme: String, literal: Option<Literal>, line: usize) -> Self {
        Self {
            ty,
            lexeme,
            literal,
            line,
        }
    }
}
