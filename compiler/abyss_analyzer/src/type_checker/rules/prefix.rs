use crate::type_checker::{
    context::SymbolInfo,
    engine::{TypeChecker, error_expr},
    resolver::InlinePolicy,
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
        _ => {
            tc.report_error(
                name_expr.span_expr(),
                "Definition name must be an identifier.".to_string(),
            );
            return error_expr(span, id);
        }
    };

    let c = tc.ctx.lookup(&name).unwrap();
    println!("{:?}", c);

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
            return TypedExpr {
                kind: TypedExprKind::Ident(name),
                ty,
                span,
                id,
            };
        }
    }

    let mut typed_value = tc.check_expr(value_expr);
    let mut final_ty = typed_value.ty.clone();
    let mut is_type_def = false;

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
        tc.complete_and_register_global(
            name.clone(),
            final_ty.clone(),
            typed_value.clone(),
            is_type_def,
            InlinePolicy::Never,
        );
    } else {
        if !tc.ctx.is_global_scope() {
            tc.ctx
                .define(name.clone(), SymbolInfo::constant(final_ty.clone(), true));
        }
    }

    tc.side_table.mark_const(id, true);

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
