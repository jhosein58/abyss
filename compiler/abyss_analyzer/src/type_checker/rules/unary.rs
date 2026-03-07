use crate::type_checker::{
    engine::{TypeChecker, error_expr},
    tast::{TypedExpr, TypedExprKind},
    types::Type,
};
use abyss_diagnostics::Span;
use abyss_parser::ast::{Expr, ExprKind, UnaryOp};

pub fn check_unary(
    tc: &mut TypeChecker,
    op: UnaryOp,
    inner_expr: &Expr,
    span: Span,
    id: u32,
) -> TypedExpr {
    let typed_inner = tc.check_expr(inner_expr);
    let inner_ty = typed_inner.ty.clone();

    if inner_ty == Type::Error {
        return error_expr(span, id);
    }

    let (result_ty, valid) = match op {
        // -x
        UnaryOp::Neg => {
            if is_numeric(&inner_ty) {
                (inner_ty, true)
            } else {
                tc.report_error(
                    span.clone(),
                    format!(
                        "Type mismatch: cannot apply unary operator '-' to type '{}'",
                        inner_ty.name()
                    ),
                );
                (Type::Error, false)
            }
        }

        // not x
        UnaryOp::Not => {
            if inner_ty == Type::Bool {
                (Type::Bool, true)
            } else {
                tc.report_error(
                    span.clone(),
                    format!(
                        "Type mismatch: logical 'not' requires a 'bool', found '{}'",
                        inner_ty.name()
                    ),
                );
                (Type::Error, false)
            }
        }

        // ~x
        UnaryOp::BitNot => {
            if is_integer(&inner_ty) {
                (inner_ty, true)
            } else {
                tc.report_error(
                    span.clone(),
                    format!(
                        "Type mismatch: bitwise '~' requires an integer, found '{}'",
                        inner_ty.name()
                    ),
                );
                (Type::Error, false)
            }
        }

        // &x
        UnaryOp::AddrOf => {
            if !is_lvalue(&inner_expr.kind) {
                tc.report_error(
                    inner_expr.span_expr(),
                    "Cannot take the address of an r-value. Expected a variable or memory location.".to_string(),
                );
                (Type::Error, false)
            } else {
                (Type::Ptr(Box::new(inner_ty)), true)
            }
        }

        // *x
        UnaryOp::Deref => {
            if let Type::Ptr(pointed_ty) = inner_ty {
                (*pointed_ty, true)
            } else {
                tc.report_error(
                    span.clone(),
                    format!(
                        "Type mismatch: cannot dereference type '{}'. Expected a pointer.",
                        inner_ty.name()
                    ),
                );
                (Type::Error, false)
            }
        }
    };

    if !valid {
        return error_expr(span, id);
    }

    TypedExpr {
        kind: TypedExprKind::Unary(op, Box::new(typed_inner)),
        ty: result_ty,
        span,
        id,
    }
}

fn is_numeric(t: &Type) -> bool {
    matches!(t, Type::I32 | Type::F32)
}

fn is_integer(t: &Type) -> bool {
    matches!(t, Type::I32)
}

fn is_lvalue(kind: &ExprKind) -> bool {
    matches!(
        kind,
        ExprKind::Ident(_)
            | ExprKind::Index(_, _)
            | ExprKind::Member(_, _)
            | ExprKind::Unary(UnaryOp::Deref, _)
    )
}
