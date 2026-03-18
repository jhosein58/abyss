use crate::{
    cursor::Cursor,
    token::{RawToken, RawTokenKind},
};

#[derive(Clone)]
pub struct Scanner<'a> {
    pub cursor: Cursor<'a>,
}

impl<'a> Scanner<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            cursor: Cursor::new(input),
        }
    }

    fn is_simple_whitespace(c: char) -> bool {
        matches!(c, ' ' | '\t')
    }

    fn is_newline(c: char) -> bool {
        matches!(c, '\n' | '\r')
    }

    fn is_ident_start(c: char) -> bool {
        c.is_ascii_alphabetic() || c == '_'
    }

    fn is_ident_continue(c: char) -> bool {
        c.is_ascii_alphanumeric() || c == '_'
    }

    fn is_digit(c: char) -> bool {
        c.is_ascii_digit()
    }

    pub fn next_raw(&mut self) -> RawToken {
        if self.cursor.is_eof() {
            return RawToken::new(RawTokenKind::Eof, 0);
        }

        let start_pos = self.cursor.len_consumed();
        let c = self.cursor.first();

        let kind = if Self::is_newline(c) {
            self.consume_newlines();
            RawTokenKind::Newline
        } else if Self::is_simple_whitespace(c) {
            self.consume_simple_whitespace();
            RawTokenKind::Whitespace
        } else if c == 'c' && self.cursor.second() == '"' {
            self.scan_c_string()
        } else if Self::is_digit(c) || (c == '.' && Self::is_digit(self.cursor.second())) {
            self.scan_number()
        } else if c == '"' {
            self.scan_string()
        } else if c == '\'' {
            self.scan_char()
        } else if Self::is_ident_start(c) {
            self.scan_identifier();
            RawTokenKind::Ident
        } else if c == '-' && self.cursor.second() == '-' {
            self.scan_comment();
            RawTokenKind::Comment
        } else {
            self.cursor.bump();
            RawTokenKind::Symbol
        };

        let end_pos = self.cursor.len_consumed();
        RawToken::new(kind, end_pos - start_pos)
    }

    fn consume_newlines(&mut self) {
        self.cursor.eat_while(Self::is_newline);
    }

    fn consume_simple_whitespace(&mut self) {
        self.cursor.eat_while(Self::is_simple_whitespace);
    }

    fn scan_identifier(&mut self) {
        self.cursor.eat_while(Self::is_ident_continue);
    }

    fn scan_comment(&mut self) {
        self.cursor.bump();
        self.cursor.bump();
        self.cursor.eat_while(|c| c != '\n' && c != '\r');
    }

    fn scan_c_string(&mut self) -> RawTokenKind {
        self.cursor.bump();
        self.cursor.bump();
        self.consume_string_content();
        RawTokenKind::CString
    }

    fn scan_string(&mut self) -> RawTokenKind {
        self.cursor.bump();
        self.consume_string_content();
        RawTokenKind::String
    }

    fn consume_string_content(&mut self) {
        while !self.cursor.is_eof() {
            let c = self.cursor.first();
            if c == '"' {
                break;
            }
            if c == '\\' {
                self.cursor.bump();
            }
            self.cursor.bump();
        }
        if !self.cursor.is_eof() {
            self.cursor.bump();
        }
    }

    fn scan_char(&mut self) -> RawTokenKind {
        self.cursor.bump();
        if self.cursor.first() == '\\' {
            self.cursor.bump();
            self.cursor.bump();
        } else {
            self.cursor.bump();
        }
        if self.cursor.first() == '\'' {
            self.cursor.bump();
        }
        RawTokenKind::Char
    }

    fn scan_number(&mut self) -> RawTokenKind {
        let mut is_float = false;
        let first = self.cursor.first();

        if first == '0' {
            let second = self.cursor.second();

            if second == 'x' || second == 'X' {
                self.cursor.bump();
                self.cursor.bump();
                self.cursor.eat_while(|c| c.is_ascii_hexdigit() || c == '_');
                return RawTokenKind::HexInteger;
            } else if second == 'b' || second == 'B' {
                self.cursor.bump();
                self.cursor.bump();
                self.cursor.eat_while(|c| matches!(c, '0'..='1' | '_'));
                return RawTokenKind::BinInteger;
            } else if second == 'o' || second == 'O' {
                self.cursor.bump();
                self.cursor.bump();
                self.cursor.eat_while(|c| matches!(c, '0'..='7' | '_'));
                return RawTokenKind::OctInteger;
            }
        }

        if first != '.' {
            self.cursor.eat_while(|c| c.is_ascii_digit() || c == '_');
        } else {
            is_float = true;
        }

        if self.cursor.first() == '.' && self.cursor.second() != '.' {
            let second = self.cursor.second();

            if Self::is_ident_start(second) && second != 'e' && second != 'E' {
                return RawTokenKind::Integer;
            }

            is_float = true;
            self.cursor.bump();
            self.cursor.eat_while(|c| c.is_ascii_digit() || c == '_');
        }

        let current = self.cursor.first();
        if current == 'e' || current == 'E' {
            is_float = true;
            self.cursor.bump();

            let next = self.cursor.first();
            if next == '+' || next == '-' {
                self.cursor.bump();
            }

            self.cursor.eat_while(|c| c.is_ascii_digit() || c == '_');
        }

        if is_float {
            RawTokenKind::Float
        } else {
            RawTokenKind::Integer
        }
    }

    pub fn peek_raw(&self) -> RawToken {
        let mut cloned_scanner = self.clone();
        cloned_scanner.next_raw()
    }
}

impl<'a> Iterator for Scanner<'a> {
    type Item = RawToken;

    fn next(&mut self) -> Option<Self::Item> {
        let token = self.next_raw();

        if let RawTokenKind::Eof = token.kind {
            None
        } else {
            Some(token)
        }
    }
}
