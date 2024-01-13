use crate::token::Token;

pub struct Expr {
    pub binary: Box<Binary>,
}

pub struct Binary {
    left: Expr,
    operator: Token,
    right: Expr,
}

impl Binary {
    /// Creates a new `Binary`.
    pub fn new(left: Expr, operator: Token, right: Expr) -> Self {
        Self {
            left,
            operator,
            right,
        }
    }
}
