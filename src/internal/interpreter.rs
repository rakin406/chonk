use std::collections::HashMap;
use std::fmt;
use std::iter::zip;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::internal::ast::{Expr, Program, Stmt};
use crate::internal::runtime_error::RuntimeError;
use crate::internal::token::{Literal, Token, TokenType};

// TODO: Variables inside a function should be visible to functions inside that
// function. Gotta figure out how to implement it.

pub struct Interpreter {
    globals: Environment,
    environment: Environment,
}

trait Callable {
    /// Returns the number of arguments of the function.
    fn arity(&self) -> u8;

    /// Calls the chonk function.
    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: &[Value],
    ) -> Result<Value, RuntimeError>;
}

impl Default for Interpreter {
    fn default() -> Self {
        let mut globals = Environment::default();

        globals.set(
            String::from("clock"),
            &Value::NativeFunction(NativeFunction {
                name: String::from("clock"),
                arity: 0,
                callable: |_, _| match SystemTime::now().duration_since(UNIX_EPOCH) {
                    Ok(n) => Ok(Value::Number(n.as_secs_f64())),
                    Err(_) => panic!("Time went backwards!"),
                },
            }),
        );

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

    /// Executes a list of statements.
    fn execute_multiple(&mut self, statements: &[Stmt]) -> Result<(), RuntimeError> {
        for stmt in statements.iter() {
            self.execute(stmt)?;
        }
        Ok(())
    }

    /// Executes a list of statements in a new isolated environment.
    fn execute_new(
        &mut self,
        statements: &[Stmt],
        environment: Environment,
    ) -> Result<(), RuntimeError> {
        let previous = self.environment.clone();
        self.environment = environment;
        self.execute_multiple(statements)?;
        self.environment = previous;

        Ok(())
    }

    /// Executes statement.
    fn execute(&mut self, stmt: &Stmt) -> Result<(), RuntimeError> {
        match stmt {
            Stmt::Function { name, params, body } => {
                // NOTE: Too many clones here!
                let function = ChonkFunction::new(name.clone(), params.clone(), body.clone());
                self.environment
                    .set(name.lexeme.clone(), &Value::ChonkFunction(function));
            }
            Stmt::Return { keyword: _, value } => {
                if let Some(expr) = value {
                    self.interpret_expr(expr)?;
                }
            }
            Stmt::Delete(_, _) => todo!(),
            Stmt::For { .. } => todo!(),
            Stmt::While { test, body } => {
                while is_truthy(&self.interpret_expr(test)?) {
                    self.execute_multiple(body)?;
                }
            }
            Stmt::If {
                test,
                body,
                or_else,
            } => {
                if is_truthy(&self.interpret_expr(test)?) {
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
            }
        }

        Ok(())
    }

    /// Interprets expression.
    fn interpret_expr(&mut self, expr: &Expr) -> Result<Value, RuntimeError> {
        match expr {
            Expr::Binary(lhs, op, rhs) => Ok(self.interpret_binary(lhs, op.clone(), rhs)?),
            Expr::Unary(op, rhs) => Ok(self.interpret_unary(op.clone(), rhs)?),
            Expr::Grouping(e) => self.interpret_expr(e),
            Expr::Assign(name, e) => {
                let value = self.interpret_expr(e)?;
                self.environment.set(name.lexeme.clone(), &value);
                Ok(value)
            }
            Expr::AugAssign(_lhs, _op, _rhs) => todo!(),
            Expr::Logical(lhs, op, rhs) => {
                let left = self.interpret_expr(lhs)?;

                if op.ty == TokenType::DoubleVBar {
                    if is_truthy(&left) {
                        return Ok(left);
                    }
                } else if !is_truthy(&left) {
                    return Ok(left);
                }

                self.interpret_expr(rhs)
            }
            Expr::Call(callee, paren, arguments) => self.call(callee, paren, arguments),
            Expr::Constant(literal) => Ok(get_value(literal)),
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
            (Value::Number(n1), TokenType::Greater, Value::Number(n2)) => Ok(Value::Bool(n1 > n2)),
            (Value::Number(n1), TokenType::GreaterEqual, Value::Number(n2)) => {
                Ok(Value::Bool(n1 >= n2))
            }
            (Value::Number(n1), TokenType::Less, Value::Number(n2)) => Ok(Value::Bool(n1 < n2)),
            (Value::Number(n1), TokenType::LessEqual, Value::Number(n2)) => {
                Ok(Value::Bool(n1 <= n2))
            }
            (Value::Number(n1), TokenType::BangEqual, Value::Number(n2)) => {
                Ok(Value::Bool(n1 != n2))
            }
            (Value::Number(n1), TokenType::EqEqual, Value::Number(n2)) => Ok(Value::Bool(n1 == n2)),
            (Value::Number(n1), TokenType::Minus, Value::Number(n2)) => Ok(Value::Number(n1 - n2)),
            (Value::Number(n1), TokenType::Plus, Value::Number(n2)) => Ok(Value::Number(n1 + n2)),
            (Value::String(s1), TokenType::Plus, Value::String(s2)) => Ok(Value::String(s1 + &s2)),
            (Value::Number(n1), TokenType::Slash, Value::Number(n2)) => Ok(Value::Number(n1 / n2)),
            (Value::Number(n1), TokenType::Star, Value::Number(n2)) => Ok(Value::Number(n1 * n2)),
            _ => Err(RuntimeError::new(op, "Invalid operands in binary operator")),
        }
    }

    fn interpret_unary(&mut self, op: Token, rhs: &Expr) -> Result<Value, RuntimeError> {
        let right = &self.interpret_expr(rhs)?;

        match (op.ty, right) {
            (TokenType::Minus, Value::Number(value)) => Ok(Value::Number(-value)),
            (TokenType::Bang, _) => match is_truthy(right) {
                true => Ok(Value::Bool(false)),
                false => Ok(Value::Bool(true)),
            },
            _ => Err(RuntimeError::new(op, "Invalid operand to unary operator")),
        }
    }

    fn call(
        &mut self,
        callee: &Expr,
        paren: &Token,
        arguments: &[Expr],
    ) -> Result<Value, RuntimeError> {
        let callee_value = self.interpret_expr(callee)?;

        let mut args: Vec<Value> = Vec::new();
        for arg in arguments.iter() {
            args.push(self.interpret_expr(arg)?);
        }

        let function = if let Some(func) = callee_value.as_callable() {
            func
        } else {
            return Err(RuntimeError::new(
                paren.to_owned(),
                "Can only call functions",
            ));
        };

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

        function.call(self, &args)
    }
}

/// Returns `true` if the value is "truthy".
fn is_truthy(value: &Value) -> bool {
    match value {
        Value::Null => false,
        Value::Bool(value) => *value,
        _ => true,
    }
}

/// Returns value from literal.
fn get_value(literal: &Literal) -> Value {
    match literal {
        Literal::Number(n) => Value::Number(*n),
        Literal::String(s) => Value::String(s.clone()),
        Literal::True => Value::Bool(true),
        Literal::False => Value::Bool(false),
        Literal::Null => Value::Null,
    }
}

#[derive(Clone)]
enum Value {
    Number(f64),
    String(String),
    Bool(bool),
    NativeFunction(NativeFunction),
    ChonkFunction(ChonkFunction),
    Null,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Number(value) => write!(f, "{value}"),
            Value::String(value) => write!(f, "{value}"),
            Value::Bool(value) => write!(f, "{value}"),
            Value::NativeFunction(func) => write!(f, "{func}"),
            Value::ChonkFunction(func) => write!(f, "{func}"),
            Value::Null => write!(f, "null"),
        }
    }
}

