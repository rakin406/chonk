use std::fmt;

use crate::internal::token::{Token, TokenType};

/// All possible error types in the parser.
pub enum ParseError {
    ExpectedExpression(Token),
    TokenMismatch {
        expected: TokenType,
        found: Token,
        message: String,
    },
}

impl fmt::Debug for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::ExpectedExpression(token) => {
                write!(
                    f,
                    "[line {}] ParseError: Expected expression, but found token {:#?}",
                    token.line, token.ty
                )
            }
            ParseError::TokenMismatch {
                expected,
                found,
                message,
            } => {
                write!(
                    f,
                    "[line {}] ParseError: Expected token {:#?} but found {:#?}: {}",
                    found.line, expected, found.ty, message
                )
            }
        }
    }
}
