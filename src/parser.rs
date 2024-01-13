use crate::expr::Expr;
use crate::token::Token;
use crate::token_type::TokenType;

struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl Default for Parser {
    fn default() -> Self {
        Self {
            tokens: Vec::new(),
            current: 0,
        }
    }
}

impl Parser {
    /// Creates a new `Parser`.
    fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            ..Default::default()
        }
    }

    /// Expands to the `equality` rule.
    fn expression(&self) -> Expr {
        self.equality()
    }

    fn equality(&self) -> Expr {
    }

    fn comparison(&self) -> Expr {
    }

    fn term(&self) -> Expr {
    }

    fn factor(&self) -> Expr {
    }

    fn unary(&self) -> Expr {
    }

    fn primary(&self) -> Expr {
    }

    fn match_types(&self, types: TokenType) -> bool {
    }

    fn has_type(&self, token_type: TokenType) -> bool {
    }

    fn is_at_end(&self) -> bool {
    }

    fn advance(&self) -> Token {
    }

    fn peek(&self) -> Token {
    }

    fn previous(&self) -> Token {
    }
}
