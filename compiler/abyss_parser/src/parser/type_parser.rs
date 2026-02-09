use crate::{ast::Type, error::ParseErrorKind, parser::Parser};
use abyss_lexer::token::{LiteralKind, TokenKind};

impl<'a> Parser<'a> {
    pub fn parse_type(&mut self) -> Option<Type> {
        let first = self.parse_unary_type()?;

        if self.stream.is(TokenKind::Pipe) {
            let mut variants = vec![first];
            while self.stream.consume(TokenKind::Pipe) {
                variants.push(self.parse_unary_type()?);
            }
            return Some(Type::Union(variants));
        }

        Some(first)
    }

    fn parse_unary_type(&mut self) -> Option<Type> {
        if self.stream.is(TokenKind::Const) {
            self.advance();
            return Some(Type::Const(Box::new(self.parse_unary_type()?)));
        }

        if self.stream.is(TokenKind::Amp) {
            self.advance();
            return Some(Type::Pointer(Box::new(self.parse_unary_type()?)));
        }

        let mut base = self.parse_base_type()?;

        loop {
            if self.stream.is(TokenKind::Star) {
                self.advance();
                base = Type::Pointer(Box::new(base));
            } else if self.stream.is(TokenKind::OBracket) {
                self.advance();
                base = self.parse_array_type_suffix(base)?;
            } else {
                break;
            }
        }

        Some(base)
    }

    fn parse_base_type(&mut self) -> Option<Type> {
        let simple_types: &[(TokenKind, Type)] = &[
            (TokenKind::U8, Type::U8),
            (TokenKind::U16, Type::U16),
            (TokenKind::U32, Type::U32),
            (TokenKind::U64, Type::U64),
            (TokenKind::Usize, Type::Usize),
            (TokenKind::I8, Type::I8),
            (TokenKind::I16, Type::I16),
            (TokenKind::I32, Type::I32),
            (TokenKind::I64, Type::I64),
            (TokenKind::Isize, Type::Isize),
            (TokenKind::F32, Type::F32),
            (TokenKind::F64, Type::F64),
            (TokenKind::Bool, Type::Bool),
            (TokenKind::Pass, Type::Void),
            (TokenKind::Char, Type::Char),
        ];

        for (token, ty) in simple_types {
            if self.stream.consume(*token) {
                return Some(ty.clone());
            }
        }

        if self.stream.is(TokenKind::Ident) {
            let path = self.parse_path()?;
            let generics = self.parse_generic_args()?;
            return Some(Type::Struct(path, generics));
        }

        self.emit_error_at_current(ParseErrorKind::Expected("type name".to_string()));
        None
    }

    fn parse_array_type_suffix(&mut self, base: Type) -> Option<Type> {
        if let TokenKind::Literal(LiteralKind::Int) = self.stream.current().kind {
            let text = self.stream.current_lit();
            if let Ok(size) = text.parse::<usize>() {
                self.advance();
                if self.stream.consume(TokenKind::CBracket) {
                    return Some(Type::Array(Box::new(base), size));
                }
            }
        }
        self.emit_error_at_current(ParseErrorKind::Expected("Array size".to_string()));
        None
    }
}
