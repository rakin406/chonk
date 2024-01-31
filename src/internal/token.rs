mod literal;
pub mod token_type;

pub use literal::Literal;
pub use token_type::TokenType;

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