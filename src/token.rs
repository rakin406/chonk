use std::any::Any;

use crate::token_type::TokenType;

pub struct Token {
    pub token_type: TokenType,
    pub lexeme: &'static str,
    pub literal: Option<Box<dyn Any>>,
    pub line: usize,
}
