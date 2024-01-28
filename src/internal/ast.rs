use super::token::{Literal, Token};

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
    Block(Vec<Stmt>),
}

#[allow(dead_code)]
pub enum Expr {
    Binary(Box<Expr>, Token, Box<Expr>),
    Unary(Token, Box<Expr>),
    Grouping(Box<Expr>),
    Assign(Token, Box<Expr>),
    AugAssign(Box<Expr>, Token, Box<Expr>),
    Logical(Box<Expr>, Token, Box<Expr>),
    Call(Box<Expr>, Vec<Expr>),
    Constant(Literal),
    Variable(Token),
}

#[derive(Default)]
pub struct Program {
    statements: Vec<Stmt>,
}

impl Program {
    /// Appends a new statement to the list.
    pub fn add(&mut self, statement: Stmt) {
        self.statements.push(statement);
    }

    /// Returns a list of statements
    pub fn get(&self) -> &Vec<Stmt> {
        &self.statements
    }
}

pub trait Visitor<T> {
    fn visit_expr(&mut self, expr: &Expr) -> T;
}
