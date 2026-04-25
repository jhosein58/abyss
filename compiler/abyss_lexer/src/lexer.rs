use crate::{
    cursor::Cursor,
    token::{Token, TokenKind},
};

pub struct Lexer<'a> {
    source: &'a str,
    cursor: Cursor<'a>,
    had_newline: bool,
    finished: bool,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            cursor: Cursor::new(source),
            had_newline: true,
            finished: false,
        }
    }

    pub fn next_token(&mut self) -> Token<'a> {
        if self.finished {
            return Token::new(
                TokenKind::Eof,
                "",
                self.cursor.len_consumed(),
                0,
                self.had_newline,
            );
        }

        loop {
            if self.cursor.is_eof() {
                self.finished = true;
                return Token::new(
                    TokenKind::Eof,
                    "",
                    self.cursor.len_consumed(),
                    0,
                    self.had_newline,
                );
            }

            let start_offset = self.cursor.len_consumed();
            let c = self.cursor.first();

            if Self::is_newline(c) {
                self.consume_newlines();
                self.had_newline = true;
                continue;
            }

            if Self::is_simple_whitespace(c) {
                self.consume_simple_whitespace();
                continue;
            }

            if c == '-' && self.cursor.second() == '-' {
                self.scan_comment();
                continue;
            }

            let kind = if c == 'c' && self.cursor.second() == '"' {
                self.scan_c_string()
            } else if Self::is_digit(c) || (c == '.' && Self::is_digit(self.cursor.second())) {
                self.scan_number()
            } else if c == '"' {
                self.scan_string()
            } else if c == '\'' {
                self.scan_char()
            } else if Self::is_ident_start(c) {
                self.scan_identifier();
                let end_offset = self.cursor.len_consumed();
                let text = &self.source[start_offset..end_offset];
                TokenKind::lookup_ident(text)
            } else {
                let max_len = 3.min(self.source.len() - start_offset);
                let mut matched_len = 0;
                let mut matched_kind = TokenKind::Unknown;

                for i in (1..=max_len).rev() {
                    if self.source.is_char_boundary(start_offset + i) {
                        let text = &self.source[start_offset..start_offset + i];
                        let kind = TokenKind::lookup_symbol(text);
                        if kind != TokenKind::Unknown {
                            matched_len = i;
                            matched_kind = kind;
                            break;
                        }
                    }
                }

                if matched_len > 0 {
                    for _ in 0..matched_len {
                        self.cursor.bump();
                    }
                    matched_kind
                } else {
                    self.cursor.bump();
                    TokenKind::Unknown
                }
            };

            let end_offset = self.cursor.len_consumed();
            let len = end_offset - start_offset;
            let text = &self.source[start_offset..end_offset];

            let token = Token::new(kind, text, start_offset, len, self.had_newline);
            self.had_newline = false;

            return token;
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

    fn scan_c_string(&mut self) -> TokenKind {
        self.cursor.bump();
        self.cursor.bump();
        self.consume_string_content();
        TokenKind::CStrLit
    }

    fn scan_string(&mut self) -> TokenKind {
        self.cursor.bump();
        self.consume_string_content();
        TokenKind::StrLit
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

    fn scan_char(&mut self) -> TokenKind {
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
        TokenKind::CharLit
    }

    fn scan_number(&mut self) -> TokenKind {
        let mut is_float = false;
        let first = self.cursor.first();

        if first == '0' {
            let second = self.cursor.second();

            if second == 'x' || second == 'X' {
                self.cursor.bump();
                self.cursor.bump();
                self.cursor.eat_while(|c| c.is_ascii_hexdigit() || c == '_');
                return TokenKind::HexIntLit;
            } else if second == 'b' || second == 'B' {
                self.cursor.bump();
                self.cursor.bump();
                self.cursor.eat_while(|c| matches!(c, '0'..='1' | '_'));
                return TokenKind::BinIntLit;
            } else if second == 'o' || second == 'O' {
                self.cursor.bump();
                self.cursor.bump();
                self.cursor.eat_while(|c| matches!(c, '0'..='7' | '_'));
                return TokenKind::OctIntLit;
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
                return TokenKind::IntLit;
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
            TokenKind::FloatLit
        } else {
            TokenKind::IntLit
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
