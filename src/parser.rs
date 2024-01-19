use crate::error_reporter::ErrorReporter;
use crate::expr::Expr;
use crate::token::{Literal, Token};
use crate::token_type::TokenType;

#[derive(Debug, Clone)]
pub struct ParseError;

#[derive(Default)]
struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

/// Parses tokens and returns expression.
pub fn parse(tokens: Vec<Token>) -> Result<Expr, ParseError> {
    let mut parser = Parser::new(tokens);

    match parser.parse() {
        Ok(value) => return Ok(value),
        Err(error) => return Err(error),
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

    /// Parses expressions.
    fn parse(&mut self) -> Result<Expr, ParseError> {
        Ok(self.expression()?)
    }

    /// Discards tokens until it finds a statement boundary.
    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            match self.previous().token_type {
                TokenType::Newline => break,
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

    /// Expands to the `equality` rule.
    fn expression(&mut self) -> Result<Expr, ParseError> {
        self.equality()
    }

    // TODO: Add missing documentation.
    fn equality(&mut self) -> Result<Expr, ParseError> {
        use TokenType::*;

        let mut expr = self.comparison()?;

        while self.match_types(Vec::from([BangEqual, EqEqual])) {
            let operator: Token = self.previous().clone();
            let right: Expr = self.comparison()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    /// Matches an equality operator.
    fn comparison(&mut self) -> Result<Expr, ParseError> {
        use TokenType::*;

        let mut expr = self.term()?;

        while self.match_types(Vec::from([Greater, GreaterEqual, Less, LessEqual])) {
            let operator: Token = self.previous().clone();
            let right: Expr = self.term()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    // TODO: Add missing documentation.
    fn term(&mut self) -> Result<Expr, ParseError> {
        use TokenType::*;

        let mut expr = self.factor()?;

        while self.match_types(Vec::from([Minus, Plus])) {
            let operator: Token = self.previous().clone();
            let right: Expr = self.factor()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    // TODO: Add missing documentation.
    fn factor(&mut self) -> Result<Expr, ParseError> {
        use TokenType::*;

        let mut expr = self.unary()?;

        while self.match_types(Vec::from([Percent, Slash, Star])) {
            let operator: Token = self.previous().clone();
            let right: Expr = self.unary()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    // TODO: Add missing documentation.
    fn unary(&mut self) -> Result<Expr, ParseError> {
        use TokenType::*;

        if self.match_types(Vec::from([Bang, Minus])) {
            let operator: Token = self.previous().clone();
            // TODO: Avoid recursion.
            let right: Expr = self.unary()?;
            return Ok(Expr::Unary(operator, Box::new(right)));
        }

        self.primary()
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
            let expr = self.expression()?;
            let _ = self.consume(TokenType::RightParen, "Expected \')\' after expression");
            return Ok(Expr::Grouping(Box::new(expr)));
        }

        Err(self.parsing_error(self.peek().clone(), "Expected expression"))
    }

    /// Reports a parsing error.
    fn parsing_error(&self, token: Token, message: &str) -> ParseError {
        self.token_error(token, message);
        ParseError
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
