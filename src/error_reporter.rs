use crate::token::Token;
use crate::token_type::TokenType;

pub trait ErrorReporter {
    // TODO: Add missing documentation.
    fn error(&self, line: usize, column: i64, message: &str) {
        self.report(line, column, "", message);
    }

    // TODO: Add missing documentation.
    fn token_error(&self, token: Token, message: &str) {
        if token.token_type == TokenType::Eof {
            self.report(token.line, token.column, " at end", message);
        } else {
            self.report(
                token.line,
                token.column,
                &format!(" at \'{}\'", token.lexeme),
                message,
            );
        }
    }

    // NOTE: I should probably reduce the amount of parameters here.
    // TODO: Add missing documentation.
    fn report(&self, line: usize, column: i64, location: &str, message: &str) {
        eprintln!(
            "[line {} col {}] Error{}: {}",
            line, column, location, message
        );
    }
}
