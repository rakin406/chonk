use crate::token_type::TokenType;

#[derive(Clone, Debug)]
pub enum Literal {
    Identifier(String),
    String(String),
    Number(f64),
}

#[derive(Clone, Debug)]
pub struct Token {
    pub token_type: TokenType,
    pub lexeme: String,
    pub literal: Option<Literal>,
    pub line: usize,
    pub column: i64,
}
