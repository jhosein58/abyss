use abyss_diagnostics::Span;
use abyss_parser::ast::{Expr, Lit, OrderedFloat};
use abyss_types::{
    tast::{TypedExpr, TypedExprKind},
    types::Type,
};

use crate::type_checker::engine::{TypeChecker, error_expr};

pub fn check_cast<'a>(
    tc: &mut TypeChecker<'a>,
    left_expr: &'a Expr,
    right_expr: &'a Expr,
    span: Span,
    id: u32,
) -> TypedExpr {
    let typed_left = tc.check_expr(left_expr);
    if typed_left.ty == Type::Error {
        return error_expr(span, id);
    }

    let typed_right = tc.check_expr(right_expr);
    let target_ty = extract_type_for_cast(tc, &typed_right);

    if target_ty == Type::Error {
        tc.report_error(
            right_expr.span_expr(),
            "Invalid target type for cast operation.".to_string(),
        );
        return error_expr(span, id);
    }

    if !is_cast_valid(&typed_left.ty, &target_ty) {
        tc.report_error(
            span.clone(),
            format!(
                "Invalid cast: cannot cast expression of type '{}' to '{}'",
                typed_left.ty.name(),
                target_ty.name()
            ),
        );
        return error_expr(span, id);
    }

    if is_alias_cast(&typed_left.ty, &target_ty) {
        let is_const = tc.side_table.is_const(typed_left.id);
        tc.side_table.mark_const(id, is_const);
        return TypedExpr {
            kind: typed_left.kind,
            ty: target_ty,
            span,
            id,
        };
    }

    if let TypedExprKind::Lit(ref lit) = typed_left.kind {
        if let Some(new_lit) = cast_literal_value(lit, &target_ty) {
            tc.side_table.mark_const(id, true);

            return TypedExpr {
                kind: TypedExprKind::Lit(new_lit),
                ty: target_ty,
                span,
                id,
            };
        }
    }

    let cast_expr = TypedExpr {
        kind: TypedExprKind::Cast(Box::new(typed_left.clone()), Box::new(typed_right.clone())),
        ty: target_ty.clone(),
        span: span.clone(),
        id,
    };

    let is_const = tc.side_table.is_const(typed_left.id);
    let should_fold = tc.side_table.should_fold(typed_left.id);

    if is_const && should_fold {
        let mut folded_expr = tc.comptime.evaluate_expr(cast_expr);

        folded_expr.id = id;
        folded_expr.span = span;

        tc.side_table.mark_const(id, true);
        return folded_expr;
    }

    cast_expr
}

fn is_alias_cast(from: &Type, to: &Type) -> bool {
    if let Type::Alias(_, inner) = to {
        return from.is_equal(inner);
    }
    false
}

fn extract_type_for_cast(tc: &mut TypeChecker, expr: &TypedExpr) -> Type {
    match &expr.kind {
        TypedExprKind::Type(ty) => ty.clone(),
        TypedExprKind::Ident(name) => {
            if let Some(ty) = tc.type_registry.get(name) {
                return ty.clone();
            }
            if let Some(ty) = tc.primitive_type_from_name(name) {
                return ty;
            }
            if let Some(_) = tc.resolve_global(name, expr.span.clone()) {
                if let Some(ty) = tc.type_registry.get(name) {
                    return ty.clone();
                }
            }
            Type::Error
        }
        _ => tc.evaluate_as_type(expr.clone()),
    }
}

fn is_cast_valid(from: &Type, to: &Type) -> bool {
    if from == to {
        return true;
    }

    if let Type::Alias(_, inner_ty) = to {
        if from.is_equal(inner_ty) {
            return true;
        }
    }

    match (from, to) {
        (Type::I32, Type::F32) | (Type::F32, Type::I32) => true,
        (Type::I32, Type::Bool) | (Type::Bool, Type::I32) => true,
        (Type::Char, Type::I32) | (Type::I32, Type::Char) => true,

        _ => false,
    }
}

fn cast_literal_value(lit: &Lit, target_ty: &Type) -> Option<Lit> {
    let base_ty = target_ty.underlying_type();

    match (lit, base_ty) {
        (Lit::Int(_), Type::I32) | (Lit::Float(_), Type::F32) | (Lit::Bool(_), Type::Bool) => {
            Some(lit.clone())
        }

        // Int Casts
        (Lit::Int(val), Type::F32) => Some(Lit::Float(OrderedFloat(*val as f64))),
        (Lit::Int(val), Type::Bool) => Some(Lit::Bool(*val != 0)),

        // Float Casts
        (Lit::Float(val), Type::I32) => Some(Lit::Int(val.0 as i64)),

        // Bool Casts
        (Lit::Bool(val), Type::I32) => Some(Lit::Int(if *val { 1 } else { 0 })),

        (Lit::Char(val), Type::I32) => Some(Lit::Int(*val as i64)),
        (Lit::Int(val), Type::Char) => {
            if let Some(c) = char::from_u32(*val as u32) {
                Some(Lit::Char(c))
            } else {
                None
            }
        }

        _ => None,
    }
}
