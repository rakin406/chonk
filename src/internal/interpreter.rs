use super::ast::{self, Visitor};
use super::token::Literal;
use super::token_type::TokenType;

struct Interpreter {}

impl Visitor<Literal> for Interpreter {
    fn visit_expr(&self, expr: &ast::Expr) -> Literal {
        use ast::Expr;

        match expr {
            Expr::BoolOp(op, rhs) => todo!(),
            Expr::BinOp(lhs, op, rhs) => {
                let left = self.visit_expr(lhs);
                let right = self.visit_expr(rhs);

                match op.ty {
                    // TODO: I seriously need to change my design and code
                    // structure. I need so many match statements here that it's
                    // actually crazy.
                    TokenType::Minus => {}
                    TokenType::Slash => {}
                    TokenType::Star => {}
                    _ => {}
                }

                return Literal::Null;
            }
            Expr::UnaryOp(op, rhs) => {
                let right = self.visit_expr(rhs);

                match right {
                    Literal::Number(value) => match op.ty {
                        TokenType::Minus => return Literal::Number(-value),
                        _ => {}
                    },
                    _ => match op.ty {
                        TokenType::Bang => match is_truthy(right) {
                            true => return Literal::False(false),
                            false => return Literal::True(true),
                        },
                        _ => {}
                    },
                }

                return Literal::Null;
            }
            Expr::Grouping(e) => self.visit_expr(e),
            Expr::Logical(lhs, op, rhs) => todo!(),
            Expr::Call { func, args } => todo!(),
            Expr::Constant(literal) => return literal.to_owned(),
            Expr::Variable(token) => todo!(),
        }
    }
}

/// Returns `true` if the literal is "truthy".
fn is_truthy(literal: Literal) -> bool {
    match literal {
        Literal::Null => return false,
        Literal::True(value) | Literal::False(value) => return value,
        _ => return true,
    }
}
