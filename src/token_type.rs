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
    Exponent,

    // Assignment operators
    Equal,
    PlusEqual,
    MinusEqual,
    AsteriskEqual,
    SlashEqual,
    PercentEqual,
    // TODO: Add bitwise assignment operators.

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

    // Bitwise operators
    // NOTE: I was thinking about changing these names to And, Or etc. but there
    // are collisions with the logical operators, unfortunately :(
    Ampersand,   // AND
    VerticalBar, // OR
    Caret,       // XOR
    Tilde,       // NOT
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
