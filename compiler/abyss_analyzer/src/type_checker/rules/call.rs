use abyss_diagnostics::Span;
use abyss_parser::ast::Expr;

use crate::type_checker::{
    engine::{TypeChecker, error_expr},
    tast::{TypedExpr, TypedExprKind},
    types::Type,
};

pub fn check_call(
    tc: &mut TypeChecker,
    calle: &Box<Expr>,
    args: &Vec<Expr>,
    span: Span,
    id: u32,
) -> TypedExpr {
    let checked_calle = tc.check_expr(&calle);

    if let Type::Signature(_, ret_ty) = checked_calle.ty.clone() {
        let mut new_args = Vec::with_capacity(args.len());

        for a in args.iter() {
            new_args.push(tc.check_expr(a));
        }

        return TypedExpr {
            kind: TypedExprKind::Call(Box::new(checked_calle), new_args),
            ty: *ret_ty,
            span,
            id,
        };
    }

    tc.report_error(
        calle.span_expr(),
        format!(
            "Only Signatures can be called. found '{}'.",
            checked_calle.ty.name()
        ),
    );
    error_expr(span, id)
}
