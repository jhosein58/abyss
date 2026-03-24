use abyss_diagnostics::Span;
use abyss_parser::ast::{Expr, UnaryOp};
use abyss_types::{
    tast::{TypedExpr, TypedExprKind},
    types::Type,
};

use crate::type_checker::{
    engine::{TypeChecker, error_expr},
    method_registry::MethodRegistry,
};
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

    if typed_base.ty == Type::Metatype {
        let target_type = tc.evaluate_as_type(typed_base.clone());
        let type_mangled_name = target_type.mangled_name();
        let mangled_method_name =
            MethodRegistry::mangle_method_name(&type_mangled_name, field_name);

        if let Some(func_ty) = tc.resolver.get_resolved_type(&mangled_method_name) {
            return TypedExpr {
                kind: TypedExprKind::Ident(mangled_method_name),
                ty: func_ty,
                span,
                id,
            };
        }

        if tc.resolver.contains(&mangled_method_name) {
            if let Some(func_ty) = tc.resolve_global(&mangled_method_name, span.clone()) {
                return TypedExpr {
                    kind: TypedExprKind::Ident(mangled_method_name),
                    ty: func_ty,
                    span,
                    id,
                };
            }
        }

        tc.report_error(
            span.clone(),
            format!(
                "Static method '{}' not found on type '{}'.",
                field_name,
                target_type.name()
            ),
        );
        return error_expr(span, id);
    }

    let core_type = typed_base.ty.peel_pointers();
    let mut current_lookup_type = core_type.clone();

    loop {
        let type_mangled_name = current_lookup_type.mangled_name();
        let mangled_method_name =
            MethodRegistry::mangle_method_name(&type_mangled_name, field_name);

        if let Some(func_ty) = tc.resolver.get_resolved_type(&mangled_method_name) {
            return TypedExpr {
                kind: TypedExprKind::BoundMethod {
                    receiver: Box::new(typed_base),
                    method_name: mangled_method_name,
                },
                ty: func_ty,
                span,
                id,
            };
        }

        if tc.resolver.contains(&mangled_method_name) {
            if let Some(func_ty) = tc.resolve_global(&mangled_method_name, span.clone()) {
                return TypedExpr {
                    kind: TypedExprKind::BoundMethod {
                        receiver: Box::new(typed_base),
                        method_name: mangled_method_name,
                    },
                    ty: func_ty,
                    span,
                    id,
                };
            }
        }

        if let Type::Alias(_, inner) = current_lookup_type {
            current_lookup_type = *inner;
        } else {
            break;
        }
    }

    if let Type::Struct(ref fields) = current_lookup_type {
        for field in fields {
            if field.name == *field_name {
                let mut derefed_base = typed_base.clone();

                while let Type::Ptr(inner_ty) = derefed_base.ty.clone() {
                    derefed_base = TypedExpr {
                        kind: TypedExprKind::Unary(UnaryOp::Deref, Box::new(derefed_base)),
                        ty: *inner_ty,
                        span: span.clone(),
                        id,
                    };
                }

                return TypedExpr {
                    kind: TypedExprKind::FieldAccess(Box::new(derefed_base), field_name.clone()),
                    ty: field.ty.clone(),
                    span: span.clone(),
                    id,
                };
            }
        }
    }

    if tc.resolver.is_resolved(field_name) {
        if let Some(func_ty) = tc.resolver.get_resolved_type(field_name) {
            return TypedExpr {
                kind: TypedExprKind::BoundMethod {
                    receiver: Box::new(typed_base),
                    method_name: field_name.clone(),
                },
                ty: func_ty,
                span,
                id,
            };
        }
    }

    if tc.resolver.contains(field_name) {
        if let Some(func_ty) = tc.resolve_global(field_name, span.clone()) {
            return TypedExpr {
                kind: TypedExprKind::BoundMethod {
                    receiver: Box::new(typed_base),
                    method_name: field_name.clone(),
                },
                ty: func_ty,
                span,
                id,
            };
        }
    }

    let error_msg = if matches!(current_lookup_type, Type::Struct(_)) {
        format!(
            "Type '{}' has no field, method, or matching global function named '{}'.",
            core_type.name(),
            field_name
        )
    } else {
        format!(
            "Type '{}' has no methods, is not a struct, and no matching global function '{}' was found.",
            core_type.name(),
            field_name
        )
    };

    tc.report_error(span.clone(), error_msg);
    error_expr(span, id)
}
