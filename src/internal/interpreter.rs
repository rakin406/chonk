use super::ast::{self, Visitor};
use super::token::Literal;
use super::token_type::TokenType;

pub struct Interpreter;

impl Interpreter {
    pub fn interpret(&self, stmts: Vec<ast::Stmt>) {
        for stmt in stmts.iter() {
            self.walk_stmt(stmt);
        }
    }

    fn walk_stmt(&self, stmt: &ast::Stmt) {
        use ast::Stmt;

        match stmt {
            Stmt::FunctionDef { .. } => todo!(),
            Stmt::Return(_, _, _) => todo!(),
            Stmt::Delete(_, _) => todo!(),
            Stmt::For { .. } => todo!(),
            Stmt::While { .. } => todo!(),
            Stmt::If { .. } => todo!(),
            Stmt::Expr(expr) => {
                self.visit_expr(expr);
            }
            Stmt::Break => todo!(),
            Stmt::Continue => todo!(),
            Stmt::Echo(expr) => {
                let value = self.visit_expr(expr);
                println!("{}", value);
            }
            Stmt::Block(_, _, _) => todo!(),
        }
    }

    fn interpret_binary(&self, lhs: &ast::Expr, op: TokenType, rhs: &ast::Expr) -> Literal {
        let left = self.visit_expr(lhs);
        let right = self.visit_expr(rhs);

        match (left, op, right) {
            (Literal::Number(n1), TokenType::Greater, Literal::Number(n2)) => {
                Literal::Bool(n1 > n2)
            }
            (Literal::Number(n1), TokenType::GreaterEqual, Literal::Number(n2)) => {
                Literal::Bool(n1 >= n2)
            }
            (Literal::Number(n1), TokenType::Less, Literal::Number(n2)) => Literal::Bool(n1 < n2),
            (Literal::Number(n1), TokenType::LessEqual, Literal::Number(n2)) => {
                Literal::Bool(n1 <= n2)
            }
            (Literal::Number(n1), TokenType::BangEqual, Literal::Number(n2)) => {
                Literal::Bool(n1 != n2)
            }
            (Literal::Number(n1), TokenType::EqEqual, Literal::Number(n2)) => {
                Literal::Bool(n1 == n2)
            }
            (Literal::Number(n1), TokenType::Minus, Literal::Number(n2)) => {
                Literal::Number(n1 - n2)
            }
            (Literal::Number(n1), TokenType::Plus, Literal::Number(n2)) => Literal::Number(n1 + n2),
            (Literal::String(s1), TokenType::Plus, Literal::String(s2)) => {
                Literal::String(s1 + &s2)
            }
            (Literal::Number(n1), TokenType::Slash, Literal::Number(n2)) => {
                Literal::Number(n1 / n2)
            }
            (Literal::Number(n1), TokenType::Star, Literal::Number(n2)) => Literal::Number(n1 * n2),
            _ => Literal::Null,
        }
    }

    fn interpret_unary(&self, op: TokenType, rhs: &ast::Expr) -> Literal {
        let right = self.visit_expr(rhs);

        match (op, &right) {
            (TokenType::Minus, Literal::Number(value)) => Literal::Number(-value),
            (TokenType::Bang, _) => match is_truthy(right) {
                true => Literal::Bool(false),
                false => Literal::Bool(true),
            },
            _ => Literal::Null,
        }
    }
}

impl Visitor<Literal> for Interpreter {
    fn visit_expr(&self, expr: &ast::Expr) -> Literal {
        use ast::Expr;

        match expr {
            Expr::Binary(lhs, op, rhs) => self.interpret_binary(lhs, op.ty, rhs),
            Expr::Unary(op, rhs) => self.interpret_unary(op.ty, rhs),
            Expr::Grouping(e) => self.visit_expr(e),
            Expr::Assign(_exprs, _e) => todo!(),
            Expr::AugAssign(_lhs, _op, _rhs) => todo!(),
            Expr::Logical(_lhs, _op, _rhs) => todo!(),
            Expr::Call(_func, _args) => todo!(),
            Expr::Constant(literal) => literal.to_owned(),
            Expr::Variable(_token) => todo!(),
        }
    }
}

/// Returns `true` if the literal is "truthy".
fn is_truthy(literal: Literal) -> bool {
    match literal {
        Literal::Null => false,
        Literal::Bool(value) => value,
        _ => true,
    }
}
