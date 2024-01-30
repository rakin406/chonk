#![allow(clippy::module_inception)]

use std::fmt;

use super::lexer::Lexer;
use crate::internal::ast::{Expr, Program, Stmt};
use crate::internal::error_reporter::{ErrorReporter, ErrorType};
use crate::internal::token::{token_type, Literal, Token, TokenType};

/// All possible error types in `Parser`.
pub enum ParseError {
    ExpectedExpression(Token),
    TokenMismatch {
        expected: TokenType,
        found: Token,
        message: String,
    },
}

#[derive(Default)]
pub struct Parser {
    tokens: Vec<Token>,
    current: usize,
}

impl fmt::Debug for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::ExpectedExpression(token) => {
                write!(
                    f,
                    "[line {}] {:#?}: Expected expression, but found token {:#?}",
                    token.line,
                    ErrorType::SyntaxError,
                    token.ty
                )
            }
            ParseError::TokenMismatch {
                expected,
                found,
                message,
            } => {
                write!(
                    f,
                    "[line {}] {:#?}: Expected token {:#?} but found {:#?}: {}",
                    found.line,
                    ErrorType::SyntaxError,
                    expected,
                    found.ty,
                    message
                )
            }
        }
    }
}

impl Parser {
    /// Creates a new `Parser`.
    pub fn new(input: String) -> Self {
        let mut lexer = Lexer::new(input);
        let tokens = lexer.scan_tokens();

        Self {
            tokens,
            ..Default::default()
        }
    }

    /// Parses statements and returns program.
    pub fn parse(&mut self) -> Result<Program, ParseError> {
        let mut program = Program::default();

        while !self.is_at_end() {
            program.add(self.statement()?);
        }

        Ok(program)
    }

