use abyss_lexer::token::TokenKind;

use crate::{
    ast::{FunctionBody, FunctionDef, StaticDef, StructDef, Type},
    error::ParseErrorKind,
    parser::Parser,
};

impl<'a> Parser<'a> {
    fn synchronize_func(&mut self) {
        self.stream.advance();

        while !self.stream.is_at_end() {
            match self.stream.current().kind {
                TokenKind::Fn | TokenKind::Struct | TokenKind::Static | TokenKind::Pub => {
                    return;
                }
                _ => {}
            }

            self.stream.advance();
        }
    }
    fn consume_safely(&mut self, expected: TokenKind) -> Option<()> {
        if !self.stream.consume(expected) {
            self.emit_error_at_current(ParseErrorKind::UnexpectedToken {
                expected,
                found: self.stream.current().kind,
            });
            self.synchronize_func();
            return None;
        }
        Some(())
    }
    pub fn parse_function(&mut self, is_pub: bool) -> Option<FunctionDef> {
        self.consume_safely(TokenKind::Fn)?;

        let name = self.read_ident()?;

        let mut generics = Vec::new();
        if self.stream.is(TokenKind::Lt) {
            self.advance();
            while !self.stream.is(TokenKind::Gt) && !self.stream.is_at_end() {
                let gen_name = self.read_ident()?;
                generics.push(gen_name);
                if self.stream.is(TokenKind::Comma) {
                    self.advance();
                } else {
                    break;
                }
            }
            self.consume_safely(TokenKind::Gt)?;
        }

        let params = self.parse_func_params()?;
        let return_type = self.parse_return_type();

        let body = if self.stream.is(TokenKind::Semi) || self.stream.is(TokenKind::Semi) {
            self.advance();
            FunctionBody::Extern
        } else {
            if let Some(stmts) = self.parse_block() {
                FunctionBody::UserDefined(stmts)
            } else {
                self.synchronize();
                return None;
            }
        };

        Some(FunctionDef {
            is_pub,
            name,
            generics,
            params,
            return_type,
            body,
        })
    }

    pub fn parse_struct_def(&mut self, is_pub: bool) -> Option<StructDef> {
        self.consume_safely(TokenKind::Struct)?;

        let name = self.read_ident()?;

        let mut generics = Vec::new();
        if self.stream.is(TokenKind::Lt) {
            self.advance();
            while !self.stream.is(TokenKind::Gt) && !self.stream.is_at_end() {
                let gen_name = self.read_ident()?;
                generics.push(gen_name);
                if self.stream.is(TokenKind::Comma) {
                    self.advance();
                } else {
                    break;
                }
            }
            self.consume_safely(TokenKind::Gt)?;
        }

        self.consume_safely(TokenKind::OBrace)?;

        let mut fields = Vec::new();

        while !self.stream.is(TokenKind::CBrace) && !self.stream.is_at_end() {
            self.skip_newlines();
            if self.stream.is(TokenKind::CBrace) {
                break;
            }

            let field_name = self.read_ident()?;
            self.consume_safely(TokenKind::Colon)?;
            let field_type = self.parse_type()?;

            fields.push((field_name, field_type));

            if self.stream.is(TokenKind::Comma) {
                self.advance();
            }
            self.skip_newlines();
        }

        self.consume_safely(TokenKind::CBrace)?;

        Some(StructDef {
            is_pub,
            name,
            fields,
            generics,
        })
    }

    pub fn parse_static_def(&mut self, is_pub: bool) -> Option<StaticDef> {
        self.consume_safely(TokenKind::Static)?;

        let name = self.read_ident()?;

        self.consume_safely(TokenKind::Colon)?;
        let ty = self.parse_type()?;

        let has_eq = if self.stream.is(TokenKind::Assign) {
            self.advance();
            true
        } else if self.stream.is(TokenKind::Assign) {
            self.advance();
            true
        } else {
            false
        };

        let init_value = if has_eq {
            Some(self.parse_expr()?)
        } else {
            None
        };

        if !self.stream.consume(TokenKind::Semi) && !self.stream.consume(TokenKind::Semi) {
            self.emit_error_at_current(ParseErrorKind::UnexpectedToken {
                expected: TokenKind::Semi,
                found: self.stream.current().kind,
            });

            self.synchronize_func();
            return None;
        }

        if let Some(val) = init_value {
            Some(StaticDef {
                generics: vec![],
                is_pub,
                name,
                ty,
                value: val,
            })
        } else {
            self.emit_error_at_current(ParseErrorKind::Message(
                "Static variables must have an initial value".to_string(),
            ));
            None
        }
    }

    fn read_ident(&mut self) -> Option<String> {
        if self.stream.is(TokenKind::Ident) {
            let ident = self.stream.current_lit().to_string();
            self.advance();
            Some(ident)
        } else {
            self.emit_error_at_current(ParseErrorKind::Expected("ident".to_string()));
            self.synchronize_func();
            None
        }
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
            self.consume_safely(TokenKind::Colon)?;
            let ty = self.parse_type()?;
            params.push((name, ty));
            if self.stream.is(TokenKind::Comma) {
                self.advance();
                continue;
            }
            break;
        }
        self.consume_safely(TokenKind::CParen)?;
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

    pub fn parse_impl_block(&mut self) -> Vec<FunctionDef> {
        if !self.consume_safely(TokenKind::Impl).is_some() {
            return Vec::new();
        }

        let struct_name = match self.read_ident() {
            Some(name) => name,
            None => return Vec::new(),
        };

        if !self.consume_safely(TokenKind::OBrace).is_some() {
            return Vec::new();
        }

        let mut methods = Vec::new();

        while !self.stream.is(TokenKind::CBrace) && !self.stream.is_at_end() {
            self.skip_newlines();

            if self.stream.is(TokenKind::CBrace) {
                break;
            }

            let is_pub = if self.stream.is(TokenKind::Pub) {
                self.advance();
                true
            } else {
                false
            };

            if self.stream.is(TokenKind::Fn) {
                if let Some(mut func) = self.parse_function(is_pub) {
                    let old_name = func.name.clone();
                    let new_name = format!("{}__{}", struct_name, old_name);
                    func.name = new_name;

                    methods.push(func);
                }
            } else {
                self.emit_error_at_current(ParseErrorKind::UnexpectedToken {
                    expected: TokenKind::Fn,
                    found: self.stream.current().kind,
                });
                self.synchronize_func();
            }

            self.skip_newlines();
        }

        self.consume_safely(TokenKind::CBrace);

        methods
    }
}
