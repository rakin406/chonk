/// Token types
#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum TokenType {
    Eof,
    Identifier,
    Number,
    String,
    Newline,

    LParen,          // '('
    RParen,          // ')'
    LBracket,        // '['
    RBracket,        // ']'
    LBrace,          // '{'
    RBrace,          // '}'
    Comma,           // ','
    Plus,            // '+'
    Minus,           // '-'
    Star,            // '*'
    Slash,           // '/'
    VBar,            // '|'
    Amper,           // '&'
    Less,            // '<'
    Greater,         // '>'
    Equal,           // '='
    Dot,             // '.'
    Percent,         // '%'
    Bang,            // '!'
    EqEqual,         // '=='
    BangEqual,       // '!='
    LessEqual,       // '<='
    GreaterEqual,    // '>='
    DoubleVBar,      // '||'
    DoubleAmper,     // '&&'
    Tilde,           // '~'
    Caret,           // '^'
    LeftShift,       // '<<'
    RightShift,      // '>>'
    DoubleStar,      // '**'
    PlusEqual,       // '+='
    MinEqual,        // '-='
    StarEqual,       // '*='
    SlashEqual,      // '/='
    PercentEqual,    // '%='
    AmperEqual,      // '&='
    VBarEqual,       // '|='
    CaretEqual,      // '^='
    LeftShiftEqual,  // '<<='
    RightShiftEqual, // '>>='
    DoubleStarEqual, // '**='
    DoublePlus,      // '++'
    DoubleMinus,     // '--'

    Null,
    True,
    False,
    If,
    Elif,
    Else,
    Case,
    Default,
    // In,
    // NotIn,
    While,
    For,
    Break,
    Continue,
    Return,
    Echo,
}

/// Returns `true` if the token type is `Eof`.
pub fn is_eof(token_type: TokenType) -> bool {
    token_type == TokenType::Eof
}
