use crate::error_reporter::ErrorReporter;
use crate::expr::Expr;
use crate::token::Token;
use crate::token_type::TokenType;

#[derive(Debug, Clone)]
struct ParseError;

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

// TODO: I think I need to write an error() method.
impl Parser {
    /// Creates a new `Parser`.
    fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            ..Default::default()
        }
    }

    /// Discards tokens until it finds a statement boundary.
    fn synchronize(&self) {
        self.advance();

        while !self.is_at_end() {
            match self.previous().token_type {
                TokenType::Newline | TokenType::Backslash => break,
                _ => {}
            }

            match self.peek().token_type {
                // TODO: Add function and variable here.
                TokenType::While
                | TokenType::For
                | TokenType::If
                | TokenType::Echo
                | TokenType::Return => break,
                _ => {}
            }

            self.advance();
        }
    }

    /// Reports a parsing error.
    fn parsing_error(&self, token: Token, message: &str) -> ParseError {
        self.token_error(token, message);
        ParseError
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

        self.parse_binary_ops(Vec::from([Greater, GreaterEqual, Less, LessEqual]), &|| {
            self.term()
        })
    }

    // TODO: Add missing documentation.
    fn term(&self) -> Expr {
        use TokenType::*;
        self.parse_binary_ops(Vec::from([Sub, Add]), &|| self.factor())
    }

    // TODO: Add missing documentation.
    fn factor(&self) -> Expr {
        use TokenType::*;
        self.parse_binary_ops(Vec::from([Mod, Div, Mult]), &|| self.unary())
    }

    // TODO: Add missing documentation.
    fn unary(&self) -> Expr {
        use TokenType::*;

        if self.match_types(Vec::from([Not, Sub])) {
            let operator: Token = self.previous();
            // TODO: Avoid recursion.
            let right: Expr = self.unary();
            return Expr::UnaryOp {
                operator,
                right: Box::new(right),
            };
        }

        self.primary().unwrap()
    }

    // TODO: Add missing documentation.
    fn primary(&self) -> Result<Expr, ParseError> {
        if self.match_type(TokenType::True) {
            return Ok(Expr::Literal(Some(Box::new(true))));
        }
        if self.match_type(TokenType::False) {
            return Ok(Expr::Literal(Some(Box::new(false))));
        }
        if self.match_type(TokenType::Null) {
            return Ok(Expr::Literal(None));
        }

        if self.match_types(Vec::from([TokenType::Number, TokenType::String])) {
            return Ok(Expr::Literal(Some(Box::new(self.previous().literal))));
        }

        if self.match_type(TokenType::LeftParen) {
            let expr = self.expression();
            self.consume(TokenType::RightParen, "Expected \')\' after expression");
            return Ok(Expr::Grouping(Box::new(expr)));
        }

        Err(self.parsing_error(self.peek(), "Expected expression"))
    }

    /// Parses the binary operators from a list of token types and returns the
    /// expression.
    fn parse_binary_ops(&self, types: Vec<TokenType>, handle: &dyn Fn() -> Expr) -> Expr {
        let mut expr = handle();

        while self.match_types(types) {
            let operator: Token = self.previous();
            let right: Expr = handle();
            expr = Expr::BinaryOp {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        expr
    }

    /// Returns `true` if the current token has the given type. If so, it
    /// consumes the token.
    fn match_type(&self, token_type: TokenType) -> bool {
        if self.has_type(token_type) {
            self.advance();
            return true;
        }

        false
    }

    /// Returns `true` if the current token has any of the given types. If so,
    /// it consumes the token.
    fn match_types(&self, types: Vec<TokenType>) -> bool {
        for token_type in types.iter() {
            if self.match_type(*token_type) {
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

    /// Checks to see if the next token is of the expected type and consumes it.
    fn consume(&self, token_type: TokenType, message: &str) -> Result<Token, ParseError> {
        if self.has_type(token_type) {
            return Ok(self.advance());
        }

        Err(self.parsing_error(self.peek(), message))
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

impl ErrorReporter for Parser {}
