use abyss_diagnostics::Span;
use abyss_parser::ast::{Expr, UnaryOp};

use crate::type_checker::{
    engine::{TypeChecker, error_expr},
    template::{
        instantiate::{TypeSubstitution, instantiate_template},
        registry::{MonomorphizedInstance, ParamKind, TemplateRegistry},
    },
};
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
    let mut checked_calle = tc.check_expr(&calle);
    let mut actual_args = Vec::new();
    let mut template_ir_name_opt = None;

    match &checked_calle.kind {
        TypedExprKind::BoundMethod {
            method_name,
            receiver,
        } => {
            if tc.template_registry.is_template(method_name) {
                template_ir_name_opt = Some((method_name.clone(), Some(*receiver.clone())));
            } else {
                actual_args.push(*receiver.clone());
                checked_calle.kind = TypedExprKind::Ident(method_name.clone());
            }
        }
        TypedExprKind::Ident(name) | TypedExprKind::FuncRef(name) => {
            if tc.template_registry.is_template(name) {
                template_ir_name_opt = Some((name.clone(), None));
            }
        }
        _ => {}
    }

    if let Some((template_ir_name, receiver_opt)) = template_ir_name_opt {
        let template = tc.template_registry.get(&template_ir_name).unwrap().clone();

        let mut checked_passed_args = Vec::new();
        if let Some(recv) = receiver_opt {
            checked_passed_args.push(recv);
        }
        for a in args {
            checked_passed_args.push(tc.check_expr(a));
        }

        let mut concrete_types = Vec::new();
        let mut subst = TypeSubstitution::new();

        for param in &template.template_params {
            if param.param_index < checked_passed_args.len() {
                let passed_arg = &checked_passed_args[param.param_index];

                match &param.kind {
                    ParamKind::Duck { duck_struct, .. } => {
                        let concrete = passed_arg.ty.peel_pointers();
                        subst.add(duck_struct, &concrete);
                        concrete_types.push(passed_arg.ty.clone());
                    }
                    ParamKind::MetaType => {
                        let concrete_type = tc.evaluate_as_type(passed_arg.clone());
                        subst.add(&Type::Metatype, &concrete_type);
                        concrete_types.push(concrete_type);
                    }
                }
            }
        }

        let concrete_key = TemplateRegistry::make_concrete_key(&concrete_types);
        let final_func_name;

        if let Some(inst) = tc
            .template_registry
            .get_cached(&template_ir_name, &concrete_key)
        {
            final_func_name = inst.ir_name.clone();
            checked_calle.ty = inst.func_type.clone();
        } else {
            let mono_name = tc
                .template_registry
                .generate_mono_name(&template.source_name, &concrete_key);
            let new_def = instantiate_template(&template.typed_def, mono_name.clone(), &subst);
            let final_func_type = subst.apply(&template.func_type);

            let instance = MonomorphizedInstance {
                ir_name: mono_name.clone(),
                func_type: final_func_type.clone(),
                typed_def: new_def,
            };

            tc.template_registry
                .cache_instance(template_ir_name, concrete_key, instance);
            final_func_name = mono_name;
            checked_calle.ty = final_func_type;
        }

        checked_calle.kind = TypedExprKind::Ident(final_func_name);
        actual_args = checked_passed_args;
    } else {
        if let TypedExprKind::Ident(_) = checked_calle.kind {
            for a in args {
                actual_args.push(tc.check_expr(a));
            }
        }
    }

    if let Type::Signature(ref param_tys, ret_ty, is_native) = checked_calle.ty.clone() {
        if actual_args.len() != param_tys.len() {
            tc.report_error(
                span.clone(),
                format!("Function expects {} args.", param_tys.len()),
            );
            return error_expr(span, id);
        }

        for i in 0..actual_args.len() {
            let expected_ty = param_tys[i].clone();
            let mut arg_expr = actual_args[i].clone();

            if let Type::Ptr(expected_inner) = &expected_ty {
                if arg_expr.ty == **expected_inner {
                    arg_expr = TypedExpr {
                        kind: TypedExprKind::Unary(UnaryOp::AddrOf, Box::new(arg_expr.clone())),
                        ty: expected_ty.clone(),
                        span: arg_expr.span.clone(),
                        id: tc.next_id(),
                    };
                }
            }
            while arg_expr.ty.is_ptr() && arg_expr.ty != expected_ty {
                let inner_ty = arg_expr.ty.get_inner_ptr_type();
                arg_expr = TypedExpr {
                    kind: TypedExprKind::Unary(UnaryOp::Deref, Box::new(arg_expr.clone())),
                    ty: inner_ty,
                    span: arg_expr.span.clone(),
                    id: tc.next_id(),
                };
            }

            if !expected_ty.accepts(&arg_expr.ty) {
                tc.report_error(arg_expr.span.clone(), format!("Type mismatch."));
            }
            actual_args[i] = arg_expr;
        }

        return TypedExpr {
            kind: TypedExprKind::Call(Box::new(checked_calle), actual_args, is_native),
            ty: *ret_ty,
            span,
            id,
        };
    }

    tc.report_error(calle.span_expr(), "Only Signatures can be called.".into());
    error_expr(span, id)
}
