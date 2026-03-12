use core::fmt::{self, Display, Formatter};

use abyss_diagnostics::Span;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RawTokenKind {
    Comment,
    Whitespace,
    Newline,

    Ident,

    Integer,    // 123
    HexInteger, // 0xFF
    BinInteger, // 0b101
    Float,      // 1.0, 1.
    String,     // "hello"
    CString,    // c"hello"
    Char,       // 'A'

    Symbol,

    Eof,
}

#[derive(Debug, Clone, Copy)]
pub struct RawToken {
    pub kind: RawTokenKind,
    pub len: usize,
}

impl RawToken {
    pub fn new(kind: RawTokenKind, len: usize) -> Self {
        Self { kind, len }
    }
}

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

impl TokenKind {
    pub fn lookup_ident(ident: &str) -> TokenKind {
        match ident {
            "const" => TokenKind::Const,
            "struct" => TokenKind::Struct,
            "pub" => TokenKind::Pub,
            "ret" => TokenKind::Ret,
            "if" => TokenKind::If,
            "then" => TokenKind::Then,
            "else" => TokenKind::Else,
            "while" => TokenKind::While,
            "for" => TokenKind::For,
            "forever" => TokenKind::Forever,
            "out" => TokenKind::Out,
            "next" => TokenKind::Next,
            "in" => TokenKind::In,
            "and" => TokenKind::And,
            "or" => TokenKind::Or,
            "not" => TokenKind::Not,
            "as" => TokenKind::As,
            "is" => TokenKind::Is,
            "true" => TokenKind::True,
            "false" => TokenKind::False,
            "size" => TokenKind::Size,
            "mod" => TokenKind::Mod,
            "use" => TokenKind::Use,
            "match" => TokenKind::Match,
            "cmpt" => TokenKind::Cmpt,
            "def" => TokenKind::Def,

            _ => TokenKind::Ident,
        }
    }

    pub fn lookup_symbol(sym: &str) -> TokenKind {
        match sym {
            "+" => TokenKind::Plus,
            "-" => TokenKind::Minus,
            "*" => TokenKind::Star,
            "/" => TokenKind::Slash,
            "%" => TokenKind::Percent,
            "&" => TokenKind::Amp,
            "|" => TokenKind::Pipe,
            "^" => TokenKind::Caret,
            "<<" => TokenKind::LeftShift,
            ">>" => TokenKind::RightShift,
            "~" => TokenKind::Tilde,
            "," => TokenKind::Comma,
            ":" => TokenKind::Colon,
            "::" => TokenKind::ColonColon,
            ";" => TokenKind::Semi,
            "." => TokenKind::Dot,
            ".." => TokenKind::DotDot,
            "(" => TokenKind::OParen,
            ")" => TokenKind::CParen,
            "{" => TokenKind::OBrace,
            "}" => TokenKind::CBrace,
            "[" => TokenKind::OBracket,
            "]" => TokenKind::CBracket,
            "=" => TokenKind::Assign,
            ":=" => TokenKind::ColonEq,
            "+=" => TokenKind::PlusAssign,
            "-=" => TokenKind::MinusAssign,
            "*=" => TokenKind::StarAssign,
            "/=" => TokenKind::SlashAssign,
            "%=" => TokenKind::PercentAssign,
            "^=" => TokenKind::CaretAssign,
            "&=" => TokenKind::AmpAssign,
            "|=" => TokenKind::PipeAssign,
            ">>=" => TokenKind::RightShiftAssign,
            "<<=" => TokenKind::LeftShiftAssign,
            "==" => TokenKind::EqEq,
            "!=" => TokenKind::BangEq,
            "<" => TokenKind::Lt,
            "<=" => TokenKind::LtEq,
            ">" => TokenKind::Gt,
            ">=" => TokenKind::GtEq,
            "->" => TokenKind::RArrow,
            "=>" => TokenKind::REqArrow,
            "#" => TokenKind::Hash,
            "_" => TokenKind::Underscore,

            _ => TokenKind::Unknown,
        }
    }
}

