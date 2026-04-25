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
    #[inline]
    pub fn new(source: &'a str) -> Self {
        Self {
            source,
            cursor: Cursor::new(source),
            had_newline: true,
            finished: false,
        }
    }

    #[inline]
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
                Self::lookup_ident_fast(text)
            } else {
                self.scan_symbol_fast()
            };

            let end_offset = self.cursor.len_consumed();
            let len = end_offset - start_offset;
            let text = &self.source[start_offset..end_offset];

            let token = Token::new(kind, text, start_offset, len, self.had_newline);
            self.had_newline = false;

            return token;
        }
    }

    #[inline]
    fn is_simple_whitespace(c: char) -> bool {
        matches!(c, ' ' | '\t')
    }

    #[inline]
    fn is_newline(c: char) -> bool {
        matches!(c, '\n' | '\r')
    }

    #[inline]
    fn is_ident_start(c: char) -> bool {
        c.is_ascii_alphabetic() || c == '_'
    }

    #[inline]
    fn is_ident_continue(c: char) -> bool {
        c.is_ascii_alphanumeric() || c == '_'
    }

    #[inline]
    fn is_digit(c: char) -> bool {
        c.is_ascii_digit()
    }

    #[inline]
    fn consume_newlines(&mut self) {
        self.cursor.eat_while(Self::is_newline);
    }

    #[inline]
    fn consume_simple_whitespace(&mut self) {
        self.cursor.eat_while(Self::is_simple_whitespace);
    }

    #[inline]
    fn scan_identifier(&mut self) {
        self.cursor.eat_while(Self::is_ident_continue);
    }

    #[inline]
    fn scan_comment(&mut self) {
        self.cursor.bump();
        self.cursor.bump();
        self.cursor.eat_while(|c| c != '\n' && c != '\r');
    }

    #[inline]
    fn scan_c_string(&mut self) -> TokenKind {
        self.cursor.bump();
        self.cursor.bump();
        self.consume_string_content();
        TokenKind::CStrLit
    }

    #[inline]
    fn scan_string(&mut self) -> TokenKind {
        self.cursor.bump();
        self.consume_string_content();
        TokenKind::StrLit
    }

    #[inline]
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

    #[inline]
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

    #[inline]
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

    #[inline]
    fn scan_symbol_fast(&mut self) -> TokenKind {
        let c0 = self.cursor.first();
        let c1 = self.cursor.second();

        match c0 {
            '+' => {
                self.cursor.bump();
                if self.cursor.first() == '=' {
                    self.cursor.bump();
                    TokenKind::PlusAssign
                } else {
                    TokenKind::Plus
                }
            }

            '-' => {
                self.cursor.bump();
                if self.cursor.first() == '=' {
                    self.cursor.bump();
                    TokenKind::MinusAssign
                } else if self.cursor.first() == '>' {
                    self.cursor.bump();
                    TokenKind::RArrow
                } else {
                    TokenKind::Minus
                }
            }

            '*' => {
                self.cursor.bump();
                if self.cursor.first() == '=' {
                    self.cursor.bump();
                    TokenKind::StarAssign
                } else {
                    TokenKind::Star
                }
            }

            '/' => {
                self.cursor.bump();
                if self.cursor.first() == '=' {
                    self.cursor.bump();
                    TokenKind::SlashAssign
                } else {
                    TokenKind::Slash
                }
            }

            '%' => {
                self.cursor.bump();
                if self.cursor.first() == '=' {
                    self.cursor.bump();
                    TokenKind::PercentAssign
                } else {
                    TokenKind::Percent
                }
            }

            '&' => {
                self.cursor.bump();
                if self.cursor.first() == '=' {
                    self.cursor.bump();
                    TokenKind::AmpAssign
                } else {
                    TokenKind::Amp
                }
            }

            '|' => {
                self.cursor.bump();
                if self.cursor.first() == '=' {
                    self.cursor.bump();
                    TokenKind::PipeAssign
                } else {
                    TokenKind::Pipe
                }
            }

            '^' => {
                self.cursor.bump();
                if self.cursor.first() == '=' {
                    self.cursor.bump();
                    TokenKind::CaretAssign
                } else {
                    TokenKind::Caret
                }
            }

            '~' => {
                self.cursor.bump();
                TokenKind::Tilde
            }

            ',' => {
                self.cursor.bump();
                TokenKind::Comma
            }

            ':' => {
                self.cursor.bump();
                if self.cursor.first() == ':' {
                    self.cursor.bump();
                    TokenKind::ColonColon
                } else if self.cursor.first() == '=' {
                    self.cursor.bump();
                    TokenKind::ColonEq
                } else {
                    TokenKind::Colon
                }
            }

            ';' => {
                self.cursor.bump();
                TokenKind::Semi
            }

            '.' => {
                self.cursor.bump();
                if self.cursor.first() == '.' {
                    self.cursor.bump();
                    TokenKind::DotDot
                } else {
                    TokenKind::Dot
                }
            }

            '(' => {
                self.cursor.bump();
                TokenKind::OParen
            }

            ')' => {
                self.cursor.bump();
                TokenKind::CParen
            }

            '{' => {
                self.cursor.bump();
                TokenKind::OBrace
            }

            '}' => {
                self.cursor.bump();
                TokenKind::CBrace
            }

            '[' => {
                self.cursor.bump();
                TokenKind::OBracket
            }

            ']' => {
                self.cursor.bump();
                TokenKind::CBracket
            }

            '=' => {
                self.cursor.bump();
                if self.cursor.first() == '=' {
                    self.cursor.bump();
                    TokenKind::EqEq
                } else if self.cursor.first() == '>' {
                    self.cursor.bump();
                    TokenKind::REqArrow
                } else {
                    TokenKind::Assign
                }
            }

            '!' => {
                self.cursor.bump();
                if self.cursor.first() == '=' {
                    self.cursor.bump();
                    TokenKind::BangEq
                } else {
                    TokenKind::Unknown
                }
            }

            '<' => {
                if c1 == '<' {
                    self.cursor.bump();
                    self.cursor.bump();
                    if self.cursor.first() == '=' {
                        self.cursor.bump();
                        TokenKind::LeftShiftAssign
                    } else {
                        TokenKind::LeftShift
                    }
                } else {
                    self.cursor.bump();
                    if self.cursor.first() == '=' {
                        self.cursor.bump();
                        TokenKind::LtEq
                    } else {
                        TokenKind::Lt
                    }
                }
            }

            '>' => {
                if c1 == '>' {
                    self.cursor.bump();
                    self.cursor.bump();
                    if self.cursor.first() == '=' {
                        self.cursor.bump();
                        TokenKind::RightShiftAssign
                    } else {
                        TokenKind::RightShift
                    }
                } else {
                    self.cursor.bump();
                    if self.cursor.first() == '=' {
                        self.cursor.bump();
                        TokenKind::GtEq
                    } else {
                        TokenKind::Gt
                    }
                }
            }

            '#' => {
                self.cursor.bump();
                TokenKind::Hash
            }

            _ => {
                self.cursor.bump();
                TokenKind::Unknown
            }
        }
    }

    #[inline]
    fn lookup_ident_fast(ident: &str) -> TokenKind {
        let b = ident.as_bytes();

        match b.len() {
            2 => match b {
                b"if" => TokenKind::If,
                b"in" => TokenKind::In,
                b"is" => TokenKind::Is,
                b"as" => TokenKind::As,
                b"or" => TokenKind::Or,
                _ => TokenKind::Ident,
            },

            3 => match b {
                b"ret" => TokenKind::Ret,
                b"out" => TokenKind::Out,
                b"and" => TokenKind::And,
                b"not" => TokenKind::Not,
                b"mod" => TokenKind::Mod,
                b"use" => TokenKind::Use,
                b"def" => TokenKind::Def,
                b"for" => TokenKind::For,
                b"pub" => TokenKind::Pub,
                _ => TokenKind::Ident,
            },

            4 => match b {
                b"then" => TokenKind::Then,
                b"else" => TokenKind::Else,
                b"next" => TokenKind::Next,
                b"true" => TokenKind::True,
                b"size" => TokenKind::Size,
                b"cmpt" => TokenKind::Cmpt,
                _ => TokenKind::Ident,
            },

            5 => match b {
                b"const" => TokenKind::Const,
                b"false" => TokenKind::False,
                b"match" => TokenKind::Match,
                b"while" => TokenKind::While,
                _ => TokenKind::Ident,
            },

            6 => match b {
                b"struct" => TokenKind::Struct,
                _ => TokenKind::Ident,
            },

            7 => match b {
                b"forever" => TokenKind::Forever,
                _ => TokenKind::Ident,
            },

            _ => TokenKind::Ident,
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = Token<'a>;

    #[inline]
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
