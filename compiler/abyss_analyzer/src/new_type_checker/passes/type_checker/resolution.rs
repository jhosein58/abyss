use super::utils::GenericsEngine;
use crate::new_type_checker::{Pass, context::TypeContext};
use abyss_parser::ast::{Expr, ExprKind, FunctionBody, FunctionDef, Lit, Stmt, StmtKind, Type};
use std::collections::HashMap;

pub struct ResolutionPass;

impl ResolutionPass {
    pub fn new() -> Self {
        ResolutionPass
    }

    fn infer_lit_type(&self, lit: &Lit, _ctx: &mut TypeContext) -> Type {
        match lit {
            Lit::Int(_) => Type::I64,
            Lit::Float(_) => Type::F64,
            Lit::Bool(_) => Type::Bool,
            Lit::Str(s) => Type::Array(Box::new(Type::U8), s.len() - 2),
            Lit::Null => Type::Pointer(Box::new(Type::Void)),
            Lit::Array(elements) => {
                if elements.is_empty() {
                    return Type::Array(Box::new(Type::Void), 0);
                }
                let first_ty = elements[0].ty.clone().unwrap_or(Type::Void);
                Type::Array(Box::new(first_ty), elements.len())
            }
        }
    }

    fn path_to_string(&self, path: &[String]) -> String {
        path.join("::")
    }
}

impl ResolutionPass {
    pub fn visit_function_def(&mut self, func: &mut FunctionDef, ctx: &mut TypeContext) {
        if let FunctionBody::UserDefined(body) = &mut func.body {
            ctx.set_current_function(func.name.clone());

            for (param_name, param_type) in &func.params {
                let _ = ctx.define_symbol(param_name.clone(), param_type.clone());
            }

            for stmt in body {
                self.visit_stmt(stmt, ctx);
            }
        }
    }

    fn visit_stmt(&mut self, stmt: &mut Stmt, ctx: &mut TypeContext) {
        match stmt.kind {
            StmtKind::Let(ref name, ref mut ty, Some(ref mut expr)) => {
                self.visit_expr(expr, ctx);

                if ty.is_none() {
                    *ty = expr.ty.clone();
                }

                if let Some(resolved_ty) = ty {
                    let _ = ctx.define_symbol(name.to_string(), resolved_ty.clone());
                }
            }
            StmtKind::Let(ref name, ref ty, None) => {
                if let Some(resolved_ty) = ty {
                    let _ = ctx.define_symbol(name.to_string(), resolved_ty.clone());
                }
            }
            StmtKind::Block(ref mut stmts) => {
                ctx.enter_scope();
                for s in stmts {
                    self.visit_stmt(s, ctx);
                }
                ctx.exit_scope();
            }
            _ => {}
        }
    }
}

