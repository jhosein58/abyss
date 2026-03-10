use abyss_diagnostics::Span;
use abyss_parser::ast::Expr;

use crate::type_checker::engine::{TypeChecker, error_expr};
use abyss_types::{
    tast::{TypedExpr, TypedExprKind},
    types::Type,
};

pub fn check_if(
    tc: &mut TypeChecker,
    cond: &Box<Expr>,
    then_b: &Box<Expr>,
    else_b: &Option<Box<Expr>>,
    span: Span,
    id: u32,
) -> TypedExpr {
    let checked_cond = tc.check_expr(cond);

    if checked_cond.ty != Type::Bool {
        tc.report_error(
            checked_cond.span.clone(),
            format!(
                "If condition must be a boolean, but got '{}'",
                checked_cond.ty.name()
            ),
        );
        return error_expr(span, id);
    }

    let checked_then = tc.check_expr(then_b);

    match else_b {
        Some(else_expr) => {
            let checked_else = tc.check_expr(else_expr);

            if checked_then.ty != checked_else.ty {
                tc.report_error_with_hint(
                    span.clone().merge(checked_else.span_expr()),
                    format!("'if' and 'else' have incompatible types"),
                    format!(
                        "The 'if' branch has type '{}', but the 'else' branch has type '{}'",
                        checked_then.ty.name(),
                        checked_else.ty.name()
                    ),
                );

                TypedExpr {
                    kind: TypedExprKind::If(
                        Box::new(checked_cond),
                        Box::new(checked_then),
                        Some(Box::new(checked_else)),
                    ),
                    ty: Type::Error,
                    span,
                    id,
                }
            } else {
                let overall_type = checked_then.ty.clone();
                TypedExpr {
                    kind: TypedExprKind::If(
                        Box::new(checked_cond),
                        Box::new(checked_then),
                        Some(Box::new(checked_else)),
                    ),
                    ty: overall_type,
                    span,
                    id,
                }
            }
        }

        None => TypedExpr {
            kind: TypedExprKind::If(Box::new(checked_cond), Box::new(checked_then), None),
            ty: Type::Unit,
            span,
            id,
        },
    }
}
