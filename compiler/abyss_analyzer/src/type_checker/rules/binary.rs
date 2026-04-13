use crate::type_checker::{
    context::{SymbolInfo, SymbolKind},
    engine::{TypeChecker, error_expr},
};
use abyss_types::{
    tast::{TypedExpr, TypedExprKind},
    types::Type,
};

use abyss_diagnostics::Span;
use abyss_parser::ast::{BinaryOp, Expr, ExprKind};

pub fn check_binary<'a>(
    tc: &mut TypeChecker<'a>,
    left_expr: &'a Expr,
    op: BinaryOp,
    right_expr: &'a Expr,
    span: Span,
    id: u32,
) -> TypedExpr {
    if op == BinaryOp::Assign {
        match left_expr.kind {
            // _ = expr (discard)
            ExprKind::Wildcard => {
                let typed_right = tc.check_expr(right_expr);

                let typed_left = TypedExpr {
                    kind: TypedExprKind::Wildcard,
                    ty: Type::Unit,
                    span: left_expr.span.clone(),
                    id: left_expr.id,
                };

                return TypedExpr {
                    kind: TypedExprKind::Binary(Box::new(typed_left), op, Box::new(typed_right)),
                    ty: Type::Unit,
                    span,
                    id,
                };
            }

            // pattern: type = expr (variable declaration)
            ExprKind::Binary(ref var, ref o, ref ty) => {
                if *o == BinaryOp::KeyValue {
                    let typed_right = tc.check_expr(right_expr);
                    return check_var_dec(tc, var, ty, typed_right, span, id);
                } else {
                    let s = left_expr.span.clone().merge(ty.span.clone());
                    tc.report_error(
                        s,
                        "Expected 'pattern := expr' or 'pattern: type = expr' syntax.".to_string(),
                    );
                    return error_expr(span, id);
                }
            }

            _ => {}
        }
    }

    // KeyValue (:)
    if op == BinaryOp::KeyValue {
        let typed_left = tc.check_expr(left_expr);
        let typed_right = tc.check_expr(right_expr);

        if typed_right.ty == Type::Metatype {
            let actual_type = extract_type_from_expr(tc, &typed_right);
            return TypedExpr {
                kind: TypedExprKind::Binary(Box::new(typed_left), op, Box::new(typed_right)),
                ty: actual_type,
                span,
                id,
            };
        }

        return TypedExpr {
            kind: TypedExprKind::Binary(Box::new(typed_left), op, Box::new(typed_right.clone())),
            ty: typed_right.ty,
            span,
            id,
        };
    }

    let typed_right = tc.check_expr(right_expr);
    let right_ty = typed_right.ty.clone();

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
            } else if left_ty == Type::Metatype && right_ty == Type::Metatype {
                (Type::Metatype, true)
            } else {
                let s = left_expr.span.clone().merge(right_expr.span.clone());
                tc.report_error(
                    s,
                    format!(
                        "Type mismatch: cannot perform arithmetic operation between '{}' and '{}'",
                        left_ty.name(),
                        right_ty.name()
                    ),
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
                tc.report_error(
                    s,
                    format!(
                        "Type mismatch: cannot perform bitwise operation between '{}' and '{}'",
                        left_ty.name(),
                        right_ty.name()
                    ),
                );
                (Type::Error, false)
            }
        }

        // --- Comparison (==, !=) ---
        BinaryOp::Eq | BinaryOp::Neq => {
            if left_ty == right_ty {
                (Type::Bool, true)
            } else {
                let s = left_expr.span.clone().merge(right_expr.span.clone());
                tc.report_error(
                    s,
                    format!(
                        "Type mismatch: cannot compare '{}' with '{}'",
                        left_ty.name(),
                        right_ty.name()
                    ),
                );
                (Type::Error, false)
            }
        }

        // --- Ordering (<, <=, >, >=) ---
        BinaryOp::Lt | BinaryOp::Lte | BinaryOp::Gt | BinaryOp::Gte => {
            if left_ty == right_ty && is_numeric(&left_ty) {
                (Type::Bool, true)
            } else {
                let s = left_expr.span.clone().merge(right_expr.span.clone());
                tc.report_error(
                    s,
                    format!(
                        "Type mismatch: cannot compare '{}' with '{}'",
                        left_ty.name(),
                        right_ty.name()
                    ),
                );
                (Type::Error, false)
            }
        }

        // --- Logical (and, or) ---
        BinaryOp::And | BinaryOp::Or => {
            if left_ty == Type::Bool && right_ty == Type::Bool {
                (Type::Bool, true)
            } else {
                let s = left_expr.span.clone().merge(right_expr.span.clone());
                tc.report_error(
                    s,
                    format!(
                        "Type mismatch: logical operators require 'bool', found '{}' and '{}'",
                        left_ty.name(),
                        right_ty.name()
                    ),
                );
                (Type::Error, false)
            }
        }

        // --- Assignment (=) ---
        BinaryOp::Assign => {
            if is_assignable(tc, &left_ty, &right_ty) {
                if let ExprKind::Ident(ref original_name) = left_expr.kind {
                    if let Err(e) = tc.ctx.assign(original_name) {
                        tc.report_error(left_expr.span.clone(), e);
                        return error_expr(span, id);
                    }
                }
                (right_ty, true)
            } else {
                let s = left_expr.span.clone().merge(right_expr.span.clone());
                tc.report_error(
                    s,
                    format!(
                        "Type mismatch: cannot assign '{}' to '{}'",
                        right_ty.name(),
                        left_ty.name()
                    ),
                );
                (Type::Error, false)
            }
        }

        // --- Compound Assignment (+=, -=, ...) ---
        BinaryOp::AssignAdd
        | BinaryOp::AssignSub
        | BinaryOp::AssignMul
        | BinaryOp::AssignDiv
        | BinaryOp::AssignMod
        | BinaryOp::AssignBitAnd
        | BinaryOp::AssignBitOr
        | BinaryOp::AssignBitXor
        | BinaryOp::AssignShl
        | BinaryOp::AssignShr => {
            if left_ty == right_ty && is_numeric(&left_ty) {
                if let ExprKind::Ident(ref original_name) = left_expr.kind {
                    if let Err(e) = tc.ctx.assign(original_name) {
                        tc.report_error(left_expr.span.clone(), e);
                        return error_expr(span, id);
                    }
                }
                (right_ty, true)
            } else {
                let s = left_expr.span.clone().merge(right_expr.span.clone());
                tc.report_error(
                    s,
                    format!(
                        "Type mismatch: cannot perform compound assignment between '{}' and '{}'",
                        left_ty.name(),
                        right_ty.name()
                    ),
                );
                (Type::Error, false)
            }
        }

        BinaryOp::KeyValue | BinaryOp::ConstDef => (right_ty, true),
    };

    if !valid {
        return error_expr(span, id);
    }
    let binary_expr = TypedExpr {
        kind: TypedExprKind::Binary(
            Box::new(typed_left.clone()),
            op,
            Box::new(typed_right.clone()),
        ),
        ty: result_ty,
        span: span.clone(),
        id,
    };

    let is_foldable_op = match op {
        BinaryOp::Assign
        | BinaryOp::KeyValue
        | BinaryOp::ConstDef
        | BinaryOp::AssignAdd
        | BinaryOp::AssignSub
        | BinaryOp::AssignMul
        | BinaryOp::AssignDiv
        | BinaryOp::AssignMod
        | BinaryOp::AssignBitAnd
        | BinaryOp::AssignBitOr
        | BinaryOp::AssignBitXor
        | BinaryOp::AssignShl
        | BinaryOp::AssignShr => false,
        _ => true,
    };

    if is_foldable_op {
        let left_is_const =
            tc.side_table.is_const(typed_left.id) && tc.side_table.should_fold(typed_left.id);

        let right_is_const =
            tc.side_table.is_const(typed_right.id) && tc.side_table.should_fold(typed_right.id);

        if left_is_const && right_is_const {
            let mut folded_expr = tc.comptime.evaluate_expr(binary_expr);

            folded_expr.id = id;
            folded_expr.span = span;

            tc.side_table.mark_const(id, true);

            return folded_expr;
        }
    }

    binary_expr
}