impl ResolutionPass {
    pub fn visit_expr(&mut self, expr: &mut Expr, ctx: &mut TypeContext) {
        match expr.kind {
            ExprKind::Lit(ref mut lit) => {
                if let Lit::Array(elems) = lit {
                    for e in elems {
                        self.visit_expr(e, ctx);
                    }
                }
                expr.ty = Some(self.infer_lit_type(lit, ctx));
            }

            ExprKind::Ident(ref path) => {
                let name = self.path_to_string(path);
                if let Some(ty) = ctx.resolve_symbol(&name) {
                    expr.ty = Some(ty.clone());
                } else if let Some(func_def) = ctx.generic_func_templates.get(&name) {
                    let param_types: Vec<Type> = func_def
                        .params
                        .iter()
                        .map(|(_, t)| {
                            GenericsEngine::resolve_generic_references(t, &func_def.generics)
                        })
                        .collect();
                    let return_type = GenericsEngine::resolve_generic_references(
                        &func_def.return_type,
                        &func_def.generics,
                    );
                    let generics_decl: Vec<Type> = func_def
                        .generics
                        .iter()
                        .map(|g_name| Type::Generic(g_name.clone()))
                        .collect();
                    expr.ty = Some(Type::Function(
                        param_types,
                        Box::new(return_type),
                        generics_decl,
                    ));
                } else if let Some(func_def) = ctx.concrete_funcs.get(&name) {
                    let param_types: Vec<Type> =
                        func_def.params.iter().map(|(_, t)| t.clone()).collect();
                    expr.ty = Some(Type::Function(
                        param_types,
                        Box::new(func_def.return_type.clone()),
                        vec![],
                    ));
                }
            }

            ExprKind::Binary(ref mut left, _, ref mut right) => {
                self.visit_expr(left, ctx);
                self.visit_expr(right, ctx);
                expr.ty = left.ty.clone();
            }

            ExprKind::Unary(_, ref mut operand) => {
                self.visit_expr(operand, ctx);
                expr.ty = operand.ty.clone();
            }

            ExprKind::Call(ref mut callee, ref mut args, ref explicit_generics) => {
                self.visit_expr(callee, ctx);
                for arg in args.iter_mut() {
                    self.visit_expr(arg, ctx);
                }

                if let Some(Type::Function(param_types, ret_type, generic_params_decl)) =
                    callee.ty.clone()
                {
                    let mut generic_map = HashMap::new();
                    let engine = GenericsEngine;

                    if !explicit_generics.is_empty() {
                        for (decl, concrete) in
                            generic_params_decl.iter().zip(explicit_generics.iter())
                        {
                            if let Type::Generic(name) = decl {
                                generic_map.insert(name.clone(), concrete.clone());
                            }
                        }
                    } else {
                        for (param_ty, arg) in param_types.iter().zip(args.iter()) {
                            if let Some(arg_ty) = &arg.ty {
                                engine.unify_types(param_ty, arg_ty, &mut generic_map);
                            }
                        }
                    }

                    if !generic_params_decl.is_empty() {
                        let callee_name = match &callee.kind {
                            ExprKind::Ident(path) => Some(self.path_to_string(path)),
                            _ => None,
                        };

                        if let Some(name) = callee_name {
                            let mut concrete_types = Vec::new();
                            for gen_decl in generic_params_decl {
                                if let Type::Generic(n) = gen_decl {
                                    if let Some(t) = generic_map.get(&n) {
                                        concrete_types.push(t.clone());
                                    }
                                }
                            }
                            ctx.register_generic_func_request(name, concrete_types);
                        }
                    }

                    expr.ty = Some(GenericsEngine::substitute_generics_mut(
                        &ret_type,
                        &generic_map,
                        ctx,
                    ));
                }
            }

            ExprKind::StructInit(ref mut path, ref mut fields, ref generics) => {
                let struct_name = path.last().unwrap().clone();

                let (struct_generics_decl, raw_fields_decl) =
                    if let Some(def) = ctx.concrete_structs.get(&struct_name) {
                        (def.generics.clone(), def.fields.clone())
                    } else if let Some(def) = ctx.generic_struct_templates.get(&struct_name) {
                        (def.generics.clone(), def.fields.clone())
                    } else {
                        // If not found, we can't infer much, safe to return or panic in safety pass
                        return;
                    };

                let struct_fields_decl: Vec<(String, Type)> = raw_fields_decl
                    .iter()
                    .map(|(name, ty)| {
                        (
                            name.clone(),
                            GenericsEngine::resolve_generic_references(ty, &struct_generics_decl),
                        )
                    })
                    .collect();

                let mut type_map = HashMap::new();
                let mut concrete_generics = generics.clone();
                let engine = GenericsEngine;

                for (_, ex) in fields.iter_mut() {
                    self.visit_expr(ex, ctx);
                }

                if !struct_generics_decl.is_empty() {
                    if concrete_generics.is_empty() {
                        for (field_name, f_expr) in fields.iter() {
                            if let Some((_, expected_def_ty)) =
                                struct_fields_decl.iter().find(|(n, _)| n == field_name)
                            {
                                if let Some(act_ty) = &f_expr.ty {
                                    engine.unify_types(expected_def_ty, act_ty, &mut type_map);
                                }
                            }
                        }
                        for gen_name in &struct_generics_decl {
                            if let Some(ty) = type_map.get(gen_name) {
                                concrete_generics.push(ty.clone());
                            }
                        }
                    } else {
                        for (name, ty) in struct_generics_decl.iter().zip(concrete_generics.iter())
                        {
                            type_map.insert(name.clone(), ty.clone());
                        }
                    }

                    ctx.register_generic_struct_request(
                        struct_name.clone(),
                        concrete_generics.clone(),
                    );
                    let mangled_name =
                        GenericsEngine::instantiate_struct(&struct_name, &concrete_generics, ctx);
                    *path = vec![mangled_name.clone()];
                    expr.ty = Some(Type::Struct(vec![mangled_name], vec![]));
                } else {
                    expr.ty = Some(Type::Struct(path.clone(), concrete_generics));
                }
            }

            ExprKind::Member(ref mut object, ref field_name) => {
                self.visit_expr(object, ctx);
                if let Some(obj_ty) = &object.ty {
                    let mut actual_ty = obj_ty;
                    if let Type::Pointer(inner) = obj_ty {
                        actual_ty = inner;
                    }

                    if let Type::Struct(path, concrete_generics) = actual_ty {
                        let struct_name = path.last().unwrap();
                        let (struct_generics_decl, struct_fields_decl) = if let Some(def) =
                            ctx.concrete_structs.get(struct_name)
                        {
                            (def.generics.clone(), def.fields.clone())
                        } else if let Some(def) = ctx.generic_struct_templates.get(struct_name) {
                            (def.generics.clone(), def.fields.clone())
                        } else {
                            return;
                        };

                        if let Some((_, field_ty)) =
                            struct_fields_decl.iter().find(|(n, _)| n == field_name)
                        {
                            let mut field_map = HashMap::new();
                            for (gen_name, concrete) in
                                struct_generics_decl.iter().zip(concrete_generics.iter())
                            {
                                field_map.insert(gen_name.clone(), concrete.clone());
                            }
                            expr.ty = Some(GenericsEngine::substitute_generics_mut(
                                field_ty, &field_map, ctx,
                            ));
                        }
                    }
                }
            }

            ExprKind::Index(ref mut arr, ref mut idx) => {
                self.visit_expr(arr, ctx);
                self.visit_expr(idx, ctx);
                if let Some(arr_ty) = &arr.ty {
                    match arr_ty {
                        Type::Array(inner, _) => expr.ty = Some(*inner.clone()),
                        Type::Pointer(inner) => expr.ty = Some(*inner.clone()),
                        _ => {}
                    }
                }
            }
            ExprKind::Deref(ref mut inner) => {
                self.visit_expr(inner, ctx);
                if let Some(Type::Pointer(inner_ty)) = &inner.ty {
                    expr.ty = Some(*inner_ty.clone());
                }
            }
            ExprKind::AddrOf(ref mut inner) => {
                self.visit_expr(inner, ctx);
                if let Some(ty) = &inner.ty {
                    expr.ty = Some(Type::Pointer(Box::new(ty.clone())));
                }
            }
            ExprKind::Cast(ref mut inner, ref target_ty) => {
                self.visit_expr(inner, ctx);
                expr.ty = Some(target_ty.clone());
            }

            _ => {}
        }
    }
}

impl Pass for ResolutionPass {
    fn name(&self) -> &str {
        "ResolutionPass"
    }

    fn run(&mut self, ctx: &mut TypeContext) {
        let keys: Vec<_> = ctx.concrete_funcs.keys().cloned().collect();
        for k in keys {
            let mut func = ctx.concrete_funcs.remove(&k).unwrap();
            self.visit_function_def(&mut func, ctx);
            ctx.concrete_funcs.insert(k, func);
        }
    }
}
