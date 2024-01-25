use std::fmt;

use super::ast::Expr;
use super::token::{Literal, Token};
use super::token_type::{self, TokenType};

/// All possible error types in `Parser`.
pub enum ParseError {
    ExpectedExpression(Token),
    TokenMismatch { expected: TokenType, found: Token },
}

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

impl fmt::Debug for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::ExpectedExpression(token) => {
                write!(
                    f,
                    "[line {}] ParseError: Expected expression, but found token {:#?}",
                    token.line, token.ty
                )
            }
            ParseError::TokenMismatch { expected, found } => {
                write!(
                    f,
                    "[line {}] ParseError: Expected token {:#?} but found {:#?}",
                    found.line, expected, found.ty
                )
            }
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

    /// Parses expressions.
    fn parse(&mut self) -> Result<Expr, ParseError> {
        Ok(self.expression()?)
    }

    /// Discards tokens until it finds a statement boundary.
    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            match self.previous().ty {
                TokenType::Newline => break,
                _ => {}
            }

            match self.peek().ty {
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
        let mut expr = self.comparison()?;

        while self.match_types(Vec::from([TokenType::BangEqual, TokenType::EqEqual])) {
            let operator: Token = self.previous().clone();
            let right: Expr = self.comparison()?;
            expr = Expr::BinOp(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    /// Matches an equality operator.
    fn comparison(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.term()?;

        while self.match_types(Vec::from([
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ])) {
            let operator: Token = self.previous().clone();
            let right: Expr = self.term()?;
            expr = Expr::BinOp(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    // TODO: Add missing documentation.
    fn term(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.factor()?;

        while self.match_types(Vec::from([TokenType::Minus, TokenType::Plus])) {
            let operator: Token = self.previous().clone();
            let right: Expr = self.factor()?;
            expr = Expr::BinOp(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    // TODO: Add missing documentation.
    fn factor(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.unary()?;

        while self.match_types(Vec::from([
            TokenType::Percent,
            TokenType::Slash,
            TokenType::Star,
        ])) {
            let operator: Token = self.previous().clone();
            let right: Expr = self.unary()?;
            expr = Expr::BinOp(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    // TODO: Add missing documentation.
    fn unary(&mut self) -> Result<Expr, ParseError> {
        if self.match_types(Vec::from([TokenType::Bang, TokenType::Minus])) {
            let operator: Token = self.previous().clone();
            // TODO: Avoid recursion.
            let right: Expr = self.unary()?;
            return Ok(Expr::UnaryOp(operator, Box::new(right)));
        }

        self.primary()
    }

    // TODO: Add missing documentation.
    fn primary(&mut self) -> Result<Expr, ParseError> {
        if self.match_type(TokenType::True) {
            return Ok(Expr::Constant(Literal::True(true)));
        } else if self.match_type(TokenType::False) {
            return Ok(Expr::Constant(Literal::False(false)));
        } else if self.match_type(TokenType::Null) {
            return Ok(Expr::Constant(Literal::Null));
        } else if self.match_type(TokenType::Number) {
            match &self.previous().literal {
                Some(Literal::Number(num)) => {
                    return Ok(Expr::Constant(Literal::Number(*num)));
                }
                Some(_) => {}
                None => {}
            }
        } else if self.match_type(TokenType::String) {
            match &self.previous().literal {
                Some(Literal::String(str)) => {
                    return Ok(Expr::Constant(Literal::String(str.to_string())));
                }
                Some(_) => {}
                None => {}
            }
        } else if self.match_type(TokenType::LParen) {
            let expr = self.expression()?;
            self.consume(TokenType::RParen)?;
            return Ok(Expr::Grouping(Box::new(expr)));
        }

        Err(ParseError::ExpectedExpression(self.peek().clone()))
    }

    /// Returns `true` if the current token has the given type. If so, it
    /// consumes the token.
    fn match_type(&mut self, ty: TokenType) -> bool {
        if self.has_type(ty) {
            self.advance();
            return true;
        }

        false
    }

    /// Returns `true` if the current token has any of the given types. If so,
    /// it consumes the token.
    fn match_types(&mut self, types: Vec<TokenType>) -> bool {
        for ty in types.iter() {
            if self.match_type(*ty) {
                return true;
            }
        }

        false
    }

    /// Returns `true` if the current token is of the given type.
    fn has_type(&self, ty: TokenType) -> bool {
        if self.is_at_end() {
            return false;
        }
        self.peek().ty == ty
    }

    /// Returns `true` if there is no more tokens to parse.
    fn is_at_end(&self) -> bool {
        token_type::is_eof(self.peek().ty)
    }

    /// Consumes the current token and returns it.
    fn advance(&mut self) -> Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous().clone()
    }

    /// Checks to see if the next token is of the expected type and consumes it.
    fn consume(&mut self, ty: TokenType) -> Result<Token, ParseError> {
        if self.has_type(ty) {
            return Ok(self.advance());
        }

        // FIX: TokenMismatch error not printing.
        Err(ParseError::TokenMismatch {
            expected: ty,
            found: self.peek().clone(),
        })
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
