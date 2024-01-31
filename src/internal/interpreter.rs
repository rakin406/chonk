use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::internal::ast::{Expr, Program, Stmt};
use crate::internal::runtime_error::RuntimeError;
use crate::internal::token::{Literal, Token, TokenType};

#[derive(Debug, Clone)]
enum Value {
    Number(f64),
    String(String),
    Bool(bool),
    Null,
    ChonkFunction(ChonkFunction),
}

pub struct Interpreter {
    globals: Environment,
    environment: Environment,
}

#[derive(Default, Clone)]
struct Environment {
    store: HashMap<String, Literal>,
    outer: Option<Box<Environment>>,
}

#[derive(Debug, Clone)]
struct ChonkFunction {
    // callee: Literal,
    arity: u8,
    callable: fn(&mut Interpreter, &[Value]) -> Value,
}

trait Callable {
    /// Returns the number of arguments of the function.
    fn arity(&self) -> u8;

    // TODO: Add missing documentation.
    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: &[Value],
    ) -> Result<Value, RuntimeError>;
}

impl Default for Interpreter {
    fn default() -> Self {
        let globals = Environment::default();
        // TODO: Define global functions.

        // globals.set(
        //     String::from("clock"),
        //     ChonkFunction::new(0, |_, _| {
        //         match SystemTime::now().duration_since(UNIX_EPOCH) {
        //             Ok(n) => Literal::Number(n.as_secs().into()),
        //             Err(_) => panic!("Time went backwards!"),
        //         }
        //     }),
        // );

        Self {
            globals: globals.to_owned(),
            environment: globals.to_owned(),
        }
    }
}

impl Interpreter {
    /// Interprets a program.
    pub fn interpret(&mut self, program: Program) -> Result<(), RuntimeError> {
        self.execute_multiple(program.get())
    }

    // fn execute_block(&mut self, statements: Vec<Stmt>, environment: Environment) {
    //     let previous = self.environment.clone();
    //     self.environment = environment;
    //
    //     for stmt in statements.iter() {
    //         self.walk_stmt(stmt);
    //     }
    //     self.environment = previous;
    // }

    /// Executes a list of statements.
    fn execute_multiple(&mut self, statements: &[Stmt]) -> Result<(), RuntimeError> {
        for stmt in statements.iter() {
            self.execute(stmt)?;
        }
        Ok(())
    }

    /// Executes statement.
    fn execute(&mut self, stmt: &Stmt) -> Result<(), RuntimeError> {
        match stmt {
            Stmt::Function { .. } => todo!(),
            Stmt::Return(_, _, _) => todo!(),
            Stmt::Delete(_, _) => todo!(),
            Stmt::For { .. } => todo!(),
            Stmt::While { test, body } => {
                while is_truthy(self.interpret_expr(test)?) {
                    self.execute_multiple(body)?;
                }
            }
            Stmt::If {
                test,
                body,
                or_else,
            } => {
                if is_truthy(self.interpret_expr(test)?) {
                    self.execute_multiple(body)?;
                } else if let Some(else_stmt) = or_else {
                    self.execute_multiple(else_stmt)?;
                }
            }
            Stmt::Expr(expr) => {
                self.interpret_expr(expr)?;
            }
            Stmt::Break => todo!(),
            Stmt::Continue => todo!(),
            Stmt::Echo(expr) => {
                let value = self.interpret_expr(expr)?;
                println!("{}", value);
            } // Stmt::Block(statements) => {
              //     // WARNING: I want control blocks to stay in the same outer scope. New
              //     // environment should only be created inside function blocks.
              //     // self.execute_block(
              //     //     statements.clone(),
              //     //     Environment::new_outer(Box::new(self.environment.clone())),
              //     // );
              // }
        }

        Ok(())
    }

    /// Interprets expression.
    fn interpret_expr(&mut self, expr: &Expr) -> Result<Value, RuntimeError> {
        match expr {
            Expr::Binary(lhs, op, rhs) => Ok(self.interpret_binary(lhs, op.clone(), rhs)?),
            Expr::Unary(op, rhs) => Ok(self.interpret_unary(op.ty, rhs)?),
            Expr::Grouping(e) => self.interpret_expr(e),
            Expr::Assign(name, e) => {
                let value = &self.interpret_expr(e)?;
                self.environment.set(name.lexeme.clone(), value.clone());
                Ok(value.clone())
            }
            Expr::AugAssign(_lhs, _op, _rhs) => todo!(),
            Expr::Logical(lhs, op, rhs) => {
                let left = &self.interpret_expr(lhs)?;

                if op.ty == TokenType::DoubleVBar {
                    if is_truthy(left.clone()) {
                        return Ok(left.clone());
                    }
                } else if !is_truthy(left.clone()) {
                    return Ok(left.clone());
                }

                self.interpret_expr(rhs)
            }
            Expr::Call(callee, paren, arguments) => {
                let callee_literal = &self.interpret_expr(callee)?;

                let mut args: Vec<Literal> = Vec::new();
                for arg in arguments.iter() {
                    args.push(self.interpret_expr(arg)?);
                }

                let function = ChonkFunction::new(callee_literal.clone());
                if args.len() != function.arity().into() {
                    return Err(RuntimeError::new(
                        paren.to_owned(),
                        &format!(
                            "Expected {} arguments but got {}",
                            function.arity(),
                            args.len()
                        ),
                    ));
                }

                Ok(function.call(self, &args))
            }
            Expr::Constant(literal) => Ok(literal.clone()),
            Expr::Variable(name) => self.environment.get(name),
        }
    }

