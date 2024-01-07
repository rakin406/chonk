enum TokenType {
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
    NotEqualTo,
    GreaterThan,
    LessThan,
    GreaterThanOrEqualTo,
    LessThanOrEqualTo,

    // Literals
    Identifier,
    String,
    Number,

    // Keywords
    And,
    Or,
    Not,
}
