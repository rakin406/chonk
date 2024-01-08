use std::any::Any;

use crate::token_type::TokenType;

pub struct Token {
    pub token_type: TokenType,
    pub data_type: Option<Box<dyn Any>>,
    pub lexeme: &'static str,
    pub line: usize,
}
