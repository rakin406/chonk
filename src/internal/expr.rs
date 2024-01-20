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

// NOTE: I don't know what to do with these yet.
// impl Expr {
//     pub fn accept<T>(&self, visitor: &mut dyn Visitor<T>) -> T {
//         visitor.visit_expr(self)
//     }
// }
//
// pub trait Visitor<T> {
//     fn visit_expr(&mut self, expr: &Expr) -> T;
// }
