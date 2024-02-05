use std::collections::HashMap;
use std::fmt;
use std::iter::zip;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::internal::ast::{Expr, Stmt};
use crate::internal::runtime_error::RuntimeError;
use crate::internal::token::{Literal, Token, TokenType};

#[allow(dead_code)]
pub struct Interpreter {
    is_interactive: bool,
    globals: Environment,
    environment: Environment,
    retval: Option<Value>,
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
            "clock",
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
            is_interactive: false,
            globals: globals.clone(),
            environment: globals.clone(),
            retval: None,
        }
    }
}

impl Interpreter {
    /// Creates a new `Interpreter`.
    pub fn new(is_interactive: bool) -> Self {
        Self {
            is_interactive,
            ..Default::default()
        }
    }

    /// Interprets a list of statements.
    pub fn interpret(&mut self, statements: &[Stmt]) -> Result<(), RuntimeError> {
        for stmt in statements {
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
        self.interpret(statements)?;
        self.environment = previous;

        Ok(())
    }

    /// Executes statement.
    fn execute(&mut self, stmt: &Stmt) -> Result<(), RuntimeError> {
        if self.retval.is_some() {
            return Ok(());
        }

        match stmt {
            Stmt::Function { name, params, body } => {
                let function = ChonkFunction {
                    // NOTE: Too many clones here!
                    name: name.clone(),
                    params: params.clone(),
                    body: body.clone(),
                    closure: self.environment.clone(),
                };
                self.environment
                    .set(&name.lexeme, &Value::ChonkFunction(function));
            }
            Stmt::While { test, body } => {
                while is_truthy(&self.interpret_expr(test)?) {
                    self.interpret(body)?;
                }
            }
            Stmt::If {
                test,
                body,
                or_else,
            } => {
                if is_truthy(&self.interpret_expr(test)?) {
                    self.interpret(body)?;
                } else if let Some(else_stmt) = or_else {
                    self.interpret(else_stmt)?;
                }
            }
            Stmt::Return(value) => {
                self.retval = Some(match value {
                    Some(expr) => self.interpret_expr(expr)?,
                    None => Value::Null,
                });
            }
            Stmt::Delete(targets) => {
                for target in targets {
                    self.environment.pop(target)?;
                }
            }
            Stmt::Expr(expr) => {
                let value = self.interpret_expr(expr)?;
                if self.is_interactive {
                    println!("{}", value);
                }
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
                self.environment.set(&name.lexeme, &value);
                Ok(value)
            }
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
            Expr::AugAssign {
                name,
                operator,
                value,
            } => {
                let target = self.environment.get(name)?;
                let result = self.interpret_aug_assign(&target, operator.clone(), value)?;
                self.environment.set(&name.lexeme, &result);
                Ok(result)
            }
            Expr::Prefix { operator, name } => {
                let target = self.environment.get(name)?;
                let value = self.interpret_prefix(operator.clone(), &target)?;
                self.environment.set(&name.lexeme, &value);
                Ok(value)
            }
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
            (Value::Number(n1), TokenType::Percent, Value::Number(n2)) => {
                Ok(Value::Number(n1 % n2))
            }
            (Value::Number(n1), TokenType::Slash, Value::Number(n2)) => Ok(Value::Number(n1 / n2)),
            (Value::Number(n1), TokenType::Star, Value::Number(n2)) => Ok(Value::Number(n1 * n2)),
            _ => Err(RuntimeError::new(op, "Invalid operands in binary operator")),
        }
    }

    fn interpret_unary(&mut self, op: Token, rhs: &Expr) -> Result<Value, RuntimeError> {
        let right = &self.interpret_expr(rhs)?;

        match (op.ty, right) {
            (TokenType::Plus, Value::Number(value)) => Ok(Value::Number(*value)),
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
        for arg in arguments {
            args.push(self.interpret_expr(arg)?);
        }

        let function = if let Some(func) = callee_value.as_callable() {
            func
        } else {
            return Err(RuntimeError::new(paren.clone(), "Can only call functions"));
        };

        if args.len() != function.arity().into() {
            return Err(RuntimeError::new(
                paren.clone(),
                &format!(
                    "Expected {} arguments but got {}",
                    function.arity(),
                    args.len()
                ),
            ));
        }

        function.call(self, &args)
    }

    fn interpret_aug_assign(
        &mut self,
        target: &Value,
        operator: Token,
        value: &Expr,
    ) -> Result<Value, RuntimeError> {
        let rhs = self.interpret_expr(value)?;

        match (target, operator.ty, rhs) {
            (Value::Number(n1), TokenType::MinusEqual, Value::Number(n2)) => {
                Ok(Value::Number(n1 - n2))
            }
            (Value::Number(n1), TokenType::PlusEqual, Value::Number(n2)) => {
                Ok(Value::Number(n1 + n2))
            }
            (Value::String(s1), TokenType::PlusEqual, Value::String(s2)) => {
                Ok(Value::String(s1.to_owned() + &s2))
            }
            (Value::Number(n1), TokenType::PercentEqual, Value::Number(n2)) => {
                Ok(Value::Number(n1 % n2))
            }
            (Value::Number(n1), TokenType::SlashEqual, Value::Number(n2)) => {
                Ok(Value::Number(n1 / n2))
            }
            (Value::Number(n1), TokenType::StarEqual, Value::Number(n2)) => {
                Ok(Value::Number(n1 * n2))
            }
            _ => Err(RuntimeError::new(
                operator,
                "Invalid value in assignment operator",
            )),
        }
    }

    fn interpret_prefix(&mut self, operator: Token, target: &Value) -> Result<Value, RuntimeError> {
        match (operator.ty, target) {
            (TokenType::DoubleMinus, Value::Number(n)) => Ok(Value::Number(n - 1.0)),
            (TokenType::DoublePlus, Value::Number(n)) => Ok(Value::Number(n + 1.0)),
            _ => Err(RuntimeError::new(
                operator,
                "Invalid value to prefix operator",
            )),
        }
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
    fn new_outer(outer: Environment) -> Self {
        Self {
            outer: Some(Box::new(outer)),
            ..Default::default()
        }
    }

    /// Returns the value bound to the name.
    fn get(&self, name: &Token) -> Result<Value, RuntimeError> {
        if let Some(value) = self.store.get(&name.lexeme) {
            return Ok(value.clone());
        }

        if let Some(outer_env) = &self.outer {
            return outer_env.get(name);
        }

        Err(RuntimeError::new(
            name.clone(),
            &format!("Undefined variable \"{}\"", name.lexeme),
        ))
    }

    /// Binds a new name to a value. If the name exists, it assigns a new value
    /// to it.
    fn set(&mut self, name: &str, value: &Value) {
        self.store.insert(name.to_string(), value.clone());
    }

    /// Removes a name-value pair.
    fn pop(&mut self, name: &Token) -> Result<(), RuntimeError> {
        if self.store.remove(&name.lexeme).is_none() {
            if let Some(ref mut outer_env) = self.outer {
                outer_env.pop(name)?;
            }

            return Err(RuntimeError::new(
                name.clone(),
                &format!("Undefined variable \"{}\"", name.lexeme),
            ));
        }

        Ok(())
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
    closure: Environment,
}

impl fmt::Display for ChonkFunction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<function {}>", self.name.lexeme)
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
        let mut environment = Environment::new_outer(self.closure.clone());
        for (param, arg) in zip(&self.params, arguments) {
            environment.set(&param.lexeme, arg);
        }

        let saved_retval = interpreter.retval.clone();
        interpreter.execute_new(&self.body, environment)?;
        let retval = interpreter.retval.clone();
        interpreter.retval = saved_retval;

        match retval {
            Some(value) => Ok(value),
            None => Ok(Value::Null),
        }
    }
}
