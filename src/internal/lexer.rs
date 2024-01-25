use std::collections::HashMap;
use std::fmt;

use super::token::{Literal, Token};
use super::token_type::TokenType;

/// All possible error types in `Lexer`.
pub enum LexError {
    UnexpectedChar { character: char, line: usize },
    UnterminatedString(usize),
}

struct Lexer {
    input: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    keywords: HashMap<String, TokenType>,
}

/// Scans tokens from source and returns it.
pub fn scan_tokens(input: String) -> Result<Vec<Token>, LexError> {
    let mut lexer = Lexer::new(input);
    lexer.scan_tokens()
}

impl fmt::Debug for LexError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LexError::UnexpectedChar { character, line } => {
                write!(
                    f,
                    "[line {}] LexError: Unexpected character \'{}\'",
                    line, character
                )
            }
            LexError::UnterminatedString(line) => {
                write!(f, "[line {}] LexError: Unterminated string", line)
            }
        }
    }
}

impl Default for Lexer {
    fn default() -> Self {
        Self {
            input: String::new(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
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
                ("del", TokenType::Delete),
                ("echo", TokenType::Echo),
            ])
            .into_iter()
            .map(|(key, value)| (String::from(key), value))
            .collect(),
        }
    }
}

impl Lexer {
    /// Creates a new `Lexer`.
    fn new(input: String) -> Self {
        Self {
            input,
            ..Default::default()
        }
    }

    /// Adds tokens from source until character ends.
    fn scan_tokens(&mut self) -> Result<Vec<Token>, LexError> {
        while !self.is_at_end() {
            // We are at the beginning of the next lexeme
            self.start = self.current;
            self.scan_token()?;
        }

        self.tokens
            .push(Token::new(TokenType::Eof, String::new(), None, self.line));

        Ok(self.tokens.clone())
    }

    /// Adds token type for the next character.
    fn scan_token(&mut self) -> Result<(), LexError> {
        use TokenType::*;

        let c: char = self.advance();
        match c {
            '(' => self.add_token(LParen),
            ')' => self.add_token(RParen),
            '[' => self.add_token(LBracket),
            ']' => self.add_token(RBracket),
            '{' => self.add_token(LBrace),
            '}' => self.add_token(RBrace),
            ',' => self.add_token(Comma),
            '.' => self.add_token(Dot),

            '+' => {
                if self.match_char('+') {
                    self.add_token(DoublePlus);
                } else if self.match_char('=') {
                    self.add_token(PlusEqual);
                } else {
                    self.add_token(Plus);
                }
            }
            '-' => {
                if self.match_char('-') {
                    self.add_token(DoubleMinus);
                } else if self.match_char('=') {
                    self.add_token(MinEqual);
                } else {
                    self.add_token(Minus);
                }
            }
            '*' => {
                if self.match_char('*') {
                    if self.match_char('=') {
                        self.add_token(DoubleStarEqual);
                    } else {
                        self.add_token(DoubleStar);
                    }
                } else if self.match_char('=') {
                    self.add_token(StarEqual);
                } else {
                    self.add_token(Star);
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
                    self.add_token(EqEqual);
                } else {
                    self.add_token(Equal);
                }
            }
            '!' => {
                if self.match_char('=') {
                    self.add_token(BangEqual);
                } else {
                    self.add_token(Bang);
                }
            }

            '>' => {
                if self.match_char('>') {
                    if self.match_char('=') {
                        self.add_token(RightShiftEqual);
                    } else {
                        self.add_token(RightShift);
                    }
                } else if self.match_char('=') {
                    self.add_token(GreaterEqual);
                } else {
                    self.add_token(Greater);
                }
            }
            '<' => {
                if self.match_char('<') {
                    if self.match_char('=') {
                        self.add_token(LeftShiftEqual);
                    } else {
                        self.add_token(LeftShift);
                    }
                } else if self.match_char('=') {
                    self.add_token(LessEqual);
                } else {
                    self.add_token(Less);
                }
            }

            '&' => {
                if self.match_char('&') {
                    self.add_token(DoubleAmper);
                } else if self.match_char('=') {
                    self.add_token(AmperEqual);
                } else {
                    self.add_token(Amper);
                }
            }
            '|' => {
                if self.match_char('|') {
                    self.add_token(DoubleVBar);
                } else if self.match_char('=') {
                    self.add_token(VBarEqual);
                } else {
                    self.add_token(VBar);
                }
            }
            '^' => {
                if self.match_char('=') {
                    self.add_token(CaretEqual);
                } else {
                    self.add_token(Caret);
                }
            }
            '~' => self.add_token(Tilde),

            '#' => {
                // A comment goes until the end of the line
                while self.peek() != '\n' && !self.is_at_end() {
                    self.advance();
                }
            }

            ' ' | '\r' | '\t' => {}
            '\n' => {
                self.add_token(Newline);
                self.line += 1;
            }

            '\'' => self.add_string('\'')?,
            '"' => self.add_string('"')?,

            _ => {
                if c.is_ascii_digit() {
                    self.add_number();
                } else if is_potential_identifier_start(c) {
                    self.add_identifier();
                } else {
                    return Err(LexError::UnexpectedChar {
                        character: c,
                        line: self.line,
                    });
                }
            }
        }

        Ok(())
    }

