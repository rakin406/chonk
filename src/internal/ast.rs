use super::token::{Literal, Token};

#[allow(dead_code)]
#[derive(Clone)]
pub enum Stmt {
    Function {
        name: Token,
        params: Vec<Token>,
        body: Vec<Stmt>,
    },
    Return {
        keyword: Token,
        value: Option<Expr>,
    },
    Delete(Token, Vec<Expr>),
    While {
        test: Expr,
        body: Vec<Stmt>,
    },
    If {
        test: Expr,
        body: Vec<Stmt>,
        or_else: Option<Vec<Stmt>>,
    },
    Expr(Expr),
    Break,
    Continue,
    Echo(Expr),
}

#[allow(dead_code)]
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
