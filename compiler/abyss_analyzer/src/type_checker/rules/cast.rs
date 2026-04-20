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

fn is_integer_type(ty: &Type) -> bool {
    matches!(
        ty,
        Type::I1
            | Type::I8
            | Type::I16
            | Type::I32
            | Type::I64
            | Type::U8
            | Type::U16
            | Type::U32
            | Type::U64
    )
}

fn is_float_type(ty: &Type) -> bool {
    matches!(ty, Type::F32 | Type::F64)
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

    let from_base = from.underlying_type();
    let to_base = to.underlying_type();

    let from_is_int = is_integer_type(&from_base);
    let to_is_int = is_integer_type(&to_base);
    let from_is_float = is_float_type(&from_base);
    let to_is_float = is_float_type(&to_base);

    // Int <-> Int
    if from_is_int && to_is_int {
        return true;
    }
    // Float <-> Float
    if from_is_float && to_is_float {
        return true;
    }
    // Int <-> Float
    if (from_is_int && to_is_float) || (from_is_float && to_is_int) {
        return true;
    }

    // Int/Float <-> Bool
    if (from_is_int || from_is_float) && to_base == Type::Bool {
        return true;
    }
    if from_base == Type::Bool && (to_is_int || to_is_float) {
        return true;
    }

    // Char <-> Int
    if from_base == Type::Char && to_is_int {
        return true;
    }
    if from_is_int && to_base == Type::Char {
        return true;
    }

    false
}

fn cast_literal_value(lit: &Lit, target_ty: &Type) -> Option<Lit> {
    let base_ty = target_ty.underlying_type();

    let is_target_int = is_integer_type(&base_ty);
    let is_target_float = is_float_type(&base_ty);

    match lit {
        Lit::Int(val) => {
            if is_target_int {
                Some(Lit::Int(*val))
            } else if is_target_float {
                Some(Lit::Float(OrderedFloat(*val as f64)))
            } else if base_ty == Type::Bool {
                Some(Lit::Bool(*val != 0))
            } else if base_ty == Type::Char {
                char::from_u32(*val as u32).map(Lit::Char)
            } else {
                None
            }
        }
        Lit::Float(val) => {
            if is_target_float {
                Some(Lit::Float(*val))
            } else if is_target_int {
                Some(Lit::Int(val.0 as i64))
            } else if base_ty == Type::Bool {
                Some(Lit::Bool(val.0 != 0.0))
            } else {
                None
            }
        }
        Lit::Bool(val) => {
            if base_ty == Type::Bool {
                Some(Lit::Bool(*val))
            } else if is_target_int {
                Some(Lit::Int(if *val { 1 } else { 0 }))
            } else if is_target_float {
                Some(Lit::Float(OrderedFloat(if *val { 1.0 } else { 0.0 })))
            } else {
                None
            }
        }
        Lit::Char(val) => {
            if base_ty == Type::Char {
                Some(Lit::Char(*val))
            } else if is_target_int {
                Some(Lit::Int(*val as i64))
            } else {
                None
            }
        }
        _ => None,
    }
}
