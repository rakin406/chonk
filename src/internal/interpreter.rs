use super::ast::{Expr, Program, Stmt, Visitor};
use super::environment::Environment;
use super::token::Literal;
use super::token_type::TokenType;

#[derive(Default)]
pub struct Interpreter {
    environment: Environment,
}

impl Interpreter {
    /// Interprets a program.
    pub fn interpret(&mut self, program: Program) {
        for stmt in program.get().iter() {
            self.walk_stmt(stmt);
        }
    }

    fn walk_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::FunctionDef { .. } => todo!(),
            Stmt::Return(_, _, _) => todo!(),
            Stmt::Delete(_, _) => todo!(),
            Stmt::For { .. } => todo!(),
            Stmt::While { .. } => todo!(),
            Stmt::If {
                test,
                body,
                or_else,
            } => {
                if is_truthy(self.visit_expr(test)) {
                    self.walk_stmt(body);
                } else if let Some(else_stmt) = or_else {
                    self.walk_stmt(else_stmt);
                }
            }
            Stmt::Expr(expr) => {
                self.visit_expr(expr);
            }
            Stmt::Break => todo!(),
            Stmt::Continue => todo!(),
            Stmt::Echo(expr) => {
                let value = self.visit_expr(expr);
                println!("{}", value);
            }
            Stmt::Block(statements) => {
                self.execute_block(
                    statements.to_owned(),
                    Environment::new_outer(Box::new(self.environment.to_owned())),
                );
            }
        }
    }

    fn execute_block(&mut self, statements: Vec<Stmt>, environment: Environment) {
        let previous = self.environment.clone();
        self.environment = environment;

        for stmt in statements.iter() {
            self.walk_stmt(stmt);
        }
        self.environment = previous;
    }

    fn interpret_binary(&mut self, lhs: &Expr, op: TokenType, rhs: &Expr) -> Literal {
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

    fn interpret_unary(&mut self, op: TokenType, rhs: &Expr) -> Literal {
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
    fn visit_expr(&mut self, expr: &Expr) -> Literal {
        match expr {
            Expr::Binary(lhs, op, rhs) => self.interpret_binary(lhs, op.ty, rhs),
            Expr::Unary(op, rhs) => self.interpret_unary(op.ty, rhs),
            Expr::Grouping(e) => self.visit_expr(e),
            Expr::Assign(name, e) => {
                let value = &self.visit_expr(e);
                self.environment
                    .set(name.lexeme.to_owned(), value.to_owned());
                value.to_owned()
            }
            Expr::AugAssign(_lhs, _op, _rhs) => todo!(),
            Expr::Logical(lhs, op, rhs) => {
                let left = &self.visit_expr(lhs);

                if op.ty == TokenType::DoubleVBar {
                    if is_truthy(left.to_owned()) {
                        return left.to_owned();
                    }
                } else if !is_truthy(left.to_owned()) {
                    return left.to_owned();
                }

                self.visit_expr(rhs)
            }
            Expr::Call(_func, _args) => todo!(),
            Expr::Constant(literal) => literal.to_owned(),
            Expr::Variable(name) => self.environment.get(name),
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
