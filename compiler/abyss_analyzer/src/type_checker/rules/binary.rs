use crate::type_checker::{
    engine::{TypeChecker, error_expr},
    tast::{TypedExpr, TypedExprKind},
    types::Type,
};
use abyss_diagnostics::Span;
use abyss_parser::ast::{BinaryOp, Expr, ExprKind};

pub fn check_binary(
    tc: &mut TypeChecker,
    left_expr: &Expr,
    op: BinaryOp,
    right_expr: &Expr,
    span: Span,
    id: u32,
) -> TypedExpr {
    let typed_right = tc.check_expr(right_expr);
    let right_ty = typed_right.ty.clone();

    if op == BinaryOp::Assign {
        match left_expr.kind {
            ExprKind::Binary(ref var, ref o, ref ty) => {
                if *o == BinaryOp::KeyValue {
                    return check_var_dec(tc, var, ty, typed_right, span, id);
                } else {
                    let s = left_expr.span.clone().merge(ty.span.clone());
                    tc.report_error(s, format!("Expected 'pattern := expr' or 'pattern: type = expr' syntax, found 'pattern {} epxr = expr'",  o));

                    return error_expr(span, id);
                }
            }
            _ => {}
        }
    }

    let typed_left = tc.check_expr(left_expr);
    let left_ty = typed_left.ty.clone();

    if left_ty == Type::Error || right_ty == Type::Error {
        return error_expr(span, id);
    }

    let (result_ty, valid) = match op {
        // --- Arithmetic (+, -, *, /, %) ---
        BinaryOp::Add | BinaryOp::Sub | BinaryOp::Mul | BinaryOp::Div | BinaryOp::Mod => {
            if left_ty == right_ty && is_numeric(&left_ty) {
                (left_ty, true)
            } else {
                let s = left_expr.span.clone().merge(right_expr.span.clone());
                tc.report_error(s, format!("Type mismatch: cannot perform arithmetic operation between '{}' and '{}'", left_ty.name(), right_ty.name())
);
                (Type::Error, false)
            }
        }

        // --- Bitwise (&, |, ^, <<, >>) ---
        BinaryOp::BitAnd | BinaryOp::Pipe | BinaryOp::BitXor | BinaryOp::Shl | BinaryOp::Shr => {
            if left_ty == right_ty && is_integer(&left_ty) {
                (left_ty, true)
            } else {

                let s = left_expr.span.clone().merge(right_expr.span.clone());

                tc.report_error(s, format!("Type mismatch: cannot perform arithmetic operation between '{}' and '{}'", left_ty.name(), right_ty.name()));
                (Type::Error, false)
            }
        }

        // --- Comparison (==, !=) ---
        BinaryOp::Eq | BinaryOp::Neq => {
            if left_ty == right_ty {
                (Type::Bool, true)
            } else {
                let s = left_expr.span.clone().merge(right_expr.span.clone());

                tc.report_error(s,  format!("Type mismatch: cannot compare '{}' with '{}'", left_ty.name(), right_ty.name()));
                (Type::Error, false)
            }
        }

        // --- Ordering (<, <=, >, >=) ---
        BinaryOp::Lt | BinaryOp::Lte | BinaryOp::Gt | BinaryOp::Gte => {
            if left_ty == right_ty && is_numeric(&left_ty) {
                (Type::Bool, true)
            } else {
                (Type::Error, false)
            }
        }

        // --- Logical and or) ---
        BinaryOp::And | BinaryOp::Or => {
            if left_ty == Type::Bool && right_ty == Type::Bool {
                (Type::Bool, true)
            } else {

                let s = left_expr.span.clone().merge(right_expr.span.clone());

                tc.report_error(s,  format!("Type mismatch: logical operators require both operands to be 'bool', found '{}' and '{}'", left_ty.name(), right_ty.name()));

                (Type::Error, false)
            }
        }

        // --- Assignment (=) ---
        BinaryOp::Assign => {
            if is_assignable(&left_ty, &right_ty) {
                (Type::Unit, true)
            } else {
                (Type::Error, false)
            }
        }

        // --- Compound Assignment (+=, -=, ...) ---
        BinaryOp::AssignAdd // +=
        | BinaryOp::AssignSub // -=
        | BinaryOp::AssignMul // *=
        | BinaryOp::AssignDiv // /=
        | BinaryOp::AssignMod // %=
        | BinaryOp::AssignBitAnd // &=
        | BinaryOp::AssignBitOr //|=
        | BinaryOp::AssignBitXor // ^=
        | BinaryOp::AssignShl // <<=
        | BinaryOp::AssignShr //>>=
        => {
            if left_ty == right_ty && is_numeric(&left_ty) {
                (Type::Unit, true)
            } else {
                (Type::Error, false)
            }
        }

        // --- Declaration / Annotation (:, ::) ---
        BinaryOp::KeyValue => (left_ty, true),

        BinaryOp::ConstDef => (right_ty, true),

       // _ => (Type::Error, false),
    };

    if !valid {
        return TypedExpr {
            kind: TypedExprKind::ErrorPlaceholder,
            ty: Type::Error,
            span,
            id,
        };
    }

    TypedExpr {
        kind: TypedExprKind::Binary(Box::new(typed_left), op, Box::new(typed_right)),
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

fn is_assignable(target: &Type, source: &Type) -> bool {
    println!("{} = {}", target.name(), source.name());
    if target == source {
        return true;
    }

    false
}

fn check_var_dec(
    tc: &mut TypeChecker,
    pattern: &Box<Expr>,
    ty: &Box<Expr>,
    right: TypedExpr,
    span: Span,
    id: u32,
) -> TypedExpr {
    match pattern.kind {
        ExprKind::Ident(ref n) => {
            let name = n.to_string();
            let init_type = if ty.kind == ExprKind::Wildcard {
                if right.ty == Type::Error {
                    tc.report_error(right.span_expr(), format!("Type resolution failed."));
                    return error_expr(span, id);
                }
                right.ty.clone()
            } else {
                let ty = tc.check_expr(ty);

                if ty.ty == Type::Error {
                    tc.report_error(
                        ty.span_expr(),
                        format!("Type resolution failed for variable '{}'.", name),
                    );
                    return error_expr(span, id);
                }
                ty.ty
            };

            tc.ctx.define(name.clone(), init_type.clone());

            TypedExpr {
                kind: TypedExprKind::VarDec(name, init_type.clone(), Some(Box::new(right))),
                id,
                span,
                ty: init_type,
            }
        }

        _ => {
            tc.report_error(
                pattern.span_expr(),
                format!("LHS must be an identifier. Found pattern.",),
            );
            error_expr(span, id)
        }
    }
}
