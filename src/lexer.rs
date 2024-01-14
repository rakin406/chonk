use std::any::Any;
use std::collections::HashMap;

use crate::token::Token;
use crate::token_type::TokenType;

#[derive(Debug, Clone)]
pub struct LexError {
    pub what: String,
    pub line: usize,
    pub column: i64,
}

struct Lexer {
    source: String,
    tokens: Vec<Token>,
    error: Option<LexError>,
    start: usize,
    current: usize,
    line: usize,
    column: i64,
    keywords: HashMap<String, TokenType>,
}

/// Scans tokens from source and returns it, otherwise returns error.
pub fn scan_tokens(source: String) -> Result<Vec<Token>, LexError> {
    let mut lexer = Lexer::default();

    lexer.scan_tokens(source);

    match lexer.error {
        Some(error) => Err(error),
        None => Ok(lexer.tokens),
    }
}

impl Default for Lexer {
    fn default() -> Self {
        Self {
            source: String::new(),
            tokens: Vec::new(),
            error: None,
            start: 0,
            current: 0,
            line: 1,
            column: 0,
            keywords: HashMap::from([
                ("null", TokenType::Null),
                ("true", TokenType::True),
                ("false", TokenType::False),
                ("if", TokenType::If),
                ("elif", TokenType::Elif),
                ("else", TokenType::Else),
                ("case", TokenType::Case),
                ("default", TokenType::Default),
                // TODO: Use these later.
                // ("in", TokenType::In),
                // ("!in", TokenType::NotIn),
                ("while", TokenType::While),
                ("for", TokenType::For),
                ("break", TokenType::Break),
                ("continue", TokenType::Continue),
                ("return", TokenType::Return),
                ("echo", TokenType::Echo),
            ])
            .into_iter()
            .map(|(key, value)| (String::from(key), value))
            .collect(),
        }
    }
}

impl Lexer {
    /// Adds tokens from source until character ends.
    fn scan_tokens(&mut self, source: String) {
        self.source = source;

        while !self.done() {
            // We are at the beginning of the next lexeme
            self.start = self.current;
            self.scan_token();
        }

        match self.error {
            Some(_) => {}
            None => self.tokens.push(Token::new(
                TokenType::Eof,
                String::new(),
                None,
                self.line,
                self.column,
            )),
        }
    }

    /// Adds token type for the next character.
    fn scan_token(&mut self) {
        use TokenType::*;

        let c: char = self.advance();
        match c {
            '(' => self.add_token(LeftParen),
            ')' => self.add_token(RightParen),
            '[' => self.add_token(LeftBracket),
            ']' => self.add_token(RightBracket),
            '{' => self.add_token(LeftBrace),
            '}' => self.add_token(RightBrace),
            ',' => self.add_token(Comma),
            '.' => self.add_token(Dot),

            '+' => {
                if self.match_char('+') {
                    self.add_token(PlusPlus);
                } else if self.match_char('=') {
                    self.add_token(PlusEqual);
                } else {
                    self.add_token(Plus);
                }
            }
            '-' => {
                if self.match_char('-') {
                    self.add_token(MinusMinus);
                } else if self.match_char('=') {
                    self.add_token(MinusEqual);
                } else {
                    self.add_token(Minus);
                }
            }
            '*' => {
                if self.match_char('=') {
                    self.add_token(AsteriskEqual);
                } else {
                    self.add_token(Asterisk);
                }
            }
            '/' => {
                if self.match_char('=') {
                    self.add_token(SlashEqual);
                } else {
                    self.add_token(Slash);
                }
            }
            '%' => {
                if self.match_char('=') {
                    self.add_token(PercentEqual);
                } else {
                    self.add_token(Percent);
                }
            }

            '=' => {
                if self.match_char('=') {
                    self.add_token(EqualTo);
                } else {
                    self.add_token(Equal);
                }
            }
            '!' => {
                if self.match_char('=') {
                    self.add_token(NotEqualTo);
                } else {
                    self.add_token(Not);
                }
            }

            '>' => {
                if self.match_char('>') {
                    self.add_token(RightShift);
                } else if self.match_char('=') {
                    self.add_token(GreaterThanOrEqualTo);
                } else {
                    self.add_token(GreaterThan);
                }
            }
            '<' => {
                if self.match_char('<') {
                    self.add_token(LeftShift);
                } else if self.match_char('=') {
                    self.add_token(LessThanOrEqualTo);
                } else {
                    self.add_token(LessThan);
                }
            }

            '&' => {
                if self.match_char('&') {
                    self.add_token(And);
                } else {
                    self.add_token(Ampersand);
                }
            }
            '|' => {
                if self.match_char('|') {
                    self.add_token(Or);
                } else {
                    self.add_token(VerticalBar);
                }
            }
            '^' => self.add_token(Caret),
            '~' => self.add_token(Tilde),

            '#' => {
                // A comment goes until the end of the line
                while self.peek() != '\n' && !self.is_at_end() {
                    self.advance();
                }
            }

            ' ' | '\r' | '\t' => {}
            '\n' => {
                self.line += 1;
                self.column = 0;
            }

            '"' => self.add_string(),

            _ => {
                if c.is_ascii_digit() {
                    self.add_number();
                } else if c.is_alphabetic() || c == '_' {
                    self.add_identifier();
                } else {
                    self.generate_error(&format!("Unexpected character: {c}"));
                }
            }
        }
    }

