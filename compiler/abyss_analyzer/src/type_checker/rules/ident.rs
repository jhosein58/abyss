use abyss_diagnostics::Span;

use crate::type_checker::engine::{TypeChecker, error_expr};
use abyss_types::{
    tast::{TypedExpr, TypedExprKind},
    types::Type,
};

pub fn check_ident(tc: &mut TypeChecker, name: String, span: Span, id: u32) -> TypedExpr {
    if let Some(builtin_type) = get_builtin_type(&name) {
        return TypedExpr {
            id,
            span,
            ty: Type::Metatype,
            kind: TypedExprKind::Type(builtin_type),
        };
    }

    if let Some(symbol) = tc.ctx.lookup(&name) {
        if symbol.ty == Type::Metatype {
            // TODO: get real type in comptime
        }

        return TypedExpr {
            id,
            span,
            ty: symbol.ty.clone(),
            kind: TypedExprKind::Ident(name),
        };
    }

    tc.report_error(span.clone(), format!("Undefined identifier: '{}'", name));
    error_expr(span, id)
}

fn get_builtin_type(name: &str) -> Option<Type> {
    match name {
        "i32" => Some(Type::I32),
        "f32" => Some(Type::F32),
        "bool" => Some(Type::Bool),
        "str" => Some(Type::Str),
        "unit" => Some(Type::Unit),
        "type" => Some(Type::Metatype),
        _ => None,
    }
}
