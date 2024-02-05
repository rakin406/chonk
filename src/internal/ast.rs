use super::token::{Literal, Token};

#[derive(Clone)]
pub enum Stmt {
    Function {
        name: Token,
        params: Vec<Token>,
        body: Vec<Stmt>,
    },
    While {
        test: Expr,
        body: Vec<Stmt>,
    },
    If {
        test: Expr,
        body: Vec<Stmt>,
        or_else: Option<Vec<Stmt>>,
    },
    Return(Option<Expr>),
    Delete(Vec<Token>),
    Expr(Expr),
    Echo(Expr),
}

#[derive(Clone)]
pub enum Expr {
    Binary(Box<Expr>, Token, Box<Expr>),
    Unary(Token, Box<Expr>),
    Grouping(Box<Expr>),
    Assign(Token, Box<Expr>),
    Logical(Box<Expr>, Token, Box<Expr>),
    Call(Box<Expr>, Token, Vec<Expr>),
    Constant(Literal),
    Variable(Token),
    AugAssign {
        name: Token,
        operator: Token,
        value: Box<Expr>,
    },
    Prefix {
        operator: Token,
        name: Token,
    },
}
