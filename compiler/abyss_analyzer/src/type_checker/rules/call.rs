use abyss_diagnostics::Span;
use abyss_parser::ast::{Expr, ExprKind};

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
    if let ExprKind::Ident(ref n) = calle.kind {
        let mut new_args = Vec::with_capacity(args.len());

        for a in args.iter() {
            new_args.push(tc.check_expr(a));
        }

        return TypedExpr {
            kind: TypedExprKind::Call(
                Box::new(TypedExpr {
                    kind: TypedExprKind::Ident(n.clone()),
                    ty: Type::Unit,
                    span: calle.span_expr(),
                    id: calle.id,
                }),
                new_args,
            ),
            ty: Type::Unit,
            span,
            id,
        };
    }

    tc.report_error(
        calle.span_expr(),
        format!("Only identifiers can be called."),
    );
    error_expr(span, id)
}
