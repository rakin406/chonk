use super::token::{Literal, Token};

// enum Program {}

#[derive(Debug)]
pub enum Stmt {
    FunctionDef {
        name: Expr,
        args: Option<Vec<Expr>>,
        body: Box<Stmt>,
        returns: Option<Expr>,
    },
    Return(Token, Option<Expr>, Token),
    Delete(Token, Vec<Expr>),
    Assign {
        targets: Vec<Expr>,
        value: Expr,
    },
    AugAssign {
        target: Expr,
        op: Token,
        value: Expr,
    },

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

    Expr(Expr, Token),
    Break,
    Continue,
    Echo(Token, Expr, Token),
    Block(Token, Box<Stmt>, Token),
}

#[derive(Debug)]
pub enum Expr {
    BoolOp(Token, Vec<Expr>),
    BinOp(Box<Expr>, Token, Box<Expr>),
    UnaryOp(Token, Box<Expr>),
    Grouping(Box<Expr>),
    Logical(Box<Expr>, Token, Box<Expr>),
    Call { func: Box<Expr>, args: Vec<Expr> },
    Constant(Literal),
    Variable(Token),
}

pub trait Visitor<T> {
    fn visit_expr(&self, expr: &Expr) -> T;
}
