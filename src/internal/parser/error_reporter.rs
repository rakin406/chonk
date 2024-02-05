use crate::internal::token::{token_type, Token};

#[derive(Debug)]
#[allow(dead_code)]
pub enum ErrorType {
    SyntaxError,
    RuntimeError,
}

pub trait ErrorReporter {
    const ERROR_TYPE: ErrorType;

    /// Reports an error.
    fn error(&self, line: usize, message: &str) {
        self.report(line, "", message);
    }

    /// Reports a token error.
    fn token_error(&self, token: &Token, message: &str) {
        if token_type::is_eof(token.ty) {
            self.report(token.line, " at end", message);
        } else {
            self.report(token.line, &format!(" at \"{}\"", token.lexeme), message);
        }
    }

    /// Pretty prints an error with the given information.
    fn report(&self, line: usize, location: &str, message: &str) {
        eprintln!(
            "[line {}] {:#?}{}: {}",
            line,
            Self::ERROR_TYPE,
            location,
            message
        );
    }
}
