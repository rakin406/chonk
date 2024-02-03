/// Token types
#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum TokenType {
    // Special token
    Eof,

    // Identifiers and literals
    Ident,  // variable
    Number, // 123
    String, // "Hello World"

    // Operators
    Plus,    // +
    Minus,   // -
    Star,    // *
    Slash,   // /
    Percent, // %

    PlusEqual,    // +=
    MinusEqual,   // -=
    StarEqual,    // *=
    SlashEqual,   // /=
    PercentEqual, // %=

    DoubleAmper, // &&
    DoubleVBar,  // ||
    DoublePlus,  // ++
    DoubleMinus, // --

    EqEqual, // ==
    Less,    // <
    Greater, // >
    Equal,   // =
    Bang,    // !

    BangEqual,    // !=
    LessEqual,    // <=
    GreaterEqual, // >=

    LParen,    // (
    RParen,    // )
    LBrace,    // {
    RBrace,    // }
    Comma,     // ,
    Semicolon, // ;

    // Keywords
    Null,
    True,
    False,
    Func,
    If,
    Else,
    While,
    Break,
    Continue,
    Return,
    Delete,
    Echo,
}

/// Returns `true` if the token type is `Eof`.
pub fn is_eof(ty: TokenType) -> bool {
    ty == TokenType::Eof
}
