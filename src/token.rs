use crate::token_type::TokenType;

pub struct Token {
    token_type: TokenType,
    lexeme: String,
    line: usize,
}
