use crate::token::Token;

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

    /// Check if current character is the last in the source code.
    fn is_at_end(&self) -> bool;
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

    fn is_at_end(&self) -> bool {
        self.current >= self.source.len()
    }
}
