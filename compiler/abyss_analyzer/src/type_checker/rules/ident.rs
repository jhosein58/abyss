use abyss_diagnostics::Span;

use crate::type_checker::engine::{TypeChecker, error_expr};
use abyss_types::{
    tast::{TypedExpr, TypedExprKind},
    types::Type,
};

pub fn check_ident(tc: &mut TypeChecker, name: String, span: Span, id: u32) -> TypedExpr {
    if let Some(info) = tc.ctx.lookup(&name) {
        if info.is_constant() {
            tc.side_table.mark_const(id, false);

            if info.is_inline {
                tc.side_table.mark_const(id, true);
            }
        }

        let ty = info.ty.clone();

        if ty == Type::Infer && tc.resolver.contains(&name) {
            if let Some(resolved_ty) = tc.resolve_global(&name, span.clone()) {
                return tc.create_ident_expr(name, resolved_ty, span, id);
            }
        }

        if ty == Type::Metatype {
            if let Some(actual_ty) = tc.type_registry.get(&name) {
                return TypedExpr {
                    kind: TypedExprKind::Type(actual_ty.clone()),
                    ty: Type::Metatype,
                    span,
                    id,
                };
            }
        }

        return tc.create_ident_expr(name, ty, span, id);
    }

    if tc.resolver.contains(&name) {
        if let Some(resolved_ty) = tc.resolve_global(&name, span.clone()) {
            return tc.create_ident_expr(name, resolved_ty, span, id);
        }
    }

    if let Some(ty) = tc.primitive_type_from_name(&name) {
        return TypedExpr {
            kind: TypedExprKind::Type(ty.clone()),
            ty: Type::Metatype,
            span,
            id,
        };
    }

    tc.report_error(span.clone(), format!("Undefined symbol: '{}'", name));
    error_expr(span, id)
}
