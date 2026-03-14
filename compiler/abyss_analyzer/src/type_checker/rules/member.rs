use abyss_diagnostics::Span;
use abyss_parser::ast::Expr;
use abyss_types::{
    tast::{TypedExpr, TypedExprKind},
    types::Type,
};

use crate::type_checker::engine::{TypeChecker, error_expr};

pub fn check_member<'a>(
    tc: &mut TypeChecker<'a>,
    base_expr: &'a Expr,
    field_name: &'a String,
    span: Span,
    id: u32,
) -> TypedExpr {
    let typed_base = tc.check_expr(base_expr);

    if typed_base.ty == Type::Error {
        return error_expr(span, id);
    }

    let actual_type = typed_base.ty.underlying_type();

    match actual_type {
        Type::Struct(fields) => {
            for field in fields {
                if field.name == *field_name {
                    return TypedExpr {
                        kind: TypedExprKind::FieldAccess(Box::new(typed_base), field_name.clone()),
                        ty: field.ty.clone(),
                        span,
                        id,
                    };
                }
            }

            tc.report_error(
                span.clone(),
                format!(
                    "Type '{}' has no field named '{}'.",
                    typed_base.ty.name(),
                    field_name
                ),
            );
            error_expr(span, id)
        }
        _ => {
            tc.report_error(
                span.clone(),
                format!(
                    "Type '{}' is not a struct and has no fields.",
                    typed_base.ty.name()
                ),
            );
            error_expr(span, id)
        }
    }
}
