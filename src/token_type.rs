/// All the token types in Chonk language.
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
    Minus,
    Plus,
    Slash,
    Asterisk,
    Percent,

    // One or two character tokens
    Equal,
    EqualTo,
    Not,
    NotEqualTo,
    GreaterThan,
    LessThan,
    GreaterThanOrEqualTo,
    LessThanOrEqualTo,

    // Membership operators
    In,
    NotIn,

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
    And,
    Or,
    While,
    For,
    Break,
    Continue,
    Return,
    Echo,

    EOF,
}
