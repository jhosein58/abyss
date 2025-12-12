use abyss_lexer::token::TokenKind;

use crate::{
    ast::{Function, Type},
    error::ParseErrorKind,
    parser::Parser,
};

impl<'a> Parser<'a> {
    fn synchronize_func(&mut self) {
        self.stream.advance();

        while !self.stream.is_at_end() {
            match self.stream.current().kind {
                TokenKind::Fn => {
                    return;
                }

                _ => {}
            }

            self.stream.advance();
        }
    }

    fn consume_func(&mut self, expected: TokenKind) -> Option<()> {
        if !self.stream.consume(expected) {
            self.emit_error_at_current(ParseErrorKind::UnexpectedToken {
                expected: expected,
                found: self.stream.current().kind,
            });
            self.synchronize_func();
            return None;
        }
        Some(())
    }

    fn expect(&mut self, expected: TokenKind) -> Option<()> {
        if !self.stream.is(expected) {
            self.emit_error_at_current(ParseErrorKind::UnexpectedToken {
                expected: expected,
                found: self.stream.current().kind,
            });
            self.synchronize_func();
            return None;
        }
        Some(())
    }

    fn read_ident(&mut self) -> Option<String> {
        self.expect(TokenKind::Ident)?;
        let ident = self.stream.current_lit().to_string();

        self.advance();
        Some(ident)
    }

    pub fn parse_function(&mut self) -> Option<Function> {
        self.stream.consume(TokenKind::Newline);

        if self.stream.is_peek(TokenKind::Eof) {
            return None;
        }

        self.consume_func(TokenKind::Fn)?;

        let name = self.read_ident()?;

        let params = self.parse_func_params()?;

        let return_type = self.parse_return_type();
        if self.stream.is(TokenKind::Semi) {
            self.stream.advance();
            return Some(Function {
                name,
                params,
                return_type,
                body: None,
            });
        }
        let body = if let Some(block) = self.parse_block() {
            block
        } else {
            self.synchronize_func();
            return None;
        };

        Some(Function {
            name,
            params,
            return_type,
            body: Some(body),
        })
    }

    fn parse_func_params(&mut self) -> Option<Vec<(String, Type)>> {
        let mut params = Vec::new();

        if !self.stream.is(TokenKind::OParen) {
            return Some(params);
        }

        self.advance();

        if self.stream.is(TokenKind::CParen) {
            self.advance();
            return Some(params);
        }

        loop {
            let name = self.read_ident()?;

            self.consume_func(TokenKind::Colon)?;
            let ty = self.parse_type()?;

            params.push((name, ty));

            if self.stream.is(TokenKind::Comma) {
                self.advance();
                continue;
            }

            break;
        }

        self.consume_func(TokenKind::CParen)?;
        Some(params)
    }

    fn parse_return_type(&mut self) -> Type {
        if self.stream.is(TokenKind::Colon) {
            self.advance();
            if let Some(ty) = self.parse_type() {
                return ty;
            }
        }
        Type::Void
    }
}