    /// Creates a new token.
    fn add_token(&mut self, ty: TokenType) {
        self.add_token_literal(ty, None);
    }

    /// Creates a new token with literal.
    fn add_token_literal(&mut self, ty: TokenType, literal: Option<Literal>) {
        let text = self.input[self.start..self.current].to_string();
        self.tokens.push(Token::new(ty, text, literal, self.line));
    }

    /// Adds string literal token.
    fn add_string(&mut self, delimiter: char) -> Result<(), LexError> {
        while self.peek() != delimiter && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            return Err(LexError::UnterminatedString(self.line));
        }

        // The closing quote
        self.advance();

        // Trim the surrounding quotes
        let value = self.input[(self.start + 1)..(self.current - 1)].to_string();
        self.add_token_literal(TokenType::String, Some(Literal::String(value)));

        Ok(())
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

        let value: f64 = self.input[self.start..self.current].parse().unwrap();
        self.add_token_literal(TokenType::Number, Some(Literal::Number(value)));
    }

    /// Adds identifier token.
    fn add_identifier(&mut self) {
        while is_potential_identifier_char(self.peek()) {
            self.advance();
        }

        let text = &self.input[self.start..self.current];
        let ty = self.keywords.get(text);
        match ty {
            Some(value) => self.add_token(*value),
            None => self.add_token(TokenType::Ident),
        }
    }

    /// Returns `true` if current character is the last in the source code.
    fn is_at_end(&self) -> bool {
        self.current >= self.input.len()
    }

    /// Consumes the current character if it's what we're looking for.
    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.input.chars().nth(self.current).unwrap() != expected {
            return false;
        }

        self.current += 1;
        true
    }

    /// Consumes and returns the next character in the source code.
    fn advance(&mut self) -> char {
        // WARNING: Not sure if `is_at_end()` should be here.
        if !self.is_at_end() {
            self.current += 1;
        }
        self.input.chars().nth(self.current - 1).unwrap()
    }

    /// Similar to `advance()`, but doesn't consume the character. This is called
    /// "lookahead".
    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.input.chars().nth(self.current).unwrap()
    }

    /// Similar to `peek()`, but checks out the next character instead.
    fn peek_next(&self) -> char {
        if (self.current + 1) >= self.input.len() {
            return '\0';
        }
        self.input.chars().nth(self.current + 1).unwrap()
    }
}

/// Returns `true` if character is a potential start for an identifier.
fn is_potential_identifier_start(c: char) -> bool {
    c.is_alphabetic() || c == '_'
}

/// Returns `true` if character is a potential part of an identifier.
fn is_potential_identifier_char(c: char) -> bool {
    c.is_alphanumeric() || c == '_'
}
