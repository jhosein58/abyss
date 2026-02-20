use crate::{
    ast::{Expr, ExprKind, Type},
    error::ParseErrorKind,
    old_parser::Parser,
};
use abyss_lexer::token::TokenKind;

impl<'a> Parser<'a> {
    pub(crate) fn parse_struct_init(&mut self, path: Vec<String>) -> Option<Expr> {
        let s = self.get_ast_span();
        let generics =
            if self.stream.is(TokenKind::ColonColon) && self.stream.is_peek(TokenKind::Lt) {
                self.advance();
                self.parse_generic_args()?
            } else {
                Vec::new()
            };

        if self.stream.is(TokenKind::OBrace) {
            self.parse_struct_literal(path, generics)
        } else {
            Some(Expr {
                kind: ExprKind::StructInit(path, vec![], generics),
                span: s,
                ty: None,
            })
        }
    }

    pub(crate) fn parse_struct_literal(
        &mut self,
        path: Vec<String>,
        generics: Vec<Type>,
    ) -> Option<Expr> {
        let s = self.get_ast_span();
        self.consume(TokenKind::OBrace)?;
        let mut fields = Vec::new();

        while !self.stream.is(TokenKind::CBrace) && !self.stream.is_at_end() {
            self.skip_newlines();
            if self.stream.is(TokenKind::CBrace) {
                break;
            }

            let field_name = self.expect_ident("Field name")?;

            if !self.stream.consume(TokenKind::Colon) {
                self.emit_error_at_current(ParseErrorKind::Expected(
                    "Colon ':' after field name".to_string(),
                ));
                return None;
            }

            fields.push((field_name, self.parse_expr()?));

            if !self.stream.consume(TokenKind::Comma) && !self.stream.is(TokenKind::CBrace) {
                self.emit_error_at_current(ParseErrorKind::Expected(
                    "Comma or closing brace".to_string(),
                ));
                return None;
            }

            self.skip_newlines();
        }

        self.consume(TokenKind::CBrace)?;
        Some(Expr {
            kind: ExprKind::StructInit(path, fields, generics),
            span: s,
            ty: None,
        })
    }

    fn expect_ident(&mut self, context: &str) -> Option<String> {
        if !self.stream.is(TokenKind::Ident) {
            self.emit_error_at_current(ParseErrorKind::Expected(context.to_string()));
            return None;
        }
        let name = self.stream.current_lit().to_string();
        self.advance();
        Some(name)
    }
}
