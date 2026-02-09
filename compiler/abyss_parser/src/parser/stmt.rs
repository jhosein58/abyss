use crate::{
    ast::{BinaryOp, Expr, Lit, Stmt},
    parser::Parser,
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
        let lhs = self.parse_expr()?;

        if self.stream.is(Tk::Assign) {
            self.advance();
            return Some(Stmt::Assign(lhs, self.parse_expr()?));
        }

        if let Some(op) = self.try_compound_assign() {
            let rhs = self.parse_expr()?;
            return Some(Stmt::Assign(
                lhs.clone(),
                Expr::Binary(Box::new(lhs), op, Box::new(rhs)),
            ));
        }

        Some(Stmt::Expr(lhs))
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
        let func_def = self.parse_function(false)?;

        Some(Stmt::FunctionDef(Box::new(func_def)))
    }
    fn parse_forever_stmt(&mut self) -> Option<Stmt> {
        self.consume(Tk::Forever)?;

        let body_stmts = self.parse_block()?;

        Some(Stmt::While(
            Expr::Lit(Lit::Bool(true)),
            Box::new(Stmt::Block(body_stmts)),
        ))
    }

    fn parse_let_stmt(&mut self) -> Option<Stmt> {
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

        Some(Stmt::Let(name, explicit_type, expr))
    }

    fn parse_ret_stmt(&mut self) -> Option<Stmt> {
        self.consume(Tk::Ret)?;
        if self.is(Tk::Semi) || self.is(Tk::Newline) {
            self.advance();
            return Some(Stmt::Ret(Expr::Lit(Lit::Null)));
        }
        let expr = self.parse_expr()?;
        Some(Stmt::Ret(expr))
    }

    fn parse_if_stmt(&mut self) -> Option<Stmt> {
        self.consume(Tk::If)?;

        let condition = self.parse_expr()?;

        let then_stmts;
        if self.stream.is(Tk::Then) || !self.is(Tk::OBrace) {
            self.optional(Tk::Then);
            then_stmts = vec![self.parse_stmt()?];
        } else {
            then_stmts = self.parse_block()?;
        }

        let then_branch = Box::new(Stmt::Block(then_stmts));

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
                    Some(Box::new(Stmt::Block(else_stmts)))
                }
            } else {
                let stmt = self.parse_stmt()?;
                Some(Box::new(stmt))
            }
        } else {
            None
        };

        Some(Stmt::If(condition, then_branch, else_branch))
    }

    fn parse_while_stmt(&mut self) -> Option<Stmt> {
        self.consume(Tk::While)?;
        let condition = self.parse_expr()?;
        let body_stmts = self.parse_block()?;
        Some(Stmt::While(condition, Box::new(Stmt::Block(body_stmts))))
    }

    fn consume_ident(&mut self) -> Option<String> {
        if self.stream.is(Tk::Ident) {
            let span = self.stream.current_span();
            let name = self.source[span.start..span.end].to_string();
            self.advance();
            Some(name)
        } else {
            None
        }
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
        self.consume(Tk::Out)?;
        Some(Stmt::Break)
    }

    pub fn parse_next_stmt(&mut self) -> Option<Stmt> {
        self.consume(Tk::Next)?;
        Some(Stmt::Continue)
    }
}
