use abyss_diagnostics::Span;

use crate::type_checker::{
    engine::{TypeChecker, error_expr},
    tast::{TypedExpr, TypedExprKind},
    types::Type,
};

pub fn check_ident(tc: &mut TypeChecker, name: String, span: Span, id: u32) -> TypedExpr {
    match name.as_str() {
        "i32" => {
            return TypedExpr {
                id,
                span,
                ty: Type::I32,
                kind: TypedExprKind::Ident(name),
            };
        }

        "f32" => {
            return TypedExpr {
                id,
                span,
                ty: Type::F32,
                kind: TypedExprKind::Ident(name),
            };
        }

        "bool" => {
            return TypedExpr {
                id,
                span,
                ty: Type::Bool,
                kind: TypedExprKind::Ident(name),
            };
        }

        _ => {}
    }

    if let Some(s) = tc.ctx.lookup(&name) {
        return TypedExpr {
            id,
            span,
            ty: s.ty.clone(),
            kind: TypedExprKind::Ident(name),
        };
    }

    error_expr(span, id)
}
