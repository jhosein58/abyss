use abyss_diagnostics::Span;
use abyss_parser::ast::Expr;

use crate::type_checker::{
    engine::TypeChecker,
    tast::{TypedExpr, TypedExprKind},
    types::Type,
};

pub fn check_block(checker: &mut TypeChecker, stmts: &[Expr], span: Span, id: u32) -> TypedExpr {
    let mut typed_stmts = Vec::with_capacity(stmts.len());

    checker.ctx.enter_scope();
    for stmt in stmts {
        typed_stmts.push(checker.check_expr(stmt));
    }
    checker.ctx.exit_scope();

    let ty = if let Some(last_stmt) = typed_stmts.last() {
        last_stmt.ty.clone()
    } else {
        Type::Unit
    };

    TypedExpr {
        kind: TypedExprKind::Block(typed_stmts),
        ty,
        span,
        id,
    }
}
