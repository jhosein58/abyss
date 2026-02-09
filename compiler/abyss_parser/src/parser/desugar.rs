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
        let col_var = self.get_unique_identifier();

        let reset = Stmt::Expr(Expr::MethodCall(
            Box::new(Expr::Ident(vec![col_var.clone()])),
            "reset_cursor".into(),
            vec![],
            vec![],
        ));

        let has_next = Expr::MethodCall(
            Box::new(Expr::Ident(vec![col_var.clone()])),
            "has_next".into(),
            vec![],
            vec![],
        );

        let item_decl = Stmt::Let(
            item_name,
            Some(item_type),
            Some(Expr::MethodCall(
                Box::new(Expr::Ident(vec![col_var.clone()])),
                "bump".into(),
                vec![],
                vec![],
            )),
        );

        let mut loop_body = vec![item_decl];
        loop_body.append(&mut body);

        Stmt::Block(vec![
            Stmt::Let(col_var, None, Some(collection)),
            reset,
            Stmt::While(has_next, Box::new(Stmt::Block(loop_body))),
        ])
    }

    pub(crate) fn desugar_for_range(
        &mut self,
        ident: String,
        start: Expr,
        end: Expr,
        mut body: Vec<Stmt>,
    ) -> Stmt {
        let i_type = Type::I64;

        let inc = Stmt::Assign(
            Expr::Ident(vec![ident.clone()]),
            Expr::Binary(
                Box::new(Expr::Ident(vec![ident.clone()])),
                BinaryOp::Add,
                Box::new(Expr::Lit(Lit::Int(1))),
            ),
        );

        let mut loop_body = vec![inc];
        loop_body.append(&mut body);

        let init_val = Expr::Binary(
            Box::new(start),
            BinaryOp::Sub,
            Box::new(Expr::Lit(Lit::Int(1))),
        );

        let condition = Expr::Binary(
            Box::new(Expr::Binary(
                Box::new(Expr::Ident(vec![ident.clone()])),
                BinaryOp::Add,
                Box::new(Expr::Lit(Lit::Int(1))),
            )),
            BinaryOp::Lt,
            Box::new(end),
        );

        Stmt::Block(vec![
            Stmt::Let(ident, Some(i_type), Some(init_val)),
            Stmt::While(condition, Box::new(Stmt::Block(loop_body))),
        ])
    }

    pub(crate) fn desugar_for_count(&mut self, end: Expr, mut body: Vec<Stmt>) -> Stmt {
        let ident = self.get_unique_identifier();
        let end_ident = self.get_unique_identifier();
        let i_type = Type::I64;

        let inc = Stmt::Assign(
            Expr::Ident(vec![ident.clone()]),
            Expr::Binary(
                Box::new(Expr::Ident(vec![ident.clone()])),
                BinaryOp::Add,
                Box::new(Expr::Lit(Lit::Int(1))),
            ),
        );

        let mut loop_body = vec![inc];
        loop_body.append(&mut body);

        let condition = Expr::Binary(
            Box::new(Expr::Binary(
                Box::new(Expr::Ident(vec![ident.clone()])),
                BinaryOp::Add,
                Box::new(Expr::Lit(Lit::Int(1))),
            )),
            BinaryOp::Lt,
            Box::new(Expr::Ident(vec![end_ident.clone()])),
        );

        Stmt::Block(vec![
            Stmt::Let(ident, Some(i_type.clone()), Some(Expr::Lit(Lit::Int(-1)))),
            Stmt::Let(end_ident, Some(i_type), Some(end)),
            Stmt::While(condition, Box::new(Stmt::Block(loop_body))),
        ])
    }
}
