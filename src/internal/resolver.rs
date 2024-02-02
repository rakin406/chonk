use std::collections::HashMap;

use crate::internal::ast::{Expr, Stmt};
use crate::internal::interpreter::Interpreter;

#[allow(dead_code)]
#[derive(Default)]
struct Resolver {
    interpreter: Interpreter,
    scopes: Vec<HashMap<String, bool>>,
}

#[allow(dead_code)]
impl Resolver {
    /// Creates a new `Resolver`.
    fn new(interpreter: Interpreter) -> Self {
        Self {
            interpreter,
            ..Default::default()
        }
    }

    /// Resolves a list of statements.
    fn resolve(&mut self, statements: &[Stmt]) {
        for stmt in statements.iter() {
            self.walk_stmt(stmt);
        }
    }

    fn walk_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Function { name, params, body } => {}
            Stmt::Return { keyword: _, value } => {}
            Stmt::Delete(_, _) => todo!(),
            Stmt::For { .. } => todo!(),
            Stmt::While { test, body } => {}
            Stmt::If {
                test,
                body,
                or_else,
            } => {}
            Stmt::Expr(expr) => {}
            Stmt::Break => todo!(),
            Stmt::Continue => todo!(),
            Stmt::Echo(expr) => {}
        }
    }
}
