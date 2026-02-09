use super::precedence::{self, Precedence};
use crate::{
    ast::{Expr, Lit, UnaryOp},
    error::ParseErrorKind,
    parser::Parser,
};
use abyss_lexer::token::{LiteralKind, TokenKind};

impl<'a> Parser<'a> {
    pub fn parse_expr(&mut self) -> Option<Expr> {
        self.parse_expr_bp(0)
    }

    fn parse_expr_bp(&mut self, min_bp: u8) -> Option<Expr> {
        self.skip_newlines();
        let mut lhs = self.parse_prefix()?;

        loop {
            self.skip_newlines();

            while let Some(prec) = Precedence::postfix_for(self.stream.current().kind) {
                if (prec as u8) < min_bp {
                    break;
                }
                lhs = self.parse_postfix(lhs)?;
            }

            let current = self.stream.current().kind;

            if (current == TokenKind::Plus || current == TokenKind::Minus)
                && self.stream.is_peek(TokenKind::Assign)
            {
                break;
            }

            if let Some(prec) = Precedence::infix_for(current) {
                if (prec as u8) < min_bp {
                    break;
                }

                if current == TokenKind::Is {
                    self.advance();
                    lhs = Expr::Is(Box::new(lhs), self.parse_type()?);
                    continue;
                }

                self.advance();
                let op = precedence::token_to_binary_op(current);
                let rhs = self.parse_expr_bp(prec.next_power())?;
                lhs = Expr::Binary(Box::new(lhs), op, Box::new(rhs));
            } else {
                break;
            }
        }

        Some(lhs)
    }

    fn parse_prefix(&mut self) -> Option<Expr> {
        self.skip_newlines();
        let kind = self.stream.current().kind;

        match kind {
            TokenKind::Literal(LiteralKind::Int) => self.parse_int_literal(),
            TokenKind::Literal(LiteralKind::Float) => self.parse_float_literal(),
            TokenKind::Literal(LiteralKind::Str) => self.parse_str_literal(),
            TokenKind::True => {
                self.advance();
                Some(Expr::Lit(Lit::Bool(true)))
            }
            TokenKind::False => {
                self.advance();
                Some(Expr::Lit(Lit::Bool(false)))
            }
            TokenKind::Null => {
                self.advance();
                Some(Expr::Lit(Lit::Null))
            }

            TokenKind::Ident => self.parse_ident_expr(),

            TokenKind::OParen => {
                self.advance();
                let expr = self.parse_expr()?;
                self.consume(TokenKind::CParen)?;
                Some(expr)
            }

            TokenKind::OBracket => {
                self.advance();
                self.parse_array_literal()
            }

            TokenKind::Size => {
                self.advance();
                self.consume(TokenKind::OParen)?;
                let ty = self.parse_type()?;
                self.consume(TokenKind::CParen)?;
                Some(Expr::SizeOf(ty))
            }

            TokenKind::Minus | TokenKind::Not | TokenKind::Tilde => {
                let op = match kind {
                    TokenKind::Minus => UnaryOp::Neg,
                    TokenKind::Not => UnaryOp::Not,
                    TokenKind::Tilde => UnaryOp::BitNot,
                    _ => unreachable!(),
                };
                self.advance();
                let rhs = self.parse_expr_bp(Precedence::Unary as u8)?;
                Some(Expr::Unary(op, Box::new(rhs)))
            }

            TokenKind::Star => {
                self.advance();
                Some(Expr::Deref(Box::new(
                    self.parse_expr_bp(Precedence::Unary as u8)?,
                )))
            }
            TokenKind::Amp => {
                self.advance();
                Some(Expr::AddrOf(Box::new(
                    self.parse_expr_bp(Precedence::Unary as u8)?,
                )))
            }

            TokenKind::Struct => {
                self.advance();
                let path = self.parse_path()?;
                self.parse_struct_init(path)
            }

            _ => {
                self.emit_error_at_current(ParseErrorKind::UnexpectedToken {
                    expected: TokenKind::Unknown,
                    found: kind,
                });
                None
            }
        }
    }

    fn parse_postfix(&mut self, lhs: Expr) -> Option<Expr> {
        match self.stream.current().kind {
            TokenKind::Dot => self.parse_dot_access(lhs),
            TokenKind::OParen => {
                self.advance();
                let args = self.parse_call_args()?;
                Some(Expr::Call(Box::new(lhs), args, Vec::new()))
            }
            TokenKind::OBracket => {
                self.advance();
                let idx = self.parse_expr()?;
                self.consume(TokenKind::CBracket)?;
                Some(Expr::Index(Box::new(lhs), Box::new(idx)))
            }
            TokenKind::As => {
                self.advance();
                Some(Expr::Cast(Box::new(lhs), self.parse_type()?))
            }
            _ => unreachable!(),
        }
    }

    fn parse_int_literal(&mut self) -> Option<Expr> {
        let val = self.stream.current_lit().parse::<i64>().ok()?;
        self.advance();
        Some(Expr::Lit(Lit::Int(val)))
    }

    fn parse_float_literal(&mut self) -> Option<Expr> {
        let val = self.stream.current_lit().parse::<f64>().ok()?;
        self.advance();
        Some(Expr::Lit(Lit::Float(val)))
    }

    fn parse_str_literal(&mut self) -> Option<Expr> {
        let val = self.stream.current_lit().to_string();
        self.advance();
        Some(Expr::Lit(Lit::Str(val)))
    }

    fn parse_ident_expr(&mut self) -> Option<Expr> {
        let path = self.parse_path()?;

        let struct_name = path.join("__");
        if self.structs.contains(&struct_name) {
            return self.parse_struct_init(path);
        }

        let generics = self.try_parse_turbofish()?;

        if !generics.is_empty() {
            return self.parse_generic_continuation(path, generics);
        }

        if self.stream.is(TokenKind::OParen) {
            self.advance();
            let args = self.parse_call_args()?;
            return Some(Expr::Call(Box::new(Expr::Ident(path)), args, Vec::new()));
        }

        Some(Expr::Ident(path))
    }

    fn try_parse_turbofish(&mut self) -> Option<Vec<crate::ast::Type>> {
        if self.stream.is(TokenKind::ColonColon)
            && (self.stream.is_peek(TokenKind::Lt) || self.stream.is_peek(TokenKind::LeftShift))
        {
            self.advance();
            self.parse_generic_args()
        } else {
            Some(Vec::new())
        }
    }

    fn parse_generic_continuation(
        &mut self,
        path: Vec<String>,
        generics: Vec<crate::ast::Type>,
    ) -> Option<Expr> {
        if self.stream.is(TokenKind::OParen) {
            self.advance();
            let args = self.parse_call_args()?;
            Some(Expr::Call(Box::new(Expr::Ident(path)), args, generics))
        } else if self.stream.is(TokenKind::OBrace) {
            self.parse_struct_literal(path, generics)
        } else {
            self.emit_error_at_current(ParseErrorKind::Message(
                "Expected '(' or '{' after generic type arguments".to_string(),
            ));
            None
        }
    }

    fn parse_dot_access(&mut self, lhs: Expr) -> Option<Expr> {
        self.advance();

        if !self.stream.is(TokenKind::Ident) {
            self.emit_error_at_current(ParseErrorKind::Expected(
                "Field or method name".to_string(),
            ));
            return None;
        }

        let name = self.stream.current_lit().to_string();
        self.advance();

        let generics = if self.stream.is(TokenKind::ColonColon) {
            self.advance();
            self.parse_generic_args()?
        } else {
            Vec::new()
        };

        if self.stream.is(TokenKind::OParen) {
            self.advance();
            let args = self.parse_call_args()?;
            Some(Expr::MethodCall(Box::new(lhs), name, args, generics))
        } else {
            if !generics.is_empty() {
                self.emit_error_at_current(ParseErrorKind::Message(
                    "Field access cannot have generic arguments".to_string(),
                ));
                return None;
            }
            Some(Expr::Member(Box::new(lhs), name))
        }
    }

    fn parse_array_literal(&mut self) -> Option<Expr> {
        let mut elements = Vec::new();
        if self.is(TokenKind::CBracket) {
            self.advance();
            return Some(Expr::Lit(Lit::Array(elements)));
        }
        loop {
            elements.push(self.parse_expr()?);
            if !self.stream.consume(TokenKind::Comma) {
                break;
            }
        }
        self.consume(TokenKind::CBracket)?;
        Some(Expr::Lit(Lit::Array(elements)))
    }
}
