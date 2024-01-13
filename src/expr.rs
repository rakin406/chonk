use crate::token::Token;

struct Expr {
    binary: Box<Binary>,
}

struct Binary {
    left: Expr,
    operator: Token,
    right: Expr,
}

impl Binary {
    /// Creates a new `Binary`.
    fn new(left: Expr, operator: Token, right: Expr) -> Self {
        Self {
            left,
            operator,
            right,
        }
    }
}
