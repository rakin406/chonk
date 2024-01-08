use std::collections::HashMap;

use crate::token_type::TokenType;

/// All the reserved words and it's token type.
pub const KEYWORDS: HashMap<&str, TokenType> = HashMap::from([
    ("null", TokenType::Null),
    ("true", TokenType::True),
    ("false", TokenType::False),
    ("if", TokenType::If),
    ("elif", TokenType::Elif),
    ("else", TokenType::Else),
    ("in", TokenType::In),
    ("!in", TokenType::NotIn),
    ("while", TokenType::While),
    ("for", TokenType::For),
    ("break", TokenType::Break),
    ("continue", TokenType::Continue),
    ("return", TokenType::Return),
    ("echo", TokenType::Echo),
]);
