use crate::internal::token::{Literal, Token};

#[derive(Debug, Clone)]
pub enum Expr {
    BinOp(Box<Expr>, Token, Box<Expr>),
    UnaryOp(Token, Box<Expr>),
    Grouping(Box<Expr>),
    Literal(Literal),
    Logical(Box<Expr>, Token, Box<Expr>),
    Variable(Token),
    Assign {
        name: Token,
        value: Box<Expr>,
    },
    Call {
        callee: Box<Expr>,
        paren: Token,
        arguments: Vec<Expr>,
    },
}

// trait ExprVisitor {
//     // fn visit_binary(binary: Expr::Binary);
// }
