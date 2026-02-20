use crate::{
    scanner::Scanner,
    token::{RawTokenKind, Token, TokenKind},
};

pub struct Lexer<'a> {
    source: &'a str,
    scanner: Scanner<'a>,
    offset: usize,
    had_newline: bool,
    finished: bool,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            scanner: Scanner::new(source),
            offset: 0,
            had_newline: true,
            finished: false,
        }
    }

    pub fn next_token(&mut self) -> Token<'a> {
        if self.finished {
            return Token::new(TokenKind::Eof, "", self.offset, 0, self.had_newline);
        }

        loop {
            let raw = self.scanner.next_raw();
            let len = raw.len;

            let start_offset = self.offset;
            let end_offset = start_offset + len;

            let text = if end_offset <= self.source.len() {
                &self.source[start_offset..end_offset]
            } else {
                ""
            };

            if raw.kind == RawTokenKind::Newline {
                self.offset += len;
                self.had_newline = true;
                continue;
            }

            if raw.kind == RawTokenKind::Whitespace || raw.kind == RawTokenKind::Comment {
                self.offset += len;
                continue;
            }

            let kind = match raw.kind {
                RawTokenKind::Eof => TokenKind::Eof,
                RawTokenKind::Ident => TokenKind::lookup_ident(text),
                RawTokenKind::Integer => TokenKind::IntLit,
                RawTokenKind::HexInteger => TokenKind::HexIntLit,
                RawTokenKind::BinInteger => TokenKind::BinIntLit,
                RawTokenKind::Float => TokenKind::FloatLit,
                RawTokenKind::String => TokenKind::StrLit,
                RawTokenKind::CString => TokenKind::CStrLit,
                RawTokenKind::Char => TokenKind::CharLit,
                RawTokenKind::Symbol => TokenKind::lookup_symbol(text),
                _ => TokenKind::Unknown,
            };

            let token = Token::new(kind, text, start_offset, len, self.had_newline);

            self.offset += len;
            self.had_newline = false;

            return token;
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }

        let token = self.next_token();

        if token.kind == TokenKind::Eof {
            self.finished = true;
            return Some(token);
        }

        Some(token)
    }
}
