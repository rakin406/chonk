use super::expr::{Expr, ExprVisitor};

// TODO: Finish this.

pub struct AstPrinter {}

impl AstPrinter {
    pub fn print_ast(&self, expr: Expr) -> String {}

    fn parenthesize_single(&self, name: String, expr: Expr) -> String {
        let value = String::new();

        value.push('(');
        value.push_str(&name);

        value.push(' ');
        value.push_str(expr.accept(self));
        value.push(')');

        value
    }

    fn parenthesize_multiple(&self, name: String, exprs: Vec<Expr>) -> String {}
}

impl ExprVisitor<String> for AstPrinter {
    fn visit_expr(&mut self, expr: &Expr) -> String {}
}
