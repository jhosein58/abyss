use crate::type_checker::{
    engine::{TypeChecker, error_expr},
    rules::signature::check_signature,
};
use abyss_diagnostics::Span;
use abyss_ir::{builder::IrBuilder, ir::IrLit};
use abyss_parser::ast::{Expr, ExprKind, Lit, OrderedFloat};
use abyss_types::{
    tast::{TypedExpr, TypedExprKind},
    types::Type,
};
use abyss_vm::execute_comptime;

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

    let typed_value = tc.check_expr(value_expr);

    tc.ctx
        .register_resolved_global(name.clone(), typed_value.clone());

    TypedExpr {
        kind: TypedExprKind::Def(name, Box::new(typed_value.clone())),
        ty: typed_value.ty,
        span,
        id,
    }
}

pub fn check_cmpt(tc: &mut TypeChecker, inner_expr: &Expr, span: Span, id: u32) -> TypedExpr {
    let typed_inner = tc.check_expr(inner_expr);
    let inner_ty = typed_inner.ty.clone();

    if inner_ty == Type::Error {
        return error_expr(span, id);
    }

    let mut ir_builder = IrBuilder::new();
    let ir_prog = ir_builder.build_comptime_program(typed_inner, &tc.ctx.resolved_globals);

    let result_lit = execute_comptime(ir_prog);

    let ast_lit = match result_lit {
        IrLit::Int(val) => Lit::Int(val),
        IrLit::Bool(val) => Lit::Bool(val),

        IrLit::Float(val) => Lit::Float(OrderedFloat(val)),
        // _ => {
        //     tc.report_error(
        //         span.clone(),
        //         "Comptime execution returned an unsupported value.".to_string(),
        //     );
        //     return error_expr(span, id);
        // }
    };

    TypedExpr {
        kind: TypedExprKind::Lit(ast_lit),
        ty: inner_ty,
        span,
        id,
    }
}
