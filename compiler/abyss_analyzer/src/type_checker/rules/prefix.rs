use abyss_diagnostics::Span;
use abyss_parser::ast::Expr;

use crate::type_checker::{
    engine::TypeChecker,
    tast::{TypedExpr, TypedExprKind},
    types::Type,
};

pub fn check_ret(tc: &mut TypeChecker, val: &Option<Box<Expr>>, span: Span, id: u32) -> TypedExpr {
    let (checked_val, ret_ty) = match val {
        Some(inner_expr) => {
            let checked = tc.check_expr(inner_expr);
            let ty = checked.ty.clone();
            (Some(Box::new(checked)), ty)
        }
        None => (None, Type::Unit),
    };

    TypedExpr {
        kind: TypedExprKind::Ret(checked_val),
        ty: ret_ty,
        span: span,
        id: id,
    }
}
