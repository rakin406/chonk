use std::any::Any;

use crate::token::Token;

#[derive(Debug)]
pub enum Expr {
    Grouping(Box<Expr>),
    Literal(Option<Box<dyn Any>>),
    Binary {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    Unary {
        operator: Token,
        right: Box<Expr>,
    },
}
