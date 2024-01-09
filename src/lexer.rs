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

impl Lexer {
    /// Create a new Lexer.
    pub fn new(&mut self, source: String) {
        self.source = source;
        self.start = 0;
        self.current = 0;
        self.line = 1;
    }

    /// Add tokens until character ends.
    pub fn scan_tokens(&mut self) -> &Vec<Token> {
        while !self.is_at_end() {
            // We are at the beginning of the next lexeme
            self.start = self.current;
            self.scan_token();
        }

        self.tokens.push(Token {
            token_type: TokenType::EOF,
            lexeme: "".to_string(),
            literal: None,
            line: self.line,
        });
        &self.tokens
    }

    /// Add token type for the next character.
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

            '+' => self.add_token(if self.match_char('+') {
                PlusPlus
            } else if self.match_char('=') {
                PlusEqual
            } else {
                Plus
            }),

            '-' => self.add_token(if self.match_char('-') {
                MinusMinus
            } else if self.match_char('=') {
                MinusEqual
            } else {
                Minus
            }),

            '*' => self.add_token(if self.match_char('=') {
                AsteriskEqual
            } else {
                Asterisk
            }),

            '/' => self.add_token(if self.match_char('=') {
                SlashEqual
            } else {
                Slash
            }),

            '%' => self.add_token(if self.match_char('=') {
                PercentEqual
            } else {
                Percent
            }),

            '=' => self.add_token(if self.match_char('=') { EqualTo } else { Equal }),
            '!' => self.add_token(if self.match_char('=') {
                NotEqualTo
            } else {
                Not
            }),
            '>' => self.add_token(if self.match_char('=') {
                GreaterThanOrEqualTo
            } else {
                GreaterThan
            }),

            '<' => self.add_token(if self.match_char('=') {
                LessThanOrEqualTo
            } else {
                LessThan
            }),

            '&' => {
                if self.match_char('&') {
                    self.add_token(And);
                } else {
                    panic!("Line {}: Missing ampersand", self.line);
                }
            }

            '|' => {
                if self.match_char('|') {
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

            '"' => self.add_string(),

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

    /// Create a new token.
    fn add_token(&self, token_type: TokenType) {
        self.add_token_with_literal(token_type, None);
    }

    /// Create a new token with literal.
    fn add_token_with_literal(&self, token_type: TokenType, literal: Option<Box<dyn Any>>) {
        let text = self.source[self.start..self.current].to_string();
        self.tokens.push(Token {
            token_type,
            lexeme: text,
            literal,
            line: self.line,
        });
    }

    /// Add string literal token.
    fn add_string(&mut self) {
        // TODO: Match single quote too. Maybe add a quote parameter?
        while self.peek() != '"' && !self.is_at_end() {
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

        // Trim the surrounding quotes
        let value = self.source[(self.start + 1)..(self.current - 1)].to_string();
        self.add_token_with_literal(TokenType::String, Some(Box::new(value)));
    }

    /// Add number literal token.
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

        self.add_token_with_literal(
            TokenType::Number,
            Some(Box::new(
                // NOTE: I might have to use unwrap() after parse.
                // WARNING: Not sure if it takes start and current too.
                self.source[self.start..self.current].parse::<f64>(),
            )),
        );
    }

    /// Add identifier token.
    fn add_identifier(&mut self) {
        let c: char = self.peek();
        while c.is_alphanumeric() || c == '_' {
            self.advance();
        }

        self.add_token(TokenType::Identifier);
    }

    /// Check if current character is the last in the source code.
    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    /// Consume the current character if it's what we're looking for.
    fn match_char(&mut self, expected: char) -> bool {
        if self.is_at_end() {
            return false;
        }

        if self.source.chars().nth(self.current).unwrap() != expected {
            return false;
        }

        self.current += 1;
        true
    }

    /// Consume and return the next character in the source code.
    fn advance(&mut self) -> char {
        self.current += 1;
        // NOTE: I read that using unwrap() function is bad. I might remove it
        // later.
        self.source.chars().nth(self.current - 1).unwrap()
    }

    /// Similar to advance(), but doesn't consume the character. This is called
    /// "lookahead".
    fn peek(&self) -> char {
        if self.is_at_end() {
            return '\0';
        }
        self.source.chars().nth(self.current).unwrap()
    }

    /// Similar to peek(), but checks out the next character instead.
    fn peek_next(&self) -> char {
        if (self.current + 1) >= self.source.len() {
            return '\0';
        }
        self.source.chars().nth(self.current + 1).unwrap()
    }
}