    fn interpret_binary(
        &mut self,
        lhs: &Expr,
        op: Token,
        rhs: &Expr,
    ) -> Result<Value, RuntimeError> {
        let left = self.interpret_expr(lhs)?;
        let right = self.interpret_expr(rhs)?;

        match (left, op.ty, right) {
            (Literal::Number(n1), TokenType::Greater, Literal::Number(n2)) => {
                Ok(Literal::Bool(n1 > n2))
            }
            (Literal::Number(n1), TokenType::GreaterEqual, Literal::Number(n2)) => {
                Ok(Literal::Bool(n1 >= n2))
            }
            (Literal::Number(n1), TokenType::Less, Literal::Number(n2)) => {
                Ok(Literal::Bool(n1 < n2))
            }
            (Literal::Number(n1), TokenType::LessEqual, Literal::Number(n2)) => {
                Ok(Literal::Bool(n1 <= n2))
            }
            (Literal::Number(n1), TokenType::BangEqual, Literal::Number(n2)) => {
                Ok(Literal::Bool(n1 != n2))
            }
            (Literal::Number(n1), TokenType::EqEqual, Literal::Number(n2)) => {
                Ok(Literal::Bool(n1 == n2))
            }
            (Literal::Number(n1), TokenType::Minus, Literal::Number(n2)) => {
                Ok(Literal::Number(n1 - n2))
            }
            (Literal::Number(n1), TokenType::Plus, Literal::Number(n2)) => {
                Ok(Literal::Number(n1 + n2))
            }
            (Literal::String(s1), TokenType::Plus, Literal::String(s2)) => {
                Ok(Literal::String(s1 + &s2))
            }
            (Literal::Number(n1), TokenType::Slash, Literal::Number(n2)) => {
                Ok(Literal::Number(n1 / n2))
            }
            (Literal::Number(n1), TokenType::Star, Literal::Number(n2)) => {
                Ok(Literal::Number(n1 * n2))
            }
            _ => Err(RuntimeError::new(op, "Invalid operands in binary operator")),
        }
    }

    fn interpret_unary(&mut self, op: TokenType, rhs: &Expr) -> Result<Value, RuntimeError> {
        let right = self.interpret_expr(rhs)?;

        match (op, &right) {
            (TokenType::Minus, Literal::Number(value)) => Ok(Literal::Number(-value)),
            (TokenType::Bang, _) => match is_truthy(right) {
                true => Ok(Literal::Bool(false)),
                false => Ok(Literal::Bool(true)),
            },
            _ => Ok(Literal::Null),
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

impl Environment {
    /// Creates a new outer scope.
    #[allow(dead_code)]
    pub fn new_outer(outer: Box<Environment>) -> Self {
        Self {
            outer: Some(outer),
            ..Default::default()
        }
    }

    /// Returns the literal value bound to the name.
    pub fn get(&self, name: &Token) -> Result<Literal, RuntimeError> {
        if let Some(value) = self.store.get(&name.lexeme) {
            return Ok(value.clone());
        }

        if let Some(outer_env) = &self.outer {
            return outer_env.get(name);
        }

        Err(RuntimeError::new(
            name.to_owned(),
            &format!("Undefined variable \"{}\"", name.lexeme),
        ))
    }

    /// Binds a new name to a value. If the name exists, it assigns a new value
    /// to it.
    pub fn set(&mut self, name: String, value: Literal) {
        self.store.insert(name, value);
    }
}


impl ChonkFunction {
    /// Creates a new `ChonkFunction`.
    // fn new(callee: Literal) -> Self {
    //     Self { callee }
    // }

    fn new(
        // callee: Literal,
        arity: u8,
        callable: fn(&mut Interpreter, &[Value]) -> Value,
    ) -> Self {
        Self {
            // callee,
            arity,
            callable,
        }
    }
}

impl Callable for ChonkFunction {
    fn arity(&self) -> u8 {
        self.arity
    }

    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: &[Value],
    ) -> Result<Value, RuntimeError> {
        Ok((self.callable)(interpreter, arguments))
    }
}
