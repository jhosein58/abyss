use super::Parser;
use crate::ast::*;

impl<'a> Parser<'a> {
    pub(crate) fn desugar_for_each(
        &mut self,
        item_name: String,
        item_type: Type,
        collection: Expr,
        mut body: Vec<Stmt>,
    ) -> Stmt {
        let s = self.get_ast_span();
        let col_var = self.get_unique_identifier();

        let reset = Stmt {
            kind: StmtKind::Expr(Expr {
                kind: ExprKind::MethodCall(
                    Box::new(Expr {
                        kind: ExprKind::Ident(vec![col_var.clone()]),
                        span: s.clone(),
                        ty: None,
                    }),
                    "reset_cursor".into(),
                    vec![],
                    vec![],
                ),
                span: s.clone(),
                ty: None,
            }),
            span: s.clone(),
        };

        let has_next = Expr {
            kind: ExprKind::MethodCall(
                Box::new(Expr {
                    kind: ExprKind::Ident(vec![col_var.clone()]),
                    span: s.clone(),
                    ty: None,
                }),
                "has_next".into(),
                vec![],
                vec![],
            ),
            span: s.clone(),
            ty: None,
        };

        let item_decl = Stmt {
            kind: StmtKind::Let(
                item_name,
                Some(item_type),
                Some(Expr {
                    kind: ExprKind::MethodCall(
                        Box::new(Expr {
                            kind: ExprKind::Ident(vec![col_var.clone()]),
                            span: s.clone(),
                            ty: None,
                        }),
                        "bump".into(),
                        vec![],
                        vec![],
                    ),
                    span: s.clone(),
                    ty: None,
                }),
            ),
            span: s.clone(),
        };

        let mut loop_body = vec![item_decl];
        loop_body.append(&mut body);

        Stmt {
            kind: StmtKind::Block(vec![
                Stmt {
                    kind: StmtKind::Let(col_var, None, Some(collection)),
                    span: s.clone(),
                },
                reset,
                Stmt {
                    kind: StmtKind::While(
                        has_next,
                        Box::new(Stmt {
                            kind: StmtKind::Block(loop_body),
                            span: s.clone(),
                        }),
                    ),
                    span: s.clone(),
                },
            ]),
            span: s.clone(),
        }
    }
    pub(crate) fn desugar_for_range(
        &mut self,
        ident: String,
        start: Expr,
        end: Expr,
        mut body: Vec<Stmt>,
    ) -> Stmt {
        let s = start.span.clone();
        let i_type = Type::I64;

        let ident_expr = Expr {
            kind: ExprKind::Ident(vec![ident.clone()]),
            span: s.clone(),
            ty: None,
        };

        let one_lit = Expr {
            kind: ExprKind::Lit(Lit::Int(1)),
            span: s.clone(),
            ty: Some(Type::I64),
        };

        let inc = Stmt {
            kind: StmtKind::Assign(
                ident_expr.clone(),
                Expr {
                    kind: ExprKind::Binary(
                        Box::new(ident_expr.clone()),
                        BinaryOp::Add,
                        Box::new(one_lit.clone()),
                    ),
                    span: s.clone(),
                    ty: None,
                },
            ),
            span: s.clone(),
        };

        let mut loop_body = vec![inc];
        loop_body.append(&mut body);

        let init_val = Expr {
            kind: ExprKind::Binary(Box::new(start), BinaryOp::Sub, Box::new(one_lit.clone())),
            span: s.clone(),
            ty: None,
        };

        let condition = Expr {
            kind: ExprKind::Binary(
                Box::new(Expr {
                    kind: ExprKind::Binary(
                        Box::new(ident_expr.clone()),
                        BinaryOp::Add,
                        Box::new(one_lit.clone()),
                    ),
                    span: s.clone(),
                    ty: None,
                }),
                BinaryOp::Lt,
                Box::new(end),
            ),
            span: s.clone(),
            ty: None,
        };

        Stmt {
            kind: StmtKind::Block(vec![
                Stmt {
                    kind: StmtKind::Let(ident, Some(i_type), Some(init_val)),
                    span: s.clone(),
                },
                Stmt {
                    kind: StmtKind::While(
                        condition,
                        Box::new(Stmt {
                            kind: StmtKind::Block(loop_body),
                            span: s.clone(),
                        }),
                    ),
                    span: s.clone(),
                },
            ]),
            span: s.clone(),
        }
    }

    pub(crate) fn desugar_for_count(&mut self, end: Expr, mut body: Vec<Stmt>) -> Stmt {
        let s = end.span.clone();

        let ident = self.get_unique_identifier();
        let end_ident = self.get_unique_identifier();
        let i_type = Type::I64;

        let ident_expr = Expr {
            kind: ExprKind::Ident(vec![ident.clone()]),
            span: s.clone(),
            ty: None,
        };

        let end_ident_expr = Expr {
            kind: ExprKind::Ident(vec![end_ident.clone()]),
            span: s.clone(),
            ty: None,
        };

        let one_lit = Expr {
            kind: ExprKind::Lit(Lit::Int(1)),
            span: s.clone(),
            ty: Some(Type::I64),
        };

        let inc = Stmt {
            kind: StmtKind::Assign(
                ident_expr.clone(),
                Expr {
                    kind: ExprKind::Binary(
                        Box::new(ident_expr.clone()),
                        BinaryOp::Add,
                        Box::new(one_lit.clone()),
                    ),
                    span: s.clone(),
                    ty: None,
                },
            ),
            span: s.clone(),
        };

        let mut loop_body = vec![inc];
        loop_body.append(&mut body);

        let condition = Expr {
            kind: ExprKind::Binary(
                Box::new(Expr {
                    kind: ExprKind::Binary(
                        Box::new(ident_expr.clone()),
                        BinaryOp::Add,
                        Box::new(one_lit.clone()),
                    ),
                    span: s.clone(),
                    ty: None,
                }),
                BinaryOp::Lt,
                Box::new(end_ident_expr),
            ),
            span: s.clone(),
            ty: None,
        };

        Stmt {
            kind: StmtKind::Block(vec![
                Stmt {
                    kind: StmtKind::Let(
                        ident,
                        Some(i_type.clone()),
                        Some(Expr {
                            kind: ExprKind::Lit(Lit::Int(-1)),
                            span: s.clone(),
                            ty: Some(Type::I64),
                        }),
                    ),
                    span: s.clone(),
                },
                Stmt {
                    kind: StmtKind::Let(end_ident, Some(i_type), Some(end)),
                    span: s.clone(),
                },
                Stmt {
                    kind: StmtKind::While(
                        condition,
                        Box::new(Stmt {
                            kind: StmtKind::Block(loop_body),
                            span: s.clone(),
                        }),
                    ),
                    span: s.clone(),
                },
            ]),
            span: s.clone(),
        }
    }
}
