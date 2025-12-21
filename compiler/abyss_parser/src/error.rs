use std::fmt::{self, Display, Formatter};

use abyss_lexer::token::TokenKind;

use crate::source_map::Span;

pub struct ParseError {
    pub kind: ParseErrorKind,
    pub message: String,
    pub pos: Span,
}

pub enum ParseErrorKind {
    UnexpectedToken {
        expected: TokenKind,
        found: TokenKind,
    },
    MalformedLiteral(&'static str),
    UnexpectedEof,
    NotAFunction,
    Expected(String),
    Message(String),
}

impl Display for ParseErrorKind {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ParseErrorKind::UnexpectedToken { expected, found } => {
                write!(
                    f,
                    "unexpected token: found `{}`, expected {}",
                    found, expected
                )
            }
            ParseErrorKind::MalformedLiteral(reason) => {
                write!(f, "malformed literal: {}", reason)
            }
            ParseErrorKind::UnexpectedEof => {
                write!(f, "unexpected end of file")
            }
            ParseErrorKind::NotAFunction => {
                write!(f, "not a function")
            }
            ParseErrorKind::Expected(expected) => {
                write!(f, "expected {}", expected)
            }
            ParseErrorKind::Message(message) => {
                write!(f, "{}", message)
            }
        }
    }
}
