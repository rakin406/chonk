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

    /// Check if current character is the last in the source code.
    fn is_at_end(&self) -> bool;

    /// Return the next character in the source code.
    fn advance(&self) -> char;
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
            // TODO: Implement scan_token() method and use it here.
        }

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
        }
    }

    fn add_token(&self, token_type: TokenType) {
        // TODO: Take substring from source.
        let text = "";
    }

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }

    fn advance(&self) -> char {
        self.current += 1;
        // TODO: Return character from source.
    }
}