fn extract_type_from_expr(tc: &mut TypeChecker, expr: &TypedExpr) -> Type {
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

fn is_numeric(t: &Type) -> bool {
    matches!(t, Type::I32 | Type::F32)
}

fn is_integer(t: &Type) -> bool {
    matches!(t, Type::I32)
}

fn is_assignable(tc: &TypeChecker, target: &Type, source: &Type) -> bool {
    if target == source {
        return true;
    }

    let target_underlying = get_underlying_type(tc, target);
    let source_underlying = get_underlying_type(tc, source);

    target_underlying == source_underlying
}

fn get_underlying_type(tc: &TypeChecker, ty: &Type) -> Type {
    match ty {
        Type::Alias(name, inner) => {
            if let Some(registered) = tc.type_registry.get(name) {
                get_underlying_type(tc, registered)
            } else {
                get_underlying_type(tc, inner)
            }
        }
        _ => ty.clone(),
    }
}

fn check_var_dec<'a>(
    tc: &mut TypeChecker<'a>,
    pattern: &'a Box<Expr>,
    ty_expr: &'a Box<Expr>,
    right: TypedExpr,
    span: Span,
    id: u32,
) -> TypedExpr {
    match pattern.kind {
        ExprKind::Wildcard => {
            if right.ty == Type::Error {
                return error_expr(span, id);
            }

            TypedExpr {
                kind: TypedExprKind::Wildcard,
                id,
                span,
                ty: Type::Unit,
            }
        }

        ExprKind::Ident(ref n) => {
            let name = n.to_string();

            let init_type = if ty_expr.kind == ExprKind::Wildcard {
                if right.ty == Type::Error {
                    tc.report_error(right.span_expr(), "Type resolution failed.".to_string());
                    return error_expr(span, id);
                }
                right.ty.clone()
            } else {
                let typed_ty = tc.check_expr(ty_expr);

                if typed_ty.ty == Type::Error {
                    tc.report_error(
                        typed_ty.span_expr(),
                        format!("Type resolution failed for variable '{}'.", name),
                    );
                    return error_expr(span, id);
                }

                if typed_ty.ty == Type::Metatype {
                    extract_type_from_expr(tc, &typed_ty)
                } else {
                    typed_ty.ty
                }
            };

            if !is_assignable(tc, &init_type, &right.ty) && right.ty != Type::Error {
                tc.report_error(
                    span.clone(),
                    format!(
                        "Type mismatch: variable '{}' has type '{}' but assigned '{}'",
                        name,
                        init_type.name(),
                        right.ty.name()
                    ),
                );
                return error_expr(span, id);
            }

            let ir_name = tc.ctx.define(
                name.clone(),
                SymbolInfo {
                    ir_name: String::new(),
                    is_initialized: true,
                    is_native: false,
                    is_mutable: true,
                    kind: SymbolKind::Variable,
                    ty: init_type.clone(),
                    is_foldable: false,
                },
            );

            TypedExpr {
                kind: TypedExprKind::VarDec(ir_name, init_type.clone(), Some(Box::new(right))),
                id,
                span,
                ty: init_type,
            }
        }

        _ => {
            tc.report_error(
                pattern.span_expr(),
                "LHS must be an identifier. Found pattern.".to_string(),
            );
            error_expr(span, id)
        }
    }
}
