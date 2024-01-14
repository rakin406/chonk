use crate::token::Token;

pub struct Expr {
    pub binary: Box<BinaryOp>,
}

pub struct BinaryOp {
    left: Expr,
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
