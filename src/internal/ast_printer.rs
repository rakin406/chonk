use super::ast::{self, Visitor};
use super::token::Literal;

pub struct AstPrinter;

impl AstPrinter {
    pub fn print_ast(&self, expr: ast::Expr) -> String {
        self.visit_expr(&expr)
    }

    fn parenthesize(&self, name: String, expr: ast::Expr) -> String {
        self.parenthesize_multiple(name, Vec::from([expr]))
    }

    fn parenthesize_multiple(&self, name: String, exprs: Vec<ast::Expr>) -> String {
        let mut value = String::new();

        value.push('(');
        value.push_str(&name);

        for expr in exprs.iter() {
            value.push(' ');
            value.push_str(&self.visit_expr(expr));
        }

        value.push(')');
        value
    }
}

impl ast::Visitor<String> for AstPrinter {
    fn visit_expr(&self, expr: &ast::Expr) -> String {
        use ast::Expr;

        // TODO: Finish Logical, Call and Variable blah blah blah.
        match expr {
            Expr::BoolOp(op, rhs) => {
                self.parenthesize_multiple(op.lexeme.to_owned(), rhs.to_owned())
            }
            Expr::BinOp(lhs, op, rhs) => self.parenthesize_multiple(
                op.lexeme.to_owned(),
                Vec::from([*lhs.to_owned(), *rhs.to_owned()]),
            ),
            Expr::UnaryOp(op, rhs) => self.parenthesize(op.lexeme.to_owned(), *rhs.to_owned()),
            Expr::Grouping(e) => self.parenthesize(String::from("group"), *e.to_owned()),
            Expr::Logical(lhs, op, rhs) => todo!(),
            Expr::Call { func, args } => todo!(),
            Expr::Constant(literal) => match literal {
                Literal::Number(value) => return value.to_string(),
                Literal::String(value) => return value.to_owned(),
                Literal::True(_) => return String::from("true"),
                Literal::False(_) => return String::from("false"),
                Literal::Null => return String::from("null"),
            },
            Expr::Variable(token) => todo!(),
        }
    }
}
