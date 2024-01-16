use crate::token::{Literal, Token};

// TODO: Add more operators.

#[derive(Debug)]
pub enum Expr {
    Grouping(Box<Expr>),
    Literal(Literal),
    BinaryOp {
        left: Box<Expr>,
        operator: Token,
        right: Box<Expr>,
    },
    UnaryOp {
        operator: Token,
        right: Box<Expr>,
    },
}
