use abyss_diagnostics::Span;

use crate::type_checker::engine::{TypeChecker, error_expr};
use abyss_types::{
    tast::{TypedExpr, TypedExprKind},
    types::Type,
};

pub fn check_ident(tc: &mut TypeChecker, name: String, span: Span, id: u32) -> TypedExpr {
    // =====================================
    println!(">> [CHECK_IDENT] Looking for: {}", name);
    // =====================================
    if let Some(info) = tc.ctx.lookup(&name).cloned() {
        let ty = info.ty.clone();

        if ty == Type::Infer && tc.resolver.contains(&name) {
            if let Some(resolved_ty) = tc.resolve_global(&name, span.clone()) {
                let is_foldable = tc
                    .resolver
                    .get_metadata(&name)
                    .map(|meta| meta.is_foldable)
                    .unwrap_or(false);

                tc.side_table.mark_const(id, is_foldable);
                return tc.create_ident_expr(info.ir_name.clone(), resolved_ty, span, id);
            }
        }

        if ty == Type::Metatype {
            if let Some(actual_ty) = tc.type_registry.get(&name) {
                tc.side_table.mark_const(id, true);
                return TypedExpr {
                    kind: TypedExprKind::Type(actual_ty.clone()),
                    ty: Type::Metatype,
                    span,
                    id,
                };
            }
        }

        tc.side_table.mark_const(id, info.is_foldable);
        return tc.create_ident_expr(info.ir_name.clone(), ty, span, id);
    }

    if tc.resolver.contains(&name) {
        if let Some(resolved_ty) = tc.resolve_global(&name, span.clone()) {
            let is_foldable = tc
                .resolver
                .get_metadata(&name)
                .map(|meta| meta.is_foldable)
                .unwrap_or(false);

            tc.side_table.mark_const(id, is_foldable);
            return tc.create_ident_expr(name, resolved_ty, span, id);
        }
    }

    if let Some(ty) = tc.primitive_type_from_name(&name) {
        tc.side_table.mark_const(id, true);
        return TypedExpr {
            kind: TypedExprKind::Type(ty.clone()),
            ty: Type::Metatype,
            span,
            id,
        };
    }

    tc.side_table.mark_const(id, false);
    tc.report_error(span.clone(), format!("Undefined symbol: '{}'", name));
    error_expr(span, id)
}
