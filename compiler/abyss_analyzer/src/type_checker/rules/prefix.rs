use crate::type_checker::{
    context::SymbolInfo,
    engine::{TypeChecker, error_expr},
    method_registry::MethodRegistry,
    resolver::{GlobalMetadata, InlinePolicy},
    rules::signature::check_signature,
};
use abyss_diagnostics::Span;
use abyss_parser::ast::{Expr, ExprKind};
use abyss_types::{
    tast::{TypedExpr, TypedExprKind},
    types::Type,
};

pub fn check_ret<'a>(
    tc: &mut TypeChecker<'a>,
    val: &'a Option<Box<Expr>>,
    span: Span,
    id: u32,
) -> TypedExpr {
    let (checked_val, ret_ty) = match val {
        Some(inner_expr) => {
            let checked = tc.check_expr(inner_expr);
            let ty = checked.ty.clone();
            (Some(Box::new(checked)), ty)
        }
        None => (None, Type::Unit),
    };

    TypedExpr {
        kind: TypedExprKind::Ret(checked_val),
        ty: ret_ty,
        span,
        id,
    }
}

pub fn check_def<'a>(
    tc: &mut TypeChecker<'a>,
    name_expr: &'a Expr,
    value_expr: &'a Expr,
    span: Span,
    id: u32,
) -> TypedExpr {
    let name = match &name_expr.kind {
        ExprKind::Ident(n) => n.clone(),

        ExprKind::Member(base, field_name) => {
            let type_name = if let ExprKind::Ident(n) = &base.kind {
                let base_ty = tc.resolve_type_by_name(n, base.span_expr());
                if base_ty == Type::Error {
                    return error_expr(span, id);
                }
                n.clone()
            } else {
                let typed_base = tc.check_expr(base);
                let base_ty = tc.evaluate_as_type(typed_base);
                if base_ty == Type::Error {
                    return error_expr(span, id);
                }
                base_ty.mangled_name()
            };

            MethodRegistry::mangle_method_name(&type_name, field_name)
        }

        _ => {
            tc.report_error(
                name_expr.span.clone(),
                "Definition target must be an identifier or a `Type.method` expression."
                    .to_string(),
            );
            return error_expr(span, id);
        }
    };

    if let ExprKind::Signature(ref args, ref ret, ref body) = value_expr.kind {
        return check_signature(
            tc,
            args,
            ret,
            body,
            Some(name),
            value_expr.span.clone(),
            value_expr.id,
        );
    }

    if tc.resolver.is_resolved(&name) {
        if let Some(ty) = tc.resolver.get_resolved_type(&name) {
            if let Some(resolved_expr) = tc.resolver.get_resolved_expr(&name) {
                return TypedExpr {
                    kind: TypedExprKind::Def(name, Box::new(resolved_expr.clone())),
                    ty,
                    span,
                    id,
                };
            }
        }
    }

    let mut typed_value = tc.check_expr(value_expr);
    let mut final_ty = typed_value.ty.clone();
    let mut is_type_def = false;

    let base_is_foldable = tc.side_table.should_fold(typed_value.id);

    let mut final_is_foldable = base_is_foldable;
    let mut final_inline_policy = InlinePolicy::Never;

    if let Some(inline_attr) = tc.get_attribute("inline") {
        if inline_attr.is_empty_args() || inline_attr.has_exact_single_arg("Always") {
            final_is_foldable = true;
            final_inline_policy = InlinePolicy::Always;
        } else if inline_attr.has_exact_single_arg("Prefer") {
            final_is_foldable = base_is_foldable;
            final_inline_policy = InlinePolicy::Prefer;
        } else if inline_attr.has_exact_single_arg("Never") {
            final_is_foldable = false;
            final_inline_policy = InlinePolicy::Never;
        } else {
            tc.report_error(
                inline_attr.span.clone(),
                "Invalid arguments for #inline attribute. Expected 'Always', 'Prefer', or 'Never'."
                    .to_string(),
            );
        }
    }

    if typed_value.ty == Type::Metatype {
        let base_type = tc.evaluate_as_type(typed_value.clone());
        if !matches!(base_type, Type::Alias(_, _)) {
            let alias_type = Type::Alias(name.clone(), Box::new(base_type.clone()));
            typed_value.kind = TypedExprKind::Type(alias_type.clone());
            tc.type_registry.register(name.clone(), alias_type);
        } else {
            tc.type_registry.register(name.clone(), base_type);
        }
        is_type_def = true;
        final_ty = Type::Metatype;
    }

    tc.ctx.update_type(&name, final_ty.clone());

    if tc.resolver.contains(&name) {
        let metadata = GlobalMetadata {
            inline_policy: final_inline_policy,
            is_foldable: final_is_foldable,
        };
        tc.complete_and_register_global(
            name.clone(),
            final_ty.clone(),
            typed_value.clone(),
            is_type_def,
            metadata,
        );

        tc.ctx.define_global(
            name.clone(),
            SymbolInfo::constant(final_ty.clone(), final_is_foldable),
        );
    } else if !tc.ctx.is_global_scope() {
        tc.ctx.define(
            name.clone(),
            SymbolInfo::constant(final_ty.clone(), final_is_foldable),
        );
    }

    tc.side_table.mark_const(id, final_is_foldable);

    TypedExpr {
        kind: TypedExprKind::Def(name, Box::new(typed_value)),
        ty: final_ty,
        span,
        id,
    }
}

pub fn check_cmpt<'a>(
    tc: &mut TypeChecker<'a>,
    inner_expr: &'a Expr,
    span: Span,
    id: u32,
) -> TypedExpr {
    let typed_inner = tc.check_expr(inner_expr);

    if typed_inner.ty == Type::Error {
        return error_expr(span, id);
    }

    let mut evaluated_expr = tc.comptime.evaluate_expr(typed_inner);

    evaluated_expr.span = span;
    evaluated_expr.id = id;

    tc.side_table.mark_const(id, true);

    evaluated_expr
}
