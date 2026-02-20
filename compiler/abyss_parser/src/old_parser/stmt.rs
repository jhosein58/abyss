use crate::{
    ast::{BinaryOp, Expr, ExprKind, Lit, Stmt, StmtKind},
    old_parser::Parser,
};
use abyss_lexer::token::TokenKind as Tk;

impl<'a> Parser<'a> {
    pub fn parse_stmt(&mut self) -> Option<Stmt> {
        let stmt = match self.stream.current().kind {
            Tk::Let => self.parse_let_stmt()?,
            Tk::Fn => self.parse_nested_function()?,
            Tk::Ret => self.parse_ret_stmt()?,
            Tk::If => self.parse_if_stmt()?,

            Tk::While => self.parse_while_stmt()?,
            Tk::For => self.parse_for_stmt()?,
            Tk::Forever => self.parse_forever_stmt()?,

            Tk::Out => self.parse_out_stmt()?,
            Tk::Next => self.parse_next_stmt()?,

            _ => self.parse_assignment_or_expr_stmt()?,
        };

        self.optional(Tk::Semi);
        self.optional(Tk::Newline);
        Some(stmt)
    }

    fn parse_assignment_or_expr_stmt(&mut self) -> Option<Stmt> {
        let s = self.get_ast_span();
        let lhs = self.parse_expr()?;

        if self.stream.is(Tk::Assign) {
            self.advance();
            return Some(Stmt {
                kind: StmtKind::Assign(lhs, self.parse_expr()?),
                span: s.clone(),
            });
        }

        if let Some(op) = self.try_compound_assign() {
            let rhs = self.parse_expr()?;
            return Some(Stmt {
                kind: StmtKind::Assign(
                    lhs.clone(),
                    Expr {
                        kind: ExprKind::Binary(Box::new(lhs), op, Box::new(rhs)),
                        span: s.clone(),
                        ty: None,
                    },
                ),
                span: s,
            });
        }

        Some(Stmt {
            kind: StmtKind::Expr(lhs),
            span: s,
        })
    }

    fn try_compound_assign(&mut self) -> Option<BinaryOp> {
        let op = match self.stream.current().kind {
            Tk::Plus => BinaryOp::Add,
            Tk::Minus => BinaryOp::Sub,
            Tk::Star => BinaryOp::Mul,
            Tk::Slash => BinaryOp::Div,
            Tk::Percent => BinaryOp::Mod,
            _ => return None,
        };

        if self.stream.is_peek(Tk::Assign) {
            self.advance();
            self.advance();
            Some(op)
        } else {
            None
        }
    }

    fn parse_nested_function(&mut self) -> Option<Stmt> {
        let s = self.get_ast_span();
        let func_def = self.parse_function(false)?;

        Some(Stmt {
            kind: StmtKind::FunctionDef(Box::new(func_def)),
            span: s,
        })
    }
    fn parse_forever_stmt(&mut self) -> Option<Stmt> {
        let s = self.get_ast_span();
        self.consume(Tk::Forever)?;

        let body_stmts = self.parse_block()?;

        Some(Stmt {
            kind: StmtKind::While(
                Expr {
                    kind: ExprKind::Lit(Lit::Bool(true)),
                    span: s.clone(),
                    ty: None,
                },
                Box::new(Stmt {
                    kind: StmtKind::Block(body_stmts),
                    span: s.clone(),
                }),
            ),
            span: s,
        })
    }

    fn parse_let_stmt(&mut self) -> Option<Stmt> {
        let s = self.get_ast_span();
        self.consume(Tk::Let)?;

        let name = self.consume_ident()?;

        let explicit_type = if self.stream.is(Tk::Colon) {
            self.advance();
            Some(self.parse_type()?)
        } else {
            None
        };

        let expr = if self.stream.is(Tk::Assign) {
            self.advance();
            Some(self.parse_expr()?)
        } else {
            None
        };

        Some(Stmt {
            kind: StmtKind::Let(name, explicit_type, expr),
            span: s,
        })
    }

