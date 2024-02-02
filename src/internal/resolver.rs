use std::collections::HashMap;

use crate::internal::ast::{Expr, Stmt};
use crate::internal::interpreter::Interpreter;
use crate::internal::token::Token;

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
            self.resolve_stmt(stmt);
        }
    }

    fn begin_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    fn end_scope(&mut self) {
        self.scopes.pop();
    }

    fn define(&mut self, name: &Token) {
        if !self.scopes.is_empty() {
            if let Some(map) = self.scopes.first_mut() {
                map.insert(name.lexeme.to_owned(), true);
            }
        }
    }

    fn resolve_local(&mut self, expr: &Expr, name: &Token) {
        todo!();
    }

    fn resolve_function(&mut self, params: &[Token], body: &[Stmt]) {
        self.begin_scope();
        for param in params.iter() {
            self.define(param);
        }
        self.resolve(body);
        self.end_scope();
    }

    fn resolve_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Function { name, params, body } => {
                self.define(name);
                self.resolve_function(params, body);
            }
            Stmt::Return { keyword: _, value } => {
                if let Some(expr) = value {
                    self.resolve_expr(expr);
                }
            }
            Stmt::Delete(_, _) => todo!(),
            Stmt::For { .. } => todo!(),
            Stmt::While { test, body } => {
                self.resolve_expr(test);
                self.resolve(body);
            }
            Stmt::If {
                test,
                body,
                or_else,
            } => {
                self.resolve_expr(test);
                self.resolve(body);
                if let Some(value) = or_else {
                    self.resolve(value);
                }
            }
            Stmt::Expr(expr) => {
                self.resolve_expr(expr);
            }
            Stmt::Break => todo!(),
            Stmt::Continue => todo!(),
            Stmt::Echo(expr) => {
                self.resolve_expr(expr);
            }
        }
    }

    fn resolve_expr(&mut self, expr: &Expr) {
        match expr {
            Expr::Binary(lhs, _, rhs) => {
                self.resolve_expr(lhs);
                self.resolve_expr(rhs);
            }
            Expr::Unary(op, rhs) => todo!(),
            Expr::Grouping(e) => todo!(),
            Expr::Assign(name, e) => {
                self.resolve_expr(e);
                self.resolve_local(expr, name);
            }
            Expr::AugAssign(_lhs, _op, _rhs) => todo!(),
            Expr::Logical(lhs, op, rhs) => todo!(),
            Expr::Call(callee, paren, arguments) => todo!(),
            Expr::Constant(literal) => todo!(),
            Expr::Variable(name) => {
                // Gotta love this nesting...
                if !self.scopes.is_empty() {
                    if let Some(map) = self.scopes.first() {
                        if let Some(value) = map.get(&name.lexeme) {
                            if !value {
                                todo!("report error");
                            }
                        }
                    }
                }

                self.resolve_local(expr, name);
            }
        }
    }
}
