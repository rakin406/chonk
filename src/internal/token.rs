use std::fmt;

use super::token_type::TokenType;

#[derive(Debug, Clone)]
pub enum Literal {
    Number(f64),
    String(String),
    Bool(bool),
    Null,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub ty: TokenType,
    pub lexeme: String,
    pub literal: Option<Literal>,
    pub line: usize,
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Literal::Number(value) => write!(f, "{value}"),
            Literal::String(value) => write!(f, "{value}"),
            Literal::Bool(value) => write!(f, "{value}"),
            Literal::Null => write!(f, "null"),
        }
    }
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
