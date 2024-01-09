use std::collections::HashMap;

use crate::token::{Literal, Token};
use crate::token_type::TokenType;

// TODO: Add `keywords` hashmap field here.
struct Lexer {
    source: String,
    tokens: Vec<Token>,
    start: usize,
    current: usize,
    line: usize,
    column: i64,
    keywords: HashMap<String, TokenType>,
}

impl Default for Lexer {
    fn default() -> Self {
        Self {
            source: String::new(),
            tokens: Vec::new(),
            start: 0,
            current: 0,
            line: 1,
            column: -1,
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
    /// Create a new Lexer.
    pub fn new(source: String) -> Self {
        Self {
            source,
            ..Default::default()
        }
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
            lexeme: String::new(),
            literal: None,
            line: self.line,
            column: self.column,
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
    fn add_token(&mut self, token_type: TokenType) {
        self.add_token_with_literal(token_type, None);
    }

    /// Create a new token with literal.
    fn add_token_with_literal(&mut self, token_type: TokenType, literal: Option<Literal>) {
        let text = self.source[self.start..self.current].to_string();
        self.tokens.push(Token {
            token_type,
            lexeme: text,
            literal,
            line: self.line,
            // column: ,
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
        self.add_token_with_literal(TokenType::String, Some(value));
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
            Some(
                // NOTE: I might have to use unwrap() after parse.
                self.source[self.start..self.current].parse::<f64>(),
            ),
        );
    }

    /// Add identifier token.
    fn add_identifier(&mut self) {
        let c: char = self.peek();
        while c.is_alphanumeric() || c == '_' {
            self.advance();
        }

        let text = &self.source[self.start..self.current];
        let token_type = KEYWORDS.get(&text);
        match token_type {
            Some(value) => self.add_token(*value),
            None => self.add_token(TokenType::Identifier),
        }
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
        self.column += 1;

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
