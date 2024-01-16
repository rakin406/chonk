use crate::error_reporter::ErrorReporter;
use crate::expr::Expr;
use crate::token::{Literal, Token};
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

impl Parser {
    /// Creates a new `Parser`.
    fn new(tokens: Vec<Token>) -> Self {
        Self {
            tokens,
            ..Default::default()
        }
    }

    /// Discards tokens until it finds a statement boundary.
    fn synchronize(&mut self) {
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
    fn expression(&mut self) -> Expr {
        self.equality()
    }

    // TODO: Add missing documentation.
    fn equality(&mut self) -> Expr {
        use TokenType::*;

        let mut expr = self.comparison();

        while self.match_types(Vec::from([NotEqualTo, EqualTo])) {
            let operator: Token = self.previous().clone();
            let right: Expr = self.comparison();
            expr = Expr::BinaryOp {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        expr
    }

    /// Matches an equality operator.
    fn comparison(&mut self) -> Expr {
        use TokenType::*;

        let mut expr = self.term();

        while self.match_types(Vec::from([Greater, GreaterEqual, Less, LessEqual])) {
            let operator: Token = self.previous().clone();
            let right: Expr = self.term();
            expr = Expr::BinaryOp {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        expr
    }

    // TODO: Add missing documentation.
    fn term(&mut self) -> Expr {
        use TokenType::*;

        let mut expr = self.factor();

        while self.match_types(Vec::from([Sub, Add])) {
            let operator: Token = self.previous().clone();
            let right: Expr = self.factor();
            expr = Expr::BinaryOp {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        expr
    }

    // TODO: Add missing documentation.
    fn factor(&mut self) -> Expr {
        use TokenType::*;

        let mut expr = self.unary();

        while self.match_types(Vec::from([Mod, Div, Mult])) {
            let operator: Token = self.previous().clone();
            let right: Expr = self.unary();
            expr = Expr::BinaryOp {
                left: Box::new(expr),
                operator,
                right: Box::new(right),
            };
        }

        expr
    }

    // TODO: Add missing documentation.
    fn unary(&mut self) -> Expr {
        use TokenType::*;

        if self.match_types(Vec::from([Not, Sub])) {
            let operator: Token = self.previous().clone();
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
    fn primary(&mut self) -> Result<Expr, ParseError> {
        if self.match_type(TokenType::True) {
            return Ok(Expr::Literal(Literal::Boolean(true)));
        }
        if self.match_type(TokenType::False) {
            return Ok(Expr::Literal(Literal::Boolean(false)));
        }
        if self.match_type(TokenType::Null) {
            return Ok(Expr::Literal(Literal::Null));
        }

        if self.match_type(TokenType::Number) {
            match &self.previous().literal {
                Some(Literal::Number(num)) => {
                    return Ok(Expr::Literal(Literal::Number(*num)));
                }
                Some(literal) => panic!("Error while parsing number: found literal {:#?}", literal),
                None => panic!("Error while parsing number: no literal found"),
            }
        }
        if self.match_type(TokenType::String) {
            match &self.previous().literal {
                Some(Literal::String(str)) => {
                    return Ok(Expr::Literal(Literal::String(str.to_string())));
                }
                Some(literal) => panic!("Error while parsing string: found literal {:#?}", literal),
                None => panic!("Error while parsing string: no literal found"),
            }
        }

        if self.match_type(TokenType::LeftParen) {
            let expr = self.expression();
            self.consume(TokenType::RightParen, "Expected \')\' after expression");
            return Ok(Expr::Grouping(Box::new(expr)));
        }

        Err(self.parsing_error(self.peek().clone(), "Expected expression"))
    }

    /// Returns `true` if the current token has the given type. If so, it
    /// consumes the token.
    fn match_type(&mut self, token_type: TokenType) -> bool {
        if self.has_type(token_type) {
            self.advance();
            return true;
        }

        false
    }

    /// Returns `true` if the current token has any of the given types. If so,
    /// it consumes the token.
    fn match_types(&mut self, types: Vec<TokenType>) -> bool {
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
        self.previous().clone()
    }

    /// Checks to see if the next token is of the expected type and consumes it.
    fn consume(&mut self, token_type: TokenType, message: &str) -> Result<Token, ParseError> {
        if self.has_type(token_type) {
            return Ok(self.advance());
        }

        Err(self.parsing_error(self.peek().clone(), message))
    }

    /// Returns the current token which is yet to consume.
    fn peek(&self) -> &Token {
        &self.tokens[self.current]
    }

    /// Returns the last consumed token.
    fn previous(&self) -> &Token {
        &self.tokens[self.current - 1]
    }
}

impl ErrorReporter for Parser {}
