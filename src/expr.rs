use std::any::Any;

use crate::token::Token;

// TODO: Add more operators.

#[derive(Debug)]
pub enum Expr {
    Grouping(Box<Expr>),
    Literal(Option<Box<dyn Any>>),
    BinaryOp {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    UnaryOp {
        operator: Token,
        operand: Box<Expr>,
    },
}
