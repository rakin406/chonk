/// All the token types in `Chonk` language.
#[derive(Eq, PartialEq, Debug, Copy, Clone)]
pub enum TokenType {
    Eof,
    Identifier,
    Number,
    String,
    Newline,

    LeftParen,       // '('
    RightParen,      // ')'
    LeftBracket,     // '['
    RightBracket,    // ']'
    LeftBrace,       // '{'
    RightBrace,      // '}'
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
