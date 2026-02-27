use crate::type_checker::{
    engine::TypeChecker,
    tast::{TypedExpr, TypedExprKind},
    types::Type,
};
use abyss_diagnostics::Span;
use abyss_parser::ast::{BinaryOp, Expr};

pub fn check_binary(
    tc: &mut TypeChecker,
    left_expr: &Expr,
    op: BinaryOp,
    right_expr: &Expr,
    span: Span,
    id: u32,
) -> TypedExpr {
    let typed_left = tc.check_expr(left_expr);
    let typed_right = tc.check_expr(right_expr);

    let left_ty = typed_left.ty.clone();
    let right_ty = typed_right.ty.clone();

    if left_ty == Type::Error || right_ty == Type::Error {
        return error_expr(span, id);
    }

    let (result_ty, valid) = match op {
        // --- Arithmetic (+, -, *, /, %) ---
        BinaryOp::Add | BinaryOp::Sub | BinaryOp::Mul | BinaryOp::Div | BinaryOp::Mod => {
            if left_ty == right_ty && is_numeric(&left_ty) {
                (left_ty, true)
            } else {
                (Type::Error, false)
            }
        }

        // --- Bitwise (&, |, ^, <<, >>) ---
        BinaryOp::BitAnd | BinaryOp::Pipe | BinaryOp::BitXor | BinaryOp::Shl | BinaryOp::Shr => {
            if left_ty == right_ty && is_integer(&left_ty) {
                (left_ty, true)
            } else {
                (Type::Error, false)
            }
        }

        // --- Comparison (==, !=) ---
        BinaryOp::Eq | BinaryOp::Neq => {
            if left_ty == right_ty {
                (Type::Bool, true)
            } else {
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

fn error_expr(span: Span, id: u32) -> TypedExpr {
    TypedExpr {
        kind: TypedExprKind::ErrorPlaceholder,
        ty: Type::Error,
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
    if target == source {
        return true;
    }

    false
}