    fn parse_ret_stmt(&mut self) -> Option<Stmt> {
        let s = self.get_ast_span();
        self.consume(Tk::Ret)?;
        if self.is(Tk::Semi) || self.is(Tk::Newline) {
            self.advance();
            return Some(Stmt {
                kind: StmtKind::Ret(Expr {
                    kind: ExprKind::Lit(Lit::Null),
                    span: s.clone(),
                    ty: None,
                }),
                span: s,
            });
        }
        let expr = self.parse_expr()?;
        Some(Stmt {
            kind: StmtKind::Ret(expr),
            span: s.clone(),
        })
    }

    fn parse_if_stmt(&mut self) -> Option<Stmt> {
        let s = self.get_ast_span();
        self.consume(Tk::If)?;

        let condition = self.parse_expr()?;

        let then_stmts;
        if self.stream.is(Tk::Then) || !self.is(Tk::OBrace) {
            self.optional(Tk::Then);
            then_stmts = vec![self.parse_stmt()?];
        } else {
            then_stmts = self.parse_block()?;
        }

        let then_branch = Box::new(Stmt {
            kind: StmtKind::Block(then_stmts),
            span: s.clone(),
        });

        self.optional(Tk::Newline);

        let else_branch = if self.stream.is(Tk::Else) {
            self.advance();
            self.optional(Tk::Newline);

            if self.is(Tk::If) || self.is(Tk::OBrace) {
                if self.stream.is(Tk::If) {
                    let nested_if = self.parse_stmt()?;
                    Some(Box::new(nested_if))
                } else {
                    let else_stmts = self.parse_block()?;
                    Some(Box::new(Stmt {
                        kind: StmtKind::Block(else_stmts),
                        span: s.clone(),
                    }))
                }
            } else {
                let stmt = self.parse_stmt()?;
                Some(Box::new(stmt))
            }
        } else {
            None
        };

        Some(Stmt {
            kind: StmtKind::If(condition, then_branch, else_branch),
            span: s.clone(),
        })
    }

    fn parse_while_stmt(&mut self) -> Option<Stmt> {
        let s = self.get_ast_span();
        self.consume(Tk::While)?;
        let condition = self.parse_expr()?;
        let body_stmts = self.parse_block()?;
        Some(Stmt {
            kind: StmtKind::While(
                condition,
                Box::new(Stmt {
                    kind: StmtKind::Block(body_stmts),
                    span: s.clone(),
                }),
            ),
            span: s.clone(),
        })
    }

    fn parse_for_stmt(&mut self) -> Option<Stmt> {
        self.consume(Tk::For)?;

        if self.stream.is(Tk::Ident) && self.stream.is_peek(Tk::Colon) {
            return self.parse_for_each();
        }

        if self.stream.is(Tk::Ident) && self.stream.is_peek(Tk::In) {
            return self.parse_for_range();
        }

        self.parse_for_count()
    }

    fn parse_for_each(&mut self) -> Option<Stmt> {
        let item_name = self.consume_ident()?;
        self.consume(Tk::Colon)?;
        let item_type = self.parse_type()?;
        self.consume(Tk::In)?;
        let collection = self.parse_expr()?;
        let body = self.parse_block()?;

        Some(self.desugar_for_each(item_name, item_type, collection, body))
    }

    fn parse_for_range(&mut self) -> Option<Stmt> {
        let ident = self.consume_ident()?;
        self.consume(Tk::In)?;
        let start = self.parse_expr()?;
        self.consume(Tk::RArrow)?;
        let end = self.parse_expr()?;
        let body = self.parse_block()?;

        Some(self.desugar_for_range(ident, start, end, body))
    }

    fn parse_for_count(&mut self) -> Option<Stmt> {
        let end = self.parse_expr()?;
        let body = self.parse_block()?;

        Some(self.desugar_for_count(end, body))
    }

    pub fn parse_out_stmt(&mut self) -> Option<Stmt> {
        let s = self.get_ast_span();
        self.consume(Tk::Out)?;
        Some(Stmt {
            kind: StmtKind::Break,
            span: s,
        })
    }

    pub fn parse_next_stmt(&mut self) -> Option<Stmt> {
        let s = self.get_ast_span();
        self.consume(Tk::Next)?;
        Some(Stmt {
            kind: StmtKind::Continue,
            span: s,
        })
    }
}