    /// Discards tokens until it finds a statement boundary.
    #[allow(dead_code)]
    fn synchronize(&mut self) {
        self.advance();

        while !self.is_at_end() {
            if self.previous().ty == TokenType::Newline {
                break;
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

    /// Parses statements.
    fn statement(&mut self) -> Result<Stmt, ParseError> {
        if self.match_type(TokenType::Func) {
            return self.function_statement();
        }
        if self.match_type(TokenType::While) {
            return self.while_statement();
        }
        if self.match_type(TokenType::If) {
            return self.if_statement();
        }
        if self.match_type(TokenType::Echo) {
            return self.echo_statement();
        }

        self.expression_statement()
    }

    /// Parses function definition statement.
    fn function_statement(&mut self) -> Result<Stmt, ParseError> {
        let name: Token = self.consume(TokenType::Ident, "Expected function name")?;
        self.consume(TokenType::LParen, "Expected '(' after function name")?;
        let mut params: Vec<Token> = Vec::new();

        if !self.has_type(TokenType::RParen) {
            loop {
                if params.len() >= 255 {
                    self.token_error(self.peek().clone(), "Can't have more than 255 parameters");
                }

                params.push(self.consume(TokenType::Ident, "Expected parameter name")?);
                if !self.match_type(TokenType::Comma) {
                    break;
                }
            }
        }

        self.consume(TokenType::RParen, "Expected ')' after parameters")?;
        let _ = self.match_type(TokenType::Newline); // optional newline
        let body: Vec<Stmt> = self.block()?;

        Ok(Stmt::Function { name, params, body })
    }

    /// Parses while statement.
    fn while_statement(&mut self) -> Result<Stmt, ParseError> {
        let test = self.expression()?;
        let _ = self.match_type(TokenType::Newline); // optional newline
        let body: Vec<Stmt> = self.block()?;

        Ok(Stmt::While { test, body })
    }

    /// Parses if statement.
    fn if_statement(&mut self) -> Result<Stmt, ParseError> {
        let test = self.expression()?;
        // Optional newline after condition
        let _ = self.match_type(TokenType::Newline);

        let body: Vec<Stmt> = self.block()?;
        let or_else = if self.match_type(TokenType::Else) {
            let _ = self.match_type(TokenType::Newline);
            Some(self.block()?)
        } else {
            None
        };

        Ok(Stmt::If {
            test,
            body,
            or_else,
        })
    }

    /// Parses expression statement.
    fn expression_statement(&mut self) -> Result<Stmt, ParseError> {
        let expr = self.expression()?;
        self.consume(TokenType::Newline, "Expected newline after expression")?;
        Ok(Stmt::Expr(expr))
    }

    /// Parses echo statement.
    fn echo_statement(&mut self) -> Result<Stmt, ParseError> {
        let value = self.expression()?;
        self.consume(TokenType::Newline, "Expected newline after value")?;
        Ok(Stmt::Echo(value))
    }

    /// Parses a block of statements.
    fn block(&mut self) -> Result<Vec<Stmt>, ParseError> {
        self.consume(TokenType::LBrace, "Expected '{' before block")?;
        // Enter block after newline
        self.consume(TokenType::Newline, "Expected newline after '{'")?;
        let mut statements: Vec<Stmt> = Vec::new();

        while !self.has_type(TokenType::RBrace) && !self.is_at_end() {
            statements.push(self.statement()?);
        }

        self.consume(TokenType::RBrace, "Expected '}' after block")?;
        let _ = self.match_type(TokenType::Newline); // optional newline
        Ok(statements)
    }

    /// Expands to the `assignment` rule.
    fn expression(&mut self) -> Result<Expr, ParseError> {
        self.assignment()
    }

    /// Parses assignment expression.
    fn assignment(&mut self) -> Result<Expr, ParseError> {
        let expr = self.or()?;

        if self.match_type(TokenType::Equal) {
            let equals: Token = self.previous().clone();
            let value: Expr = self.assignment()?;

            if let Expr::Variable(name) = expr {
                return Ok(Expr::Assign(name, Box::new(value)));
            }

            self.token_error(equals, "Invalid assignment target");
        }

        Ok(expr)
    }

    /// Parses logical OR expression.
    fn or(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.and()?;

        while self.match_type(TokenType::DoubleVBar) {
            let operator: Token = self.previous().clone();
            let right: Expr = self.and()?;
            expr = Expr::Logical(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    /// Parses logical AND expression.
    fn and(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.equality()?;

        while self.match_type(TokenType::DoubleAmper) {
            let operator: Token = self.previous().clone();
            let right: Expr = self.equality()?;
            expr = Expr::Logical(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    /// Parses equality expression.
    fn equality(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.comparison()?;

        while self.match_types(&[TokenType::BangEqual, TokenType::EqEqual]) {
            let operator: Token = self.previous().clone();
            let right: Expr = self.comparison()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    /// Matches an equality operator.
    fn comparison(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.term()?;

        while self.match_types(&[
            TokenType::Greater,
            TokenType::GreaterEqual,
            TokenType::Less,
            TokenType::LessEqual,
        ]) {
            let operator: Token = self.previous().clone();
            let right: Expr = self.term()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    // TODO: Add missing documentation.
    fn term(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.factor()?;

        while self.match_types(&[TokenType::Minus, TokenType::Plus]) {
            let operator: Token = self.previous().clone();
            let right: Expr = self.factor()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    // TODO: Add missing documentation.
    fn factor(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.unary()?;

        while self.match_types(&[TokenType::Percent, TokenType::Slash, TokenType::Star]) {
            let operator: Token = self.previous().clone();
            let right: Expr = self.unary()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    /// Parses unary expression.
    fn unary(&mut self) -> Result<Expr, ParseError> {
        if self.match_types(&[TokenType::Bang, TokenType::Minus]) {
            let operator: Token = self.previous().clone();
            // TODO: Avoid recursion.
            let right: Expr = self.unary()?;
            return Ok(Expr::Unary(operator, Box::new(right)));
        }

        self.call()
    }

    /// Parses function call expression.
    fn call(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.primary()?;

        loop {
            if self.match_type(TokenType::LParen) {
                expr = self.finish_call(expr)?;
            } else {
                break;
            }
        }

        Ok(expr)
    }

    /// Finishes function call expression.
    fn finish_call(&mut self, callee: Expr) -> Result<Expr, ParseError> {
        let mut arguments: Vec<Expr> = Vec::new();

        if !self.has_type(TokenType::RParen) {
            loop {
                if arguments.len() >= 255 {
                    self.token_error(self.peek().clone(), "Can't have more than 255 arguments");
                }

                arguments.push(self.expression()?);
                if !self.match_type(TokenType::Comma) {
                    break;
                }
            }
        }

        let paren: Token = self.consume(TokenType::RParen, "Expected ')' after arguments")?;

        Ok(Expr::Call(Box::new(callee), paren, arguments))
    }

    // TODO: Add missing documentation.
    fn primary(&mut self) -> Result<Expr, ParseError> {
        if self.match_type(TokenType::True) {
            return Ok(Expr::Constant(Literal::Bool(true)));
        }
        if self.match_type(TokenType::False) {
            return Ok(Expr::Constant(Literal::Bool(false)));
        }
        if self.match_type(TokenType::Null) {
            return Ok(Expr::Constant(Literal::Null));
        }
        if self.match_type(TokenType::Number) {
            match &self.previous().literal {
                Some(Literal::Number(num)) => {
                    return Ok(Expr::Constant(Literal::Number(*num)));
                }
                Some(_) => {}
                None => {}
            }
        }
        if self.match_type(TokenType::String) {
            match &self.previous().literal {
                Some(Literal::String(str)) => {
                    return Ok(Expr::Constant(Literal::String(str.into())));
                }
                Some(_) => {}
                None => {}
            }
        }
        if self.match_type(TokenType::LParen) {
            let expr = self.expression()?;
            self.consume(TokenType::RParen, "Expected ')' after expression")?;
            return Ok(Expr::Grouping(Box::new(expr)));
        }
        if self.match_type(TokenType::Ident) {
            return Ok(Expr::Variable(self.previous().clone()));
        }

        Err(ParseError::ExpectedExpression(self.peek().to_owned()))
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
    fn match_types(&mut self, types: &[TokenType]) -> bool {
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

    /// Checks to see if the current token is of the expected type and consumes it.
    fn consume(&mut self, ty: TokenType, message: &str) -> Result<Token, ParseError> {
        if self.has_type(ty) {
            return Ok(self.advance());
        }

        Err(ParseError::TokenMismatch {
            expected: ty,
            found: self.peek().to_owned(),
            message: message.to_string(),
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

impl ErrorReporter for Parser {
    const ERROR_TYPE: ErrorType = ErrorType::SyntaxError;
}