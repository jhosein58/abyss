use super::precedence::{self, Precedence};
use crate::{
    ast::{Expr, ExprKind, Lit, UnaryOp},
    error::ParseErrorKind,
    old_parser::Parser,
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

            if (current == TokenKind::Plus
                || current == TokenKind::Minus
                || current == TokenKind::Star
                || current == TokenKind::Slash
                || current == TokenKind::Percent)
                && self.stream.is_peek(TokenKind::Assign)
            {
                break;
            }
            let s = self.get_ast_span();

            if let Some(prec) = Precedence::infix_for(current) {
                if (prec as u8) < min_bp {
                    break;
                }

                if current == TokenKind::Is {
                    self.advance();
                    lhs = Expr {
                        kind: ExprKind::Is(Box::new(lhs), self.parse_type()?),
                        span: s.clone(),
                        ty: None,
                    };
                    continue;
                }

                self.advance();
                let op = precedence::token_to_binary_op(current);
                let rhs = self.parse_expr_bp(prec.next_power())?;
                lhs = Expr {
                    kind: ExprKind::Binary(Box::new(lhs), op, Box::new(rhs)),
                    span: s.clone(),
                    ty: None,
                };
            } else {
                break;
            }
        }

        Some(lhs)
    }

    fn parse_prefix(&mut self) -> Option<Expr> {
        let s = self.get_ast_span();
        self.skip_newlines();
        let kind = self.stream.current().kind;

        match kind {
            TokenKind::Literal(LiteralKind::Int) => self.parse_int_literal(),
            TokenKind::Literal(LiteralKind::Float) => self.parse_float_literal(),
            TokenKind::Literal(LiteralKind::Str) => self.parse_str_literal(),
            TokenKind::True => {
                self.advance();
                Some(Expr {
                    kind: ExprKind::Lit(Lit::Bool(true)),
                    span: s.clone(),
                    ty: None,
                })
            }
            TokenKind::False => {
                self.advance();
                Some(Expr {
                    kind: ExprKind::Lit(Lit::Bool(false)),
                    span: s.clone(),
                    ty: None,
                })
            }
            TokenKind::Null => {
                self.advance();
                Some(Expr {
                    kind: ExprKind::Lit(Lit::Null),
                    span: s.clone(),
                    ty: None,
                })
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
                Some(Expr {
                    kind: ExprKind::SizeOf(ty),
                    span: s.clone(),
                    ty: None,
                })
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
                Some(Expr {
                    kind: ExprKind::Unary(op, Box::new(rhs)),
                    span: s.clone(),
                    ty: None,
                })
            }

            TokenKind::Star => {
                self.advance();
                Some(Expr {
                    kind: ExprKind::Deref(Box::new(self.parse_expr_bp(Precedence::Unary as u8)?)),
                    span: s.clone(),
                    ty: None,
                })
            }
            TokenKind::Amp => {
                self.advance();
                Some(Expr {
                    kind: ExprKind::AddrOf(Box::new(self.parse_expr_bp(Precedence::Unary as u8)?)),
                    span: s.clone(),
                    ty: None,
                })
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
        let s = self.get_ast_span();
        match self.stream.current().kind {
            TokenKind::Dot => self.parse_dot_access(lhs),
            TokenKind::OParen => {
                self.advance();
                let args = self.parse_call_args()?;
                Some(Expr {
                    kind: ExprKind::Call(Box::new(lhs), args, Vec::new()),
                    span: s.clone(),
                    ty: None,
                })
            }
            TokenKind::OBracket => {
                self.advance();
                let idx = self.parse_expr()?;
                self.consume(TokenKind::CBracket)?;
                Some(Expr {
                    kind: ExprKind::Index(Box::new(lhs), Box::new(idx)),
                    span: s.clone(),
                    ty: None,
                })
            }
            TokenKind::As => {
                self.advance();
                Some(Expr {
                    kind: ExprKind::Cast(Box::new(lhs), self.parse_type()?),
                    span: s.clone(),
                    ty: None,
                })
            }
            _ => unreachable!(),
        }
    }

    fn parse_int_literal(&mut self) -> Option<Expr> {
        let s = self.get_ast_span();
        let val = self.stream.current_lit().parse::<i64>().ok()?;
        self.advance();
        Some(Expr {
            kind: ExprKind::Lit(Lit::Int(val)),
            span: s.clone(),
            ty: None,
        })
    }

    fn parse_float_literal(&mut self) -> Option<Expr> {
        let s = self.get_ast_span();
        let val = self.stream.current_lit().parse::<f64>().ok()?;
        self.advance();
        Some(Expr {
            kind: ExprKind::Lit(Lit::Float(val)),
            span: s.clone(),
            ty: None,
        })
    }

    fn parse_str_literal(&mut self) -> Option<Expr> {
        let s = self.get_ast_span();
        let val = self.stream.current_lit().to_string();
        self.advance();
        Some(Expr {
            kind: ExprKind::Lit(Lit::Str(val)),
            span: s.clone(),
            ty: None,
        })
    }

    fn parse_ident_expr(&mut self) -> Option<Expr> {
        let s = self.get_ast_span();
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
            return Some(Expr {
                kind: ExprKind::Call(
                    Box::new(Expr {
                        kind: ExprKind::Ident(path),
                        span: s.clone(),
                        ty: None,
                    }),
                    args,
                    Vec::new(),
                ),
                span: s.clone(),
                ty: None,
            });
        }

        Some(Expr {
            kind: ExprKind::Ident(path),
            span: s.clone(),
            ty: None,
        })
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
        let s = self.get_ast_span();
        if self.stream.is(TokenKind::OParen) {
            self.advance();
            let args = self.parse_call_args()?;
            Some(Expr {
                kind: ExprKind::Call(
                    Box::new(Expr {
                        kind: ExprKind::Ident(path),
                        span: s.clone(),
                        ty: None,
                    }),
                    args,
                    generics,
                ),
                span: s.clone(),
                ty: None,
            })
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
        let s = self.get_ast_span();
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
            Some(Expr {
                kind: ExprKind::MethodCall(Box::new(lhs), name, args, generics),
                span: s.clone(),
                ty: None,
            })
        } else {
            if !generics.is_empty() {
                self.emit_error_at_current(ParseErrorKind::Message(
                    "Field access cannot have generic arguments".to_string(),
                ));
                return None;
            }
            Some(Expr {
                kind: ExprKind::Member(Box::new(lhs), name),
                span: s.clone(),
                ty: None,
            })
        }
    }

    fn parse_array_literal(&mut self) -> Option<Expr> {
        let s = self.get_ast_span();
        let mut elements = Vec::new();
        if self.is(TokenKind::CBracket) {
            self.advance();
            return Some(Expr {
                kind: ExprKind::Lit(Lit::Array(elements)),
                span: s.clone(),
                ty: None,
            });
        }
        loop {
            elements.push(self.parse_expr()?);
            if !self.stream.consume(TokenKind::Comma) {
                break;
            }
        }
        self.consume(TokenKind::CBracket)?;
        Some(Expr {
            kind: ExprKind::Lit(Lit::Array(elements)),
            span: s.clone(),
            ty: None,
        })
    }
}
