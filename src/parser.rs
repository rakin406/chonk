use crate::expr::{Binary, Expr};
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

    // TODO: Add missing documentation.
    fn equality(&self) -> Expr {
        use TokenType::*;
        self.parse_binary_ops(Vec::from([NotEqualTo, EqualTo]), &|| self.comparison())
    }

    /// Matches an equality operator.
    fn comparison(&self) -> Expr {
        use TokenType::*;

        self.parse_binary_ops(
            Vec::from([
                GreaterThan,
                GreaterThanOrEqualTo,
                LessThan,
                LessThanOrEqualTo,
            ]),
            &|| self.term(),
        )
    }

    fn term(&self) -> Expr {}

    fn factor(&self) -> Expr {}

    fn unary(&self) -> Expr {}

    fn primary(&self) -> Expr {}

    /// Parses the binary operators from a list of token types and returns the
    /// expression.
    fn parse_binary_ops(&self, types: Vec<TokenType>, handle: &dyn Fn() -> Expr) -> Expr {
        let mut expr = handle();

        while self.match_types(types) {
            let operator: Token = self.previous();
            let right: Expr = handle();
            expr = Expr {
                binary: Box::new(Binary::new(expr, operator, right)),
            };
        }

        expr
    }

    /// Returns `true` if the current token has any of the given types. If so,
    /// it consumes the token.
    fn match_types(&self, types: Vec<TokenType>) -> bool {
        for token_type in types.iter() {
            if self.has_type(*token_type) {
                self.advance();
                return true;
            }
        }

        false
    }

    /// Returns `true` if the current token is of the given type.
    fn has_type(&self, token_type: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().token_type == token_type
    }

    /// Returns `true` if there is no more tokens to parse.
    fn is_at_end(&self) -> bool {
        self.peek().token_type == TokenType::Eof
    }

    /// Consumes the current token and returns it.
    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    /// Returns the current token which is yet to consume.
    fn peek(&self) -> Token {
        self.tokens[self.current]
    }

    /// Returns the last consumed token.
    fn previous(&self) -> Token {
        self.tokens[self.current - 1]
    }
}
