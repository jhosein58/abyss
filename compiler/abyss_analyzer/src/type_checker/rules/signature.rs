use abyss_diagnostics::Span;
use abyss_parser::ast::{BinaryOp, Expr, ExprKind, UnaryOp};
use abyss_types::{
    tast::{TypedExpr, TypedExprKind},
    types::Type,
};

use crate::type_checker::{
    context::SymbolInfo,
    engine::TypeChecker,
    resolver::{GlobalMetadata, InlinePolicy},
};

pub fn check_signature<'a>(
    tc: &mut TypeChecker<'a>,
    args: &'a Vec<Expr>,
    ret_ty: &'a Option<Box<Expr>>,
    body: &'a Box<Expr>,
    name_opt: Option<String>,
    span: Span,
    id: u32,
) -> TypedExpr {
    let return_type = if let Some(t) = ret_ty {
        resolve_type_expr(tc, t)
    } else {
        Type::Unit
    };

    tc.ctx.enter_scope();

    let mut checked_args = Vec::new();
    let mut arg_types = Vec::new();

    for arg in args {
        if let ExprKind::Binary(ref left, BinaryOp::KeyValue, ref right) = arg.kind {
            if let ExprKind::Ident(ref arg_name) = left.kind {
                let arg_ty = resolve_type_expr(tc, right);

                let ir_name = tc.ctx.define_with_type(arg_name.clone(), arg_ty.clone());
                arg_types.push(arg_ty.clone());

                checked_args.push(TypedExpr {
                    kind: TypedExprKind::VarDec(ir_name, arg_ty.clone(), None),
                    ty: arg_ty,
                    span: arg.span.clone(),
                    id: arg.id,
                });
            }
        }
    }

    let is_native = ExprKind::Wildcard == body.kind;
    let func_type = Type::Signature(arg_types.clone(), Box::new(return_type.clone()), is_native);

    let mut func_ir_name_opt = None;
    if let Some(ref name) = name_opt {
        tc.ctx.update_type(name, func_type.clone());

        let symbol_info = if is_native {
            SymbolInfo::native_function(name.clone(), func_type.clone())
        } else {
            let initial_ir_name = if tc.resolver.contains(name) {
                name.clone()
            } else {
                String::new()
            };
            SymbolInfo::constant(initial_ir_name, func_type.clone(), false)
        };

        let ir_name = tc.ctx.define(name.clone(), symbol_info);

        func_ir_name_opt = Some(ir_name);

        tc.resolver
            .set_forward_declaration(name.clone(), func_type.clone());
    }

    let checked_body = if is_native {
        TypedExpr {
            kind: TypedExprKind::Wildcard,
            ty: Type::Unit,
            span: span.clone(),
            id: tc.next_id(),
        }
    } else {
        tc.check_expr(body)
    };

    tc.ctx.exit_scope();

    if let Some(ref name) = name_opt {
        if let Some(global_info) = tc.ctx.lookup_mut(name) {
            global_info.ir_name = name.clone();
            if is_native {
                global_info.is_native = true;
            }
        }
    }
    let func_name = func_ir_name_opt.clone().unwrap_or_else(|| {
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
        id: if is_native && name_opt.is_some() {
            tc.next_id()
        } else {
            id
        },
    };

    if let Some(name) = name_opt {
        tc.ctx.update_type(&name, func_type.clone());

        if tc.resolver.contains(&name) {
            tc.complete_and_register_global(
                name.clone(),
                func_type.clone(),
                function_def_node.clone(),
                false,
                GlobalMetadata {
                    inline_policy: InlinePolicy::Never,
                    is_foldable: false,
                },
            );
        }

        if is_native {
            TypedExpr {
                kind: TypedExprKind::Block(vec![]),
                ty: Type::Unit,
                span,
                id,
            }
        } else {
            TypedExpr {
                kind: TypedExprKind::FuncRef(func_ir_name_opt.unwrap()),
                ty: func_type,
                span,
                id,
            }
        }
    } else {
        function_def_node
    }
}

fn resolve_type_expr<'a>(tc: &mut TypeChecker<'a>, expr: &'a Expr) -> Type {
    match &expr.kind {
        ExprKind::Ident(name) => {
            if let Some(prim_ty) = tc.primitive_type_from_name(name) {
                return prim_ty;
            }
            if let Some(ty) = tc.type_registry.get(name) {
                return ty.clone();
            }

            tc.resolve_type_by_name(name, expr.span.clone())
        }

        ExprKind::Unary(UnaryOp::AddrOf, inner_expr) => {
            let inner_type = resolve_type_expr(tc, inner_expr);
            if inner_type == Type::Error {
                Type::Error
            } else {
                Type::Ptr(Box::new(inner_type))
            }
        }
        ExprKind::Sequence(_items, _) => {
            let checked = tc.check_expr(expr);
            tc.evaluate_as_type(checked)
        }

        _ => {
            let checked = tc.check_expr(expr);

            if checked.ty == Type::Metatype {
                tc.evaluate_as_type(checked)
            } else if let TypedExprKind::Type(ty) = checked.kind {
                ty
            } else {
                tc.report_error(
                    expr.span.clone(),
                    format!(
                        "Expected a type, found expression of type '{}'",
                        checked.ty.name()
                    ),
                );
                Type::Error
            }
        }
    }
}
