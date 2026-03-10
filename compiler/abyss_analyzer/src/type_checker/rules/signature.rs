use abyss_diagnostics::Span;
use abyss_parser::ast::{BinaryOp, Expr, ExprKind};
use abyss_types::{
    tast::{TypedExpr, TypedExprKind},
    types::Type,
};

use crate::type_checker::engine::TypeChecker;

pub fn check_signature(
    tc: &mut TypeChecker,
    args: &Vec<Expr>,
    ret_ty: &Option<Box<Expr>>,
    body: &Box<Expr>,
    name_opt: Option<String>,
    span: Span,
    id: u32,
) -> TypedExpr {
    let return_type = if let Some(t) = ret_ty {
        let checked_ret_expr = tc.check_expr(t);
        tc.evaluate_as_type(checked_ret_expr)
    } else {
        Type::Unit
    };

    tc.ctx.enter_scope();

    let mut checked_args = Vec::new();
    let mut arg_types = Vec::new();

    for arg in args {
        if let ExprKind::Binary(ref left, BinaryOp::KeyValue, ref right) = arg.kind {
            if let ExprKind::Ident(ref arg_name) = left.kind {
                let typed_ty_expr = tc.check_expr(right);
                let arg_ty = tc.evaluate_as_type(typed_ty_expr);

                tc.ctx.define(arg_name.clone(), arg_ty.clone());
                arg_types.push(arg_ty.clone());

                checked_args.push(TypedExpr {
                    kind: TypedExprKind::VarDec(arg_name.clone(), arg_ty.clone(), None),
                    ty: arg_ty,
                    span: arg.span.clone(),
                    id: arg.id,
                });
            }
        }
    }

    let (checked_body, is_native) = if ExprKind::Wildcard == body.kind {
        (
            TypedExpr {
                kind: TypedExprKind::Wildcard,
                ty: Type::Unit,
                span: span.clone(),
                id,
            },
            true,
        )
    } else {
        (tc.check_expr(&body), false)
    };

    let func_type = Type::Signature(arg_types, Box::new(return_type.clone()), is_native);

    if let Some(ref name) = name_opt {
        tc.ctx.define(name.clone(), func_type.clone());
    }

    tc.ctx.exit_scope();

    let func_name = name_opt.clone().unwrap_or_else(|| {
        tc.anon_func_counter += 1;
        format!("__anon_func_{}", tc.anon_func_counter)
    });

    let function_def_node = TypedExpr {
        kind: TypedExprKind::FunctionDef {
            name: func_name.clone(),
            args: checked_args,
            ret_ty: return_type,
            body: Box::new(checked_body),
            is_native,
        },
        ty: func_type.clone(),
        span: span.clone(),
        id,
    };

    if let Some(name) = name_opt {
        tc.ctx
            .register_resolved_global(name.clone(), function_def_node.clone());

        if is_native {
            TypedExpr {
                kind: TypedExprKind::Block(vec![]),
                ty: Type::Unit,
                span,
                id,
            }
        } else {
            TypedExpr {
                kind: TypedExprKind::FuncRef(name),
                ty: func_type,
                span,
                id,
            }
        }
    } else {
        function_def_node
    }
}
