use std::fmt;

mod lexer;

use crate::internal::ast::{Expr, Program, Stmt};
use crate::internal::error_reporter::{ErrorReporter, ErrorType};
use crate::internal::token::{token_type, Literal, Token, TokenType};
use lexer::Lexer;

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
    pub fn new(input: &str) -> Self {
        let mut lexer = Lexer::new(input);
        let tokens = lexer.scan_tokens().to_vec();

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
            if self.previous().ty == TokenType::Semicolon {
                break;
            }

            match self.peek().ty {
                // TODO: Add function and variable here.
                TokenType::While | TokenType::If | TokenType::Echo | TokenType::Return => break,
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
        if self.match_type(TokenType::Return) {
            return self.return_statement();
        }
        if self.match_type(TokenType::Delete) {
            return self.delete_statement();
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
                    self.token_error(self.peek(), "Can't have more than 255 parameters");
                }

                params.push(self.consume(TokenType::Ident, "Expected parameter name")?);

                if !self.match_type(TokenType::Comma) {
                    break;
                }
            }
        }

        self.consume(TokenType::RParen, "Expected ')' after parameters")?;
        let body: Vec<Stmt> = self.block()?;

        Ok(Stmt::Function { name, params, body })
    }

    /// Parses return statement.
    fn return_statement(&mut self) -> Result<Stmt, ParseError> {
        let value = if !self.has_type(TokenType::Semicolon) {
            Some(self.expression()?)
        } else {
            None
        };

        self.consume(TokenType::Semicolon, "Expected ';' after return value")?;
        Ok(Stmt::Return(value))
    }

    /// Parses delete statement.
    fn delete_statement(&mut self) -> Result<Stmt, ParseError> {
        let mut targets: Vec<Token> = Vec::new();
        if !self.has_type(TokenType::Semicolon) {
            loop {
                targets.push(self.consume(TokenType::Ident, "Expected variable name")?);

                if !self.match_type(TokenType::Comma) {
                    break;
                }
            }
        }

        self.consume(TokenType::Semicolon, "Expected ';' after del statement")?;
        Ok(Stmt::Delete(targets))
    }

    /// Parses while statement.
    fn while_statement(&mut self) -> Result<Stmt, ParseError> {
        let test = self.expression()?;
        let body: Vec<Stmt> = self.block()?;
        Ok(Stmt::While { test, body })
    }

    /// Parses if statement.
    fn if_statement(&mut self) -> Result<Stmt, ParseError> {
        let test = self.expression()?;

        let body: Vec<Stmt> = self.block()?;
        let or_else = if self.match_type(TokenType::Else) {
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
        self.consume(TokenType::Semicolon, "Expected ';' after expression")?;
        Ok(Stmt::Expr(expr))
    }

    /// Parses echo statement.
    fn echo_statement(&mut self) -> Result<Stmt, ParseError> {
        let value = self.expression()?;
        self.consume(TokenType::Semicolon, "Expected ';' after value")?;
        Ok(Stmt::Echo(value))
    }

    /// Parses a block of statements.
    fn block(&mut self) -> Result<Vec<Stmt>, ParseError> {
        self.consume(TokenType::LBrace, "Expected '{' before block")?;
        let mut statements: Vec<Stmt> = Vec::new();

        while !self.has_type(TokenType::RBrace) && !self.is_at_end() {
            statements.push(self.statement()?);
        }

        self.consume(TokenType::RBrace, "Expected '}' after block")?;
        Ok(statements)
    }

    /// Expands to the `assignment` rule.
    fn expression(&mut self) -> Result<Expr, ParseError> {
        self.assignment()
    }

    /// Parses assignment expression.
    fn assignment(&mut self) -> Result<Expr, ParseError> {
        let expr = self.aug_assignment()?;

        if self.match_type(TokenType::Equal) {
            let equals: Token = self.previous().clone();
            let value: Expr = self.assignment()?;

            if let Expr::Variable(name) = expr {
                return Ok(Expr::Assign(name, Box::new(value)));
            }

            self.token_error(&equals, "Invalid assignment target");
        }

        Ok(expr)
    }

    /// Parses augmented assignment expression.
    fn aug_assignment(&mut self) -> Result<Expr, ParseError> {
        let expr = self.or()?;

        if self.match_types(&[
            TokenType::MinusEqual,
            TokenType::PlusEqual,
            TokenType::PercentEqual,
            TokenType::SlashEqual,
            TokenType::StarEqual,
        ]) {
            let operator: Token = self.previous().clone();
            let value: Expr = self.or()?;

            if let Expr::Variable(name) = expr {
                return Ok(Expr::AugAssign {
                    name,
                    operator,
                    value: Box::new(value),
                });
            }

            self.token_error(&operator, "Invalid assignment target");
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

    /// Parses binary term expression.
    fn term(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.factor()?;

        while self.match_types(&[TokenType::Minus, TokenType::Plus]) {
            let operator: Token = self.previous().clone();
            let right: Expr = self.factor()?;
            expr = Expr::Binary(Box::new(expr), operator, Box::new(right));
        }

        Ok(expr)
    }

    /// Parses binary factor expression.
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
        if self.match_types(&[TokenType::Bang, TokenType::Minus, TokenType::Plus]) {
            let operator: Token = self.previous().clone();
            let right: Expr = self.unary()?;
            return Ok(Expr::Unary(operator, Box::new(right)));
        }

        self.prefix()
    }

    /// Parses prefix expression.
    fn prefix(&mut self) -> Result<Expr, ParseError> {
        if self.match_types(&[TokenType::DoubleMinus, TokenType::DoublePlus]) {
            let operator: Token = self.previous().clone();
            let expr = self.call()?;

            if let Expr::Variable(name) = expr {
                return Ok(Expr::Prefix { operator, name });
            }

            self.token_error(&operator, "Invalid target in prefix operation");
        }

        self.call()
    }

    /// Parses function call expression.
    fn call(&mut self) -> Result<Expr, ParseError> {
        let mut expr = self.suffix()?;

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
                    self.token_error(self.peek(), "Can't have more than 255 arguments");
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

    // NOTE: This does not create a suffix AST node. It just desugars the suffix
    // expression into a prefix expression. The reason is that only increment/
    // decrement suffixes are available (also function calls but that is already
    // handled above), so it would be unnecessary to create a new suffix node.
    /// Parses suffix expression.
    fn suffix(&mut self) -> Result<Expr, ParseError> {
        let expr = self.primary()?;

        if self.match_types(&[TokenType::DoubleMinus, TokenType::DoublePlus]) {
            let operator: Token = self.previous().clone();
            if let Expr::Variable(name) = expr {
                return Ok(Expr::Prefix { operator, name });
            }

            self.token_error(&operator, "Invalid target in suffix operation");
        }

        Ok(expr)
    }

    /// Parses primary expression.
    fn primary(&mut self) -> Result<Expr, ParseError> {
        if self.match_type(TokenType::True) {
            return Ok(Expr::Constant(Literal::True));
        }
        if self.match_type(TokenType::False) {
            return Ok(Expr::Constant(Literal::False));
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
    fn advance(&mut self) -> &Token {
        if !self.is_at_end() {
            self.current += 1;
        }
        self.previous()
    }

    /// Checks to see if the current token is of the expected type and consumes it.
    fn consume(&mut self, ty: TokenType, message: &str) -> Result<Token, ParseError> {
        if self.has_type(ty) {
            return Ok(self.advance().clone());
        }

        Err(ParseError::TokenMismatch {
            expected: ty,
            found: self.peek().clone(),
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
