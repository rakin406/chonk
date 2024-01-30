use std::fmt;

#[derive(Debug, Clone)]
pub enum Literal {
    Number(f64),
    String(String),
    Bool(bool),
    Null,
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Literal::Number(value) => write!(f, "{value}"),
            Literal::String(value) => write!(f, "{value}"),
            Literal::Bool(value) => write!(f, "{value}"),
            Literal::Null => write!(f, "null"),
        }
    }
}
