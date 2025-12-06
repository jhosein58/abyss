use abyss_lexer::token::TokenKind;

use crate::{
    ast::{BlockExpr, Function, FunctionParam, Ident, Type},
    error::ParseErrorKind,
    parser::Parser,
    source_map::Span,
};

impl<'a> Parser<'a> {
    fn synchronize(&mut self) {
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

    fn consume(&mut self, expected: TokenKind) -> Option<()> {
        if !self.stream.consume(expected) {
            self.emit_error_at_current(ParseErrorKind::UnexpectedToken {
                expected: expected,
                found: self.stream.current().kind,
            });
            self.synchronize();
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
            self.synchronize();
            return None;
        }
        Some(())
    }

    fn read_ident(&mut self) -> Option<Ident> {
        self.expect(TokenKind::Ident)?;
        let ident = Ident {
            name: self.stream.current_lit().to_string(),
            span: self.stream.current_span(),
        };
        self.advance();
        Some(ident)
    }

    pub fn parse_function(&mut self) -> Option<Function> {
        if self.stream.is_peek(TokenKind::Eof) {
            self.advance();
            return None;
        }

        self.stream.consume(TokenKind::Newline);

        self.consume(TokenKind::Fn)?;

        let function_ident = self.read_ident()?;

        let args = self.parse_fucn_args()?;

        let return_type = self.parse_return_type();

        self.consume(TokenKind::OBrace)?;
        self.advance();
        self.consume(TokenKind::CBrace)?;

        Some(Function {
            name: function_ident,
            params: args,
            return_type,
            body: BlockExpr {
                scope: vec![],
                span: Span::new(0, 0),
            },
        })
    }

    fn parse_type(&mut self) -> Option<Type> {
        let mut is_ptr = false;

        if self.stream.is(TokenKind::Amp) {
            is_ptr = true;
            self.advance();
        }

        let ty_ident = Type::Named(self.read_ident()?);

        Some(if is_ptr {
            Type::Pointer(Box::new(ty_ident))
        } else {
            ty_ident
        })
    }

    pub fn parse_fucn_args(&mut self) -> Option<Vec<FunctionParam>> {
        let mut args = Vec::new();

        self.consume(TokenKind::OParen)?;

        if self.stream.is(TokenKind::CParen) {
            self.advance();
            return Some(args);
        }

        while self.stream.is(TokenKind::Ident) {
            let arg_ident = self.read_ident()?;

            self.consume(TokenKind::Colon)?;

            args.push(FunctionParam {
                name: arg_ident,
                ty: self.parse_type()?,
            });

            if self.stream.is(TokenKind::Comma) {
                self.advance();
            }
        }

        self.consume(TokenKind::CParen)?;

        Some(args)
    }

    pub fn parse_return_type(&mut self) -> Option<Type> {
        if self.stream.is(TokenKind::Colon) {
            self.advance();
            self.parse_type()
        } else {
            None
        }
    }
}
