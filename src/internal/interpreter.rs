use super::ast::{self, Visitor};
use super::token::Literal;
use super::token_type::TokenType;

pub struct Interpreter;

impl Interpreter {
    pub fn interpret(&self, expr: ast::Expr) {
        let literal = self.visit_expr(&expr);
        println!("{}", literal.to_string());
    }

    fn interpret_binary(&self, lhs: &ast::Expr, op: TokenType, rhs: &ast::Expr) -> Literal {
        let left = self.visit_expr(lhs);
        let right = self.visit_expr(rhs);

        match (left, op, right) {
            (Literal::Number(n1), TokenType::Greater, Literal::Number(n2)) => {
                return Literal::Bool(n1 > n2);
            }
            (Literal::Number(n1), TokenType::GreaterEqual, Literal::Number(n2)) => {
                return Literal::Bool(n1 >= n2);
            }
            (Literal::Number(n1), TokenType::Less, Literal::Number(n2)) => {
                return Literal::Bool(n1 < n2);
            }
            (Literal::Number(n1), TokenType::LessEqual, Literal::Number(n2)) => {
                return Literal::Bool(n1 <= n2);
            }
            (Literal::Number(n1), TokenType::BangEqual, Literal::Number(n2)) => {
                return Literal::Bool(n1 != n2);
            }
            (Literal::Number(n1), TokenType::EqEqual, Literal::Number(n2)) => {
                return Literal::Bool(n1 == n2);
            }
            (Literal::Number(n1), TokenType::Minus, Literal::Number(n2)) => {
                return Literal::Number(n1 - n2);
            }
            (Literal::Number(n1), TokenType::Plus, Literal::Number(n2)) => {
                return Literal::Number(n1 + n2);
            }
            (Literal::String(s1), TokenType::Plus, Literal::String(s2)) => {
                return Literal::String(s1 + &s2);
            }
            (Literal::Number(n1), TokenType::Slash, Literal::Number(n2)) => {
                return Literal::Number(n1 / n2);
            }
            (Literal::Number(n1), TokenType::Star, Literal::Number(n2)) => {
                return Literal::Number(n1 * n2);
            }
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
            Expr::Logical(lhs, op, rhs) => todo!(),
            Expr::Call(func, args) => todo!(),
            Expr::Constant(literal) => literal.to_owned(),
            Expr::Variable(token) => todo!(),
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
