use abyss_diagnostics::Span;
use abyss_parser::ast::{Expr, ExprKind};

use crate::type_checker::{
    engine::{TypeChecker, error_expr},
    rules::signature::check_signature,
    tast::{TypedExpr, TypedExprKind},
    types::Type,
};

pub fn check_ret(tc: &mut TypeChecker, val: &Option<Box<Expr>>, span: Span, id: u32) -> TypedExpr {
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
        span: span,
        id: id,
    }
}

pub fn check_def(
    tc: &mut TypeChecker,
    name_expr: &Expr,
    value_expr: &Expr,
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

    if let ExprKind::Signature(ref args, ref ret, ref body) = value_expr.kind {
        let func_ref = check_signature(
            tc,
            args,
            ret,
            body,
            Some(name.clone()),
            value_expr.span.clone(),
            value_expr.id,
        );

        tc.ctx.define(name.clone(), func_ref.ty.clone());

        return TypedExpr {
            kind: TypedExprKind::FuncRef(name),
            ty: func_ref.ty,
            span,
            id,
        };
    }

    let typed_value = tc.check_expr(value_expr);

    if typed_value.ty == Type::Error {
        return error_expr(span, id);
    }

    tc.ctx.define(name.clone(), typed_value.ty.clone());

    TypedExpr {
        kind: TypedExprKind::Def(name, Box::new(typed_value.clone())),
        ty: typed_value.ty,
        span,
        id,
    }
}
