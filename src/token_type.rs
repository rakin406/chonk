/// All the token types in `Chonk` language.
#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum TokenType {
    // Single-character tokens
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    LeftBrace,
    RightBrace,
    Comma,
    Dot,

    // Arithmetic operators
    Add,
    Sub,
    Mult,
    Div,
    Mod,
    Power,
    Increment,
    Decrement,

    // Assignment operators
    Equal,
    AddEqual,
    SubEqual,
    MultEqual,
    DivEqual,
    ModEqual,
    // TODO: Add bitwise assignment operators and power operator.

    // Comparison operators
    EqualTo,
    NotEqualTo,
    Greater,
    Less,
    GreaterEqual,
    LessEqual,

    // Logical operators
    And,
    Or,
    Not,

    // Bitwise operators
    BitAnd,
    BitOr,
    BitXor,
    BitNot,
    LeftShift,
    RightShift,

    // Literals
    Identifier,
    String,
    Number,

    // Keywords
    Null,
    True,
    False,
    If,
    Elif,
    Else,
    Case,
    Default,
    In,
    NotIn,
    While,
    For,
    Break,
    Continue,
    Return,
    Echo,

    Eof,
}
