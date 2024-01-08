use std::any::Any;

use crate::token::Token;
use crate::token_type::TokenType;

pub struct Lexer {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
}

pub trait Scannable {
    fn new(&mut self, source: String);

    /// Add tokens until character ends.
    fn scan_tokens(&self) -> &Vec<Token>;

    /// Add token type for the next character.
    fn scan_token(&self);

    /// Create a new token.
    fn add_token(&self, token_type: TokenType);

    /// Create a new token with literal.
    fn add_token_with_literal(&self, token_type: TokenType, literal: Option<Box<dyn Any>>);

    /// Add string literal token.
    fn add_string(&self);

    /// Add number literal token.
    fn add_number(&self);

    /// Add identifier token.
    fn add_identifier(&self);

    /// Check if current character is the last in the source code.
    fn is_at_end(&self) -> bool;

    /// Consume the current character if it's what we're looking for.
    fn has_match(&self, expected: char) -> bool;

    /// Consume and return the next character in the source code.
    fn advance(&self) -> char;

    /// Similar to advance(), but doesn't consume the character. This is called
    /// "lookahead".
    fn peek(&self) -> char;

    /// Similar to peek(), but checks out the next character instead.
    fn peek_next(&self) -> char;
}

impl Scannable for Lexer {
    fn new(&mut self, source: String) {
        self.source = source;
        self.start = 0;
        self.current = 0;
        self.line = 1;
    }

    fn scan_tokens(&self) -> &Vec<Token> {
        while !self.is_at_end() {
            // We are at the beginning of the next lexeme
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token(TokenType::EOF, "", None, self.line));
        &self.tokens
    }

    fn scan_token(&self) {
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

            '+' => self.add_token(if self.has_match('+') {
                PlusPlus
            } else if self.has_match('=') {
                PlusEqual
            } else {
                Plus
            }),

            '-' => self.add_token(if self.has_match('-') {
                MinusMinus
            } else if self.has_match('=') {
                MinusEqual
            } else {
                Minus
            }),

            '*' => self.add_token(if self.has_match('=') {
                AsteriskEqual
            } else {
                Asterisk
            }),

            '/' => self.add_token(if self.has_match('=') {
                SlashEqual
            } else {
                Slash
            }),

            '%' => self.add_token(if self.has_match('=') {
                PercentEqual
            } else {
                Percent
            }),

            '=' => self.add_token(if self.has_match('=') { EqualTo } else { Equal }),
            '!' => self.add_token(if self.has_match('=') { NotEqualTo } else { Not }),
            '>' => self.add_token(if self.has_match('=') {
                GreaterThanOrEqualTo
            } else {
                GreaterThan
            }),

            '<' => self.add_token(if self.has_match('=') {
                LessThanOrEqualTo
            } else {
                LessThan
            }),

            '&' => {
                if self.has_match('&') {
                    self.add_token(And);
                } else {
                    panic!("Line {}: Missing ampersand", self.line);
                }
            }

            '|' => {
                if self.has_match('|') {
                    self.add_token(Or);
                } else {
                    panic!("Line {}: Missing vertical bar", self.line);
                }
            }

            '#' => {
                // A comment goes until the end of the line
                while self.peek() != '\n' && !self.is_at_end() {
                    self.advance();
                }
            }

            ' ' => {}
            '\r' => {}
            '\t' => {}
            '\n' => self.line += 1,

            _ => {
                if c.is_numeric() {
                    self.add_number();
                } else if c.is_alphabetic() || c == '_' {
                    self.add_identifier();
                } else {
                    eprintln!("Line {}: Unexpected character", self.line);
                }
            }
        }
    }

    fn add_token(&self, token_type: TokenType) {
        self.add_token_with_literal(token_type, None);
    }

    fn add_token_with_literal(&self, token_type: TokenType, literal: Option<Box<dyn Any>>) {
        // TODO: Take substring from source.
        let text = "";
    }

    fn add_string(&self) {
        // TODO: Match double quote too. Maybe add a quote parameter?
        while self.peek() != '\'' && !self.is_at_end() {
            if self.peek() == '\n' {
                self.line += 1;
            }
            self.advance();
        }

        if self.is_at_end() {
            panic!("Line {}: Unterminated string", self.line);
        }

        // The closing quote
        self.advance();

        // TODO: Trim the surrounding quotes
        // self.add_token_with_literal(TokenType::String);
    }

    fn add_number(&self) {
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

        // TODO: Parse the number literal.
        // self.add_token_with_literal(TokenType::Number);
    }

    fn add_identifier(&self) {
        let c: char = self.peek();
        while c.is_alphanumeric() || c == '_' {
            self.advance();
        }

        self.add_token(TokenType::Identifier);
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn has_match(&self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }
        // TODO: Check if current and expected character matches.

        self.current += 1;
        true
    }

    fn advance(&self) -> char {
        self.current += 1;
        // TODO: Return character from source.
    }

    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        // NOTE: I read that using unwrap() function is bad. I might remove it
        // later.
        self.source.chars().nth(self.current).unwrap()
    }

    fn peek_next(&self) -> char {
        if (self.current + 1) >= self.source.len() {
            return '\0';
        }
        // TODO: Return next character from source.
    }
}