impl Display for TokenKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            TokenKind::Comment => write!(f, "Comment"),
            TokenKind::Whitespace => write!(f, "Whitespace"),
            TokenKind::Newline => write!(f, "Newline"),
            TokenKind::Ident => write!(f, "Ident"),
            TokenKind::StrLit => write!(f, "String"),
            TokenKind::CStrLit => write!(f, "CString"),
            TokenKind::CharLit => write!(f, "Char"),
            TokenKind::HexIntLit => write!(f, "HexInteger"),
            TokenKind::BinIntLit => write!(f, "BinInteger"),
            TokenKind::FloatLit => write!(f, "Float"),
            TokenKind::IntLit => write!(f, "Integer"),
            TokenKind::Const => write!(f, "'const'"),
            TokenKind::Struct => write!(f, "'struct'"),
            TokenKind::Pub => write!(f, "'pub'"),
            TokenKind::Ret => write!(f, "'ret'"),
            TokenKind::If => write!(f, "'if'"),
            TokenKind::Then => write!(f, "'then'"),
            TokenKind::Else => write!(f, "'else'"),
            TokenKind::While => write!(f, "'while'"),
            TokenKind::For => write!(f, "'for'"),
            TokenKind::Forever => write!(f, "'forever'"),
            TokenKind::Out => write!(f, "'out'"),
            TokenKind::Next => write!(f, "'next'"),
            TokenKind::In => write!(f, "'in'"),
            TokenKind::As => write!(f, "'as'"),
            TokenKind::Is => write!(f, "'is'"),
            TokenKind::And => write!(f, "'and'"),
            TokenKind::Or => write!(f, "'or'"),
            TokenKind::Not => write!(f, "'not'"),
            TokenKind::True => write!(f, "'true'"),
            TokenKind::False => write!(f, "'false'"),
            TokenKind::Size => write!(f, "'size'"),
            TokenKind::Mod => write!(f, "'mod'"),
            TokenKind::Use => write!(f, "'use'"),
            TokenKind::Match => write!(f, "'match'"),
            TokenKind::Cmpt => write!(f, "'cmpt'"),
            TokenKind::Def => write!(f, "'def'"),
            TokenKind::Plus => write!(f, "'+'"),
            TokenKind::Minus => write!(f, "'-'"),
            TokenKind::Star => write!(f, "'*'"),
            TokenKind::Slash => write!(f, "'/'"),
            TokenKind::Percent => write!(f, "'%'"),
            TokenKind::Amp => write!(f, "'&'"),
            TokenKind::Pipe => write!(f, "'|'"),
            TokenKind::Caret => write!(f, "'^'"),
            TokenKind::LeftShift => write!(f, "'<<'"),
            TokenKind::RightShift => write!(f, "'>>'"),
            TokenKind::Tilde => write!(f, "'~'"),
            TokenKind::Comma => write!(f, "','"),
            TokenKind::Colon => write!(f, "':'"),
            TokenKind::ColonColon => write!(f, " '::'"),
            TokenKind::Semi => write!(f, "';'"),
            TokenKind::Dot => write!(f, "'.'"),
            TokenKind::DotDot => write!(f, "'..'"),
            TokenKind::OParen => write!(f, "'('"),
            TokenKind::CParen => write!(f, "')'"),
            TokenKind::OBrace => write!(f, "'{{'"),
            TokenKind::CBrace => write!(f, "'}}'"),
            TokenKind::OBracket => write!(f, "'['"),
            TokenKind::CBracket => write!(f, "']'"),
            TokenKind::Assign => write!(f, "'='"),
            TokenKind::ColonEq => write!(f, "':='"),
            TokenKind::PlusAssign => write!(f, "'+='"),
            TokenKind::MinusAssign => write!(f, "'-='"),
            TokenKind::StarAssign => write!(f, "'*='"),
            TokenKind::SlashAssign => write!(f, "'/='"),
            TokenKind::PercentAssign => write!(f, "'%='"),
            TokenKind::CaretAssign => write!(f, "'^='"),
            TokenKind::AmpAssign => write!(f, "'&='"),
            TokenKind::PipeAssign => write!(f, "'|='"),
            TokenKind::RightShiftAssign => write!(f, "'>>='"),
            TokenKind::LeftShiftAssign => write!(f, "'<<='"),
            TokenKind::EqEq => write!(f, "'=='"),
            TokenKind::BangEq => write!(f, "'!='"),
            TokenKind::Lt => write!(f, "'<'"),
            TokenKind::LtEq => write!(f, "'<='"),
            TokenKind::Gt => write!(f, "'>'"),
            TokenKind::GtEq => write!(f, "'>='"),
            TokenKind::RArrow => write!(f, "'->'"),
            TokenKind::REqArrow => write!(f, "'=>'"),
            TokenKind::Hash => write!(f, "'#'"),
            TokenKind::Underscore => write!(f, "'_'"),
            TokenKind::Unknown => write!(f, "Unknown"),
            TokenKind::Eof => write!(f, "Eof"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Token<'a> {
    pub kind: TokenKind,
    pub text: &'a str,
    pub start: usize,
    pub len: usize,
    pub preceded_by_newline: bool,
}

impl<'a> Token<'a> {
    pub fn new(
        kind: TokenKind,
        text: &'a str,
        start: usize,
        len: usize,
        preceded_by_newline: bool,
    ) -> Self {
        Self {
            kind,
            text,
            start,
            len,
            preceded_by_newline,
        }
    }

    pub fn end(&self) -> usize {
        self.start + self.len
    }

    pub fn dummy() -> Self {
        Self::new(TokenKind::Unknown, "", 0, 0, false)
    }

    pub fn span(&self, file_id: u16) -> Span {
        Span {
            file_id,
            start: self.start as u32,
            end: (self.start + self.len) as u32,
        }
    }
}
