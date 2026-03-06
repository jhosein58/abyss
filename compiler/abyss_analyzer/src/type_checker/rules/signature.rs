use abyss_diagnostics::Span;
use abyss_parser::ast::{BinaryOp, Expr, ExprKind};

use crate::type_checker::{
    engine::{TypeChecker, error_expr},
    tast::{TypedExpr, TypedExprKind},
    types::Type,
};

pub fn check_signature(
    tc: &mut TypeChecker,
    args: &Vec<Expr>,
    ret_ty: &Option<Box<Expr>>,
    body: &Box<Expr>,
    name_opt: Option<String>,
    span: Span,
    id: u32,
) -> TypedExpr {
    let ty = if let Some(t) = ret_ty {
        let checkd_ret_type = tc.check_expr(t);
        (checkd_ret_type.ty.clone(), checkd_ret_type.span_expr())
    } else {
        (Type::Unit, Span::empty())
    };

    if ty.0 == Type::Error {
        tc.report_error(ty.1, format!("Error finding return type."));
        return error_expr(span.clone(), id);
    }

    tc.ctx.enter_scope();

    let mut checked_args = Vec::new();
    let mut arg_types = Vec::new();

    for arg in args {
        if let ExprKind::Binary(ref left, BinaryOp::KeyValue, ref right) = arg.kind {
            if let ExprKind::Ident(ref arg_name) = left.kind {
                let arg_ty_expr = tc.check_expr(right);
                let arg_ty = arg_ty_expr.ty;

                tc.ctx.define(arg_name.clone(), arg_ty.clone());

                arg_types.push(arg_ty.clone());

                checked_args.push(TypedExpr {
                    kind: TypedExprKind::VarDec(arg_name.clone(), arg_ty.clone(), None),
                    ty: arg_ty,
                    span: arg.span.clone(),
                    id: arg.id,
                });
            } else {
                tc.report_error(
                    left.span.clone(),
                    "Argument name must be an identifier".into(),
                );
            }
        } else {
            tc.report_error(
                arg.span.clone(),
                "Expected 'name: type' format for arguments".into(),
            );
        }
    }

    let checked_body = tc.check_expr(&body);

    tc.ctx.exit_scope();

    let func_name = name_opt.unwrap_or_else(|| {
        tc.anon_func_counter += 1;
        format!("__anon_func_{}", tc.anon_func_counter)
    });

    let func_type = Type::Signature(arg_types, Box::new(ty.0.clone()));

    let func_def = TypedExpr {
        kind: TypedExprKind::FunctionDef {
            name: func_name.clone(),
            args: checked_args,
            ret_ty: ty.0,
            body: Box::new(checked_body),
        },
        ty: func_type.clone(),
        span: span.clone(),
        id,
    };
    tc.hoisted_functions.push(func_def);

    TypedExpr {
        kind: TypedExprKind::FuncRef(func_name),
        ty: func_type,
        span,
        id,
    }
}
