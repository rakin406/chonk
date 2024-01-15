use crate::token::Token;
use crate::token_type::TokenType;

pub trait ErrorReporter {
    // TODO: Add missing documentation.
    fn error(&self, line: usize, message: &str) {
        self.report(line, "", message);
    }

    // TODO: Add missing documentation.
    fn token_error(&self, token: Token, message: &str) {
        if token.token_type == TokenType::Eof {
            self.report(token.line, " at end", message);
        } else {
            self.report(token.line, &format!(" at \'{}\'", token.lexeme), message);
        }
    }

    // NOTE: I should probably reduce the amount of parameters here.
    // TODO: Add missing documentation.
    fn report(&self, line: usize, location: &str, message: &str) {
        eprintln!("[line {}] Error{}: {}", line, location, message);
    }
}
