use crate::{
    ast::{Expr, Type},
    error::ParseErrorKind,
    old_parser::Parser,
};
use abyss_lexer::token::TokenKind;

impl<'a> Parser<'a> {
    pub(crate) fn parse_call_args(&mut self) -> Option<Vec<Expr>> {
        let mut args = Vec::new();
        if !self.stream.is(TokenKind::CParen) {
            loop {
                args.push(self.parse_expr()?);
                if !self.stream.consume(TokenKind::Comma) {
                    break;
                }
            }
        }
        self.consume(TokenKind::CParen)?;
        Some(args)
    }

    pub(crate) fn convert_right_shift_to_gt(&mut self) {
        let token = self.stream.current_mut();
        if token.kind == TokenKind::RightShift {
            token.kind = TokenKind::Gt;
        }
    }

    pub(crate) fn parse_generic_args(&mut self) -> Option<Vec<Type>> {
        if !self.stream.is(TokenKind::Lt) && !self.stream.is(TokenKind::LeftShift) {
            return Some(Vec::new());
        }

        if self.stream.is(TokenKind::LeftShift) {
            self.emit_error_at_current(ParseErrorKind::Message(
                "Use space between nested generics: < <".to_string(),
            ));
            return None;
        }

        self.advance();
        let mut args = Vec::new();

        while !self.stream.is(TokenKind::Gt)
            && !self.stream.is(TokenKind::RightShift)
            && !self.stream.is_at_end()
        {
            args.push(self.parse_type()?);
            if self.stream.is(TokenKind::Comma) {
                self.advance();
            } else {
                break;
            }
        }

        if self.stream.is(TokenKind::Gt) {
            self.advance();
        } else if self.stream.is(TokenKind::RightShift) {
            self.convert_right_shift_to_gt();
        } else {
            self.emit_error_at_current(ParseErrorKind::Expected(
                "'>' to close generic args".to_string(),
            ));
            return None;
        }

        Some(args)
    }
}
