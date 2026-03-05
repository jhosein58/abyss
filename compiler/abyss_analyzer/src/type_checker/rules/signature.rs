use abyss_diagnostics::Span;
use abyss_parser::ast::Expr;

use crate::type_checker::{
    engine::{TypeChecker, error_expr},
    tast::{TypedExpr, TypedExprKind},
    types::Type,
};

pub fn check_signature(
    tc: &mut TypeChecker,
    _args: &Vec<Expr>,
    ret_ty: &Option<Box<Expr>>,
    body: &Box<Expr>,
    span: Span,
    id: u32,
) -> TypedExpr {
    let ty = if let Some(t) = ret_ty {
        let checkd_ret_type = tc.check_expr(t);
        (checkd_ret_type.ty.clone(), checkd_ret_type.span_expr())
    } else {
        (Type::Unit, Span::empty())
    };

    if ty.0 == Type::Error {
        tc.report_error(ty.1, format!("Error finding return type."));
        return error_expr(span, id);
    }

    let checked_args = Vec::new();

    // TODO: check args        (ident: type, ...): type { }
    let checked_body = tc.check_expr(&body);

    TypedExpr {
        kind: TypedExprKind::Signature(checked_args, ty.0.clone(), Box::new(checked_body)),
        ty: Type::Signature(vec![], Box::new(ty.0)),
        span,
        id,
    }
}
