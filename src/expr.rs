use crate::token::Token;

#[derive(Clone, Debug)]
pub enum Expr {
    Binary(Box<BinaryOp>),
    Unary(Box<UnaryOp>),
}

#[derive(Clone, Debug)]
pub struct BinaryOp {
    left: Expr,
    operator: Token,
    right: Expr,
}

#[derive(Clone, Debug)]
pub struct UnaryOp {
    operator: Token,
    right: Expr,
}

impl BinaryOp {
    /// Creates a new `Binary`.
    pub fn new(left: Expr, operator: Token, right: Expr) -> Self {
        Self {
            left,
            operator,
            right,
        }
    }
}

impl UnaryOp {
    /// Creates a new `UnaryOp`.
    pub fn new(operator: Token, right: Expr) -> Self {
        Self { operator, right }
    }
}
