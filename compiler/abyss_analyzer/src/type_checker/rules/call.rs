use abyss_diagnostics::Span;
use abyss_parser::ast::{Expr, ExprKind, UnaryOp};

use crate::type_checker::engine::{TypeChecker, error_expr};
use abyss_types::{
    tast::{TypedExpr, TypedExprKind},
    types::Type,
};

pub fn check_call<'a>(
    tc: &mut TypeChecker<'a>,
    calle: &'a Box<Expr>,
    args: &'a Vec<Expr>,
    span: Span,
    id: u32,
) -> TypedExpr {
    if let ExprKind::Ident(ref name) = calle.kind {
        if name == "print" {
            let mut new_args = Vec::with_capacity(args.len());
            for a in args.iter() {
                new_args.push(tc.check_expr(a));
            }

            return TypedExpr {
                kind: TypedExprKind::Call(
                    Box::new(TypedExpr {
                        kind: TypedExprKind::Ident(name.clone()),
                        ty: Type::Signature(vec![Type::I32], Box::new(Type::Unit), false),
                        span: calle.span_expr(),
                        id: tc.next_id(),
                    }),
                    new_args,
                    false,
                ),
                ty: Type::Unit,
                span,
                id,
            };
        }
    }

    let mut checked_calle = tc.check_expr(&calle);
    let mut actual_args = Vec::new();

    if let TypedExprKind::BoundMethod {
        receiver,
        method_name,
    } = checked_calle.kind
    {
        checked_calle = TypedExpr {
            kind: TypedExprKind::Ident(method_name.clone()),
            ty: checked_calle.ty.clone(),
            span: checked_calle.span.clone(),
            id: checked_calle.id,
        };

        if let Type::Signature(ref param_tys, _, _) = checked_calle.ty {
            if let Some(expected_self_ty) = param_tys.get(0) {
                let mut current_receiver = *receiver;

                if let Type::Ptr(expected_inner) = expected_self_ty {
                    if current_receiver.ty == **expected_inner {
                        current_receiver = TypedExpr {
                            kind: TypedExprKind::Unary(
                                UnaryOp::AddrOf,
                                Box::new(current_receiver.clone()),
                            ),
                            ty: expected_self_ty.clone(),
                            span: current_receiver.span.clone(),
                            id: tc.next_id(),
                        };
                    }
                }

                while current_receiver.ty.is_ptr() && current_receiver.ty != *expected_self_ty {
                    let inner_ty = current_receiver.ty.get_inner_ptr_type();
                    current_receiver = TypedExpr {
                        kind: TypedExprKind::Unary(
                            UnaryOp::Deref,
                            Box::new(current_receiver.clone()),
                        ),
                        ty: inner_ty,
                        span: current_receiver.span.clone(),
                        id: tc.next_id(),
                    };
                }
                actual_args.push(current_receiver);
            }
        }
    }

    if let Type::Signature(param_tys, ret_ty, is_native) = checked_calle.ty.clone() {
        let provided_arg_count = actual_args.len() + args.len();
        let expected_arg_count = param_tys.len();

        if provided_arg_count != expected_arg_count {
            tc.report_error(
                span.clone(),
                format!(
                    "Function expects {} arguments, but {} were provided.",
                    expected_arg_count, provided_arg_count
                ),
            );
            return error_expr(span, id);
        }

        for (i, a) in args.iter().enumerate() {
            let checked_arg = tc.check_expr(a);

            let param_index = actual_args.len();
            let expected_ty = &param_tys[param_index];

            if !expected_ty.accepts(&checked_arg.ty) {
                tc.report_error(
                    a.span.clone(),
                    format!(
                        "Type mismatch for argument {}: expected '{}', found '{}'.",
                        i + 1,
                        expected_ty.name(),
                        checked_arg.ty.name()
                    ),
                );
            }

            actual_args.push(checked_arg);
        }

        let call_expr = TypedExpr {
            kind: TypedExprKind::Call(Box::new(checked_calle.clone()), actual_args, is_native),
            ty: *ret_ty.clone(),
            span: span.clone(),
            id,
        };

        return call_expr;
    }

    tc.report_error(
        calle.span_expr(),
        format!(
            "Only Signatures can be called. found '{}'.",
            checked_calle.ty.name()
        ),
    );

    error_expr(span, id)
}