    /// Creates a new token.
    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_with_literal(token_type, None);
    }

    /// Creates a new token with literal.
    fn add_token_with_literal(&mut self, token_type: TokenType, literal: Option<Box<dyn Any>>) {
        let text = self.source[self.start..self.current].to_string();
        self.tokens.push(Token::new(
            token_type,
            text,
            literal,
            self.line,
            self.column,
        ));
    }

    /// Adds string literal token.
    fn add_string(&mut self) {
        // TODO: Match single quote too. Maybe add a quote parameter?
        while self.peek() != '"' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            self.generate_error("Unterminated string");
        }

        // The closing quote
        self.advance();

        // Trim the surrounding quotes
        let value = self.source[(self.start + 1)..(self.current - 1)].to_string();
        self.add_token_with_literal(TokenType::String, Some(Box::new(value)));
    }

    /// Adds number literal token.
    fn add_number(&mut self) {
        while self.peek().is_numeric() {
            self.advance();
        }

        // Look for a fractional part
        if self.peek() == '.' && self.peek_next().is_numeric() {
            // Consume the dot
            self.advance();

            while self.peek().is_numeric() {
                self.advance();
            }
        }

        let value: f64 = self.source[self.start..self.current].parse().unwrap();
        self.add_token_with_literal(TokenType::Number, Some(Box::new(value)));
    }

    /// Adds identifier token.
    fn add_identifier(&mut self) {
        let c: char = self.peek();
        while c.is_alphanumeric() || c == '_' {
            self.advance();
        }

        let text = &self.source[self.start..self.current];
        let token_type = self.keywords.get(text);
        match token_type {
            Some(value) => self.add_token(*value),
            None => self.add_token(TokenType::Identifier),
        }
    }

    /// Generates an error with the given `message`.
    fn generate_error(&mut self, message: &str) {
        self.error = Some(LexError {
            what: message.to_string(),
            line: self.line,
            column: self.column,
        });
    }

    /// Returns `true` if current character is the last in the source code.
    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    /// Similar to `is_at_end()`, but checks if there's any error as well.
    fn done(&self) -> bool {
        self.error.is_some() || self.is_at_end()
    }

    /// Consumes the current character if it's what we're looking for.
    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.source.chars().nth(self.current).unwrap() != expected {
            return false;
        }

        self.current += 1;
        self.column += 1;
        true
    }

    /// Consumes and returns the next character in the source code.
    fn advance(&mut self) -> char {
        self.current += 1;
        self.column += 1;

        // NOTE: I read that using unwrap() function is bad. I might remove it
        // later.
        self.source.chars().nth(self.current - 1).unwrap()
    }

    /// Similar to `advance()`, but doesn't consume the character. This is called
    /// "lookahead".
    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source.chars().nth(self.current).unwrap()
    }

    /// Similar to `peek()`, but checks out the next character instead.
    fn peek_next(&self) -> char {
        if (self.current + 1) >= self.source.len() {
            return '\0';
        }
        self.source.chars().nth(self.current + 1).unwrap()
    }
}
