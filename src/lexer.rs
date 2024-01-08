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

    /// Add string literal tokens.
    fn add_string(&self);

    /// Check if current character is the last in the source code.
    fn is_at_end(&self) -> bool;

    /// Consume the current character if it's what we're looking for.
    fn has_match(&self, expected: char) -> bool;

    /// Consume and return the next character in the source code.
    fn advance(&self) -> char;

    /// Similar to advance(), but doesn't consume the character. This is called
    /// "lookahead".
    fn peek(&self) -> char;
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

        self.tokens.push(Token {
            token_type: TokenType::EOF,
            lexeme: "",
            literal: None,
            line: self.line,
        });
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
            '-' => self.add_token(Minus),
            '+' => self.add_token(Plus),
            '/' => self.add_token(Slash),
            '*' => self.add_token(Asterisk),
            '%' => self.add_token(Percent),
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

            _ => eprintln!("Line {}: Unexpected character", self.line),
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
            panic!("Line {}: Unterminated string", self.line)
        }

        // The closing quote
        self.advance();

        // TODO: Trim the surrounding quotes
        // self.add_token_with_literal(TokenType::String);
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
        // TODO: Return character from source.
    }
}
