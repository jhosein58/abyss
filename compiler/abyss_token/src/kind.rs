#[repr(u8)]
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum TokenKind {
    Comment,
    Whitespace,
    Newline,

    Ident,

    // Literals
    IntLit,    // 123
    HexIntLit, // 0xFF
    BinIntLit, // 0b101
    OctIntLit, // 0o777
    FloatLit,  // 1.0
    StrLit,    // "..."
    CStrLit,   // c"..."
    CharLit,   // 'A'

    // --- Keywords ---
    Const,            // const
    Struct,           // struct
    Pub,              // pub
    Ret,              // ret
    If,               // if
    Then,             // then
    Else,             // else
    While,            // while
    For,              // for
    Forever,          // forever
    Out,              // out
    Next,             // next
    In,               // in
    As,               // as
    Is,               // is
    And,              // and
    Or,               // or
    Not,              // not
    Size,             // size
    Mod,              // mod
    Use,              // use
    True,             // true
    False,            // false
    Match,            // match
    Cmpt,             // cmpt
    Def,              // def
    Plus,             // +
    Minus,            // -
    Star,             // *
    Slash,            // /
    Percent,          // %
    Amp,              // &
    Pipe,             // |
    Caret,            // ^
    LeftShift,        // <<
    RightShift,       // >>
    Tilde,            // ~
    Comma,            // ,
    Colon,            // :
    ColonColon,       // ::
    Semi,             // ;
    Dot,              // .
    DotDot,           // ..
    OParen,           // (
    CParen,           // )
    OBrace,           // {
    CBrace,           // }
    OBracket,         // [
    CBracket,         // ]
    Assign,           // =
    ColonEq,          // :=
    PlusAssign,       // +=
    MinusAssign,      // -=
    StarAssign,       // *=
    SlashAssign,      // /=
    PercentAssign,    // %=
    AmpAssign,        // &=
    CaretAssign,      // ^=
    PipeAssign,       // |=
    RightShiftAssign, // >>=
    LeftShiftAssign,  // <<=
    EqEq,             // ==
    BangEq,           // !=
    Lt,               // <
    LtEq,             // <=
    Gt,               // >
    GtEq,             // >=
    RArrow,           // ->
    REqArrow,         // =>
    Hash,             // #
    Underscore,       // _

    Unknown,
    Eof,
}
