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
    Plus,
    PlusPlus,
    Minus,
    MinusMinus,
    Asterisk,
    Slash,
    Percent,

    // Assignment operators
    Equal,
    PlusEqual,
    MinusEqual,
    AsteriskEqual,
    SlashEqual,
    PercentEqual,

    // Comparison operators
    EqualTo,
    NotEqualTo,
    GreaterThan,
    LessThan,
    GreaterThanOrEqualTo,
    LessThanOrEqualTo,

    // Logical operators
    And,
    Or,
    Not,

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

    EOF,
}