impl Value {
    fn as_callable(&self) -> Option<&dyn Callable> {
        match self {
            Value::NativeFunction(func) => Some(func),
            Value::ChonkFunction(func) => Some(func),
            _ => None,
        }
    }
}

#[derive(Default, Clone)]
struct Environment {
    store: HashMap<String, Value>,
    outer: Option<Box<Environment>>,
}

impl Environment {
    /// Creates a new outer scope.
    pub fn new_outer(outer: Environment) -> Self {
        Self {
            outer: Some(Box::new(outer)),
            ..Default::default()
        }
    }

    /// Returns the value bound to the name.
    pub fn get(&self, name: &Token) -> Result<Value, RuntimeError> {
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
    pub fn set(&mut self, name: String, value: &Value) {
        self.store.insert(name, value.to_owned());
    }
}

#[derive(Clone)]
struct NativeFunction {
    name: String,
    arity: u8,
    callable: fn(&mut Interpreter, &[Value]) -> Result<Value, RuntimeError>,
}

impl fmt::Display for NativeFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<native function {}>", self.name)
    }
}

impl Callable for NativeFunction {
    fn arity(&self) -> u8 {
        self.arity
    }

    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: &[Value],
    ) -> Result<Value, RuntimeError> {
        (self.callable)(interpreter, arguments)
    }
}

#[derive(Clone)]
struct ChonkFunction {
    name: Token,
    params: Vec<Token>,
    body: Vec<Stmt>,
}

impl fmt::Display for ChonkFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<function {}>", self.name.lexeme)
    }
}

impl ChonkFunction {
    /// Creates a new `ChonkFunction`.
    fn new(name: Token, params: Vec<Token>, body: Vec<Stmt>) -> Self {
        Self { name, params, body }
    }
}

impl Callable for ChonkFunction {
    fn arity(&self) -> u8 {
        self.params.len().try_into().unwrap()
    }

    fn call(
        &self,
        interpreter: &mut Interpreter,
        arguments: &[Value],
    ) -> Result<Value, RuntimeError> {
        let mut environment = Environment::new_outer(interpreter.globals.clone());
        for (param, arg) in zip(&self.params, arguments) {
            environment.set(param.lexeme.clone(), arg);
        }

        interpreter.execute_new(&self.body, environment)?;
        Ok(Value::Null)
    }
}
