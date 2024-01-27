use super::token::{Literal, Token};

// enum Program {}

#[allow(dead_code)]
pub enum Stmt {
    FunctionDef {
        name: Expr,
        args: Option<Vec<Expr>>,
        body: Box<Stmt>,
        returns: Option<Expr>,
    },
    Return(Token, Option<Expr>, Token),
    Delete(Token, Vec<Expr>),

    For {
        target: Expr,
        body: Box<Stmt>,
    },
    While {
        test: Expr,
        body: Box<Stmt>,
    },
    If {
        test: Expr,
        body: Box<Stmt>,
        orelse: Option<Box<Stmt>>,
    },

    Expr(Expr),
    Break,
    Continue,
    Echo(Expr),
    Block(Token, Box<Stmt>, Token),
}

#[allow(dead_code)]
pub enum Expr {
    Binary(Box<Expr>, Token, Box<Expr>),
    Unary(Token, Box<Expr>),
    Grouping(Box<Expr>),
    Assign(Vec<Expr>, Box<Expr>),
    AugAssign(Box<Expr>, Token, Box<Expr>),
    Logical(Box<Expr>, Token, Box<Expr>),
    Call(Box<Expr>, Vec<Expr>),
    Constant(Literal),
    Variable(Token),
}

pub trait Visitor<T> {
    fn visit_expr(&self, expr: &Expr) -> T;
}
