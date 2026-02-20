use super::resolution::ResolutionPass;
use super::utils::GenericsEngine;
use crate::new_type_checker::{Pass, context::TypeContext};
use abyss_parser::ast::{Expr, ExprKind, FunctionBody, Lit, Stmt, StmtKind, Type};
use std::collections::HashMap;

pub struct MonomorphizationPass;

impl MonomorphizationPass {
    pub fn new() -> Self {
        Self
    }

    fn instantiate_function(
        &mut self,
        name: String,
        concrete_args: Vec<Type>,
        ctx: &mut TypeContext,
    ) {
        let mangled_name = GenericsEngine::mangle_name(&name, &concrete_args);

        if ctx.concrete_funcs.contains_key(&mangled_name) {
            return;
        }

        let template = match ctx.generic_func_templates.get(&name) {
            Some(t) => t.clone(),
            None => return,
        };

        if template.generics.len() != concrete_args.len() {
            return;
        }

        let mut type_map = HashMap::new();
        for (gen_name, concrete_ty) in template.generics.iter().zip(concrete_args.iter()) {
            type_map.insert(gen_name.clone(), concrete_ty.clone());
        }

        let mut new_func = template.clone();
        new_func.name = mangled_name.clone();
        new_func.generics.clear();

        // 1. Substitute Signature
        for param in &mut new_func.params {
            param.1 = GenericsEngine::substitute_generics_mut(&param.1, &type_map, ctx);
        }
        new_func.return_type =
            GenericsEngine::substitute_generics_mut(&new_func.return_type, &type_map, ctx);

        // 2. Substitute Body
        if let FunctionBody::UserDefined(ref mut stmts) = new_func.body {
            for stmt in stmts {
                self.substitute_stmt(stmt, &type_map, ctx);
            }
        }

        // 3. Register Definition first (to allow recursion)
        ctx.concrete_funcs
            .insert(mangled_name.clone(), new_func.clone());

        // 4. Run Resolution Pass on the new function to discover nested generics
        let mut resolution = ResolutionPass::new();
        // We fetch the mutable reference from context to visit it
        if let Some(mut registered_func) = ctx.concrete_funcs.remove(&mangled_name) {
            resolution.visit_function_def(&mut registered_func, ctx);
            ctx.concrete_funcs.insert(mangled_name, registered_func);
        }
    }

    fn instantiate_struct(&self, name: String, concrete_args: Vec<Type>, ctx: &mut TypeContext) {
        let mangled_name = GenericsEngine::mangle_name(&name, &concrete_args);

        if ctx.concrete_structs.contains_key(&mangled_name) {
            return;
        }

        let template = match ctx.generic_struct_templates.get(&name) {
            Some(t) => t.clone(),
            None => return,
        };

        let mut type_map = HashMap::new();
        for (gen_name, concrete_ty) in template.generics.iter().zip(concrete_args.iter()) {
            type_map.insert(gen_name.clone(), concrete_ty.clone());
        }

        let mut new_struct = template.clone();
        new_struct.name = mangled_name.clone();
        new_struct.generics.clear();

        for (_, field_ty) in &mut new_struct.fields {
            *field_ty = GenericsEngine::substitute_generics_mut(field_ty, &type_map, ctx);
        }

        ctx.concrete_structs.insert(mangled_name, new_struct);
    }

    // --- AST Traversal for Substitution ---

    fn substitute_stmt(&self, stmt: &mut Stmt, map: &HashMap<String, Type>, ctx: &mut TypeContext) {
        match &mut stmt.kind {
            StmtKind::Let(_, ty_opt, expr_opt) => {
                if let Some(ty) = ty_opt {
                    *ty = GenericsEngine::substitute_generics_mut(ty, map, ctx);
                }
                if let Some(expr) = expr_opt {
                    self.substitute_expr(expr, map, ctx);
                }
            }
            StmtKind::Assign(lhs, rhs) => {
                self.substitute_expr(lhs, map, ctx);
                self.substitute_expr(rhs, map, ctx);
            }
            StmtKind::Expr(expr) | StmtKind::Ret(expr) => {
                self.substitute_expr(expr, map, ctx);
            }
            StmtKind::Block(stmts) => {
                for s in stmts {
                    self.substitute_stmt(s, map, ctx);
                }
            }
            StmtKind::If(cond, then_b, else_b) => {
                self.substitute_expr(cond, map, ctx);
                self.substitute_stmt(then_b, map, ctx);
                if let Some(else_s) = else_b {
                    self.substitute_stmt(else_s, map, ctx);
                }
            }
            StmtKind::While(cond, body) => {
                self.substitute_expr(cond, map, ctx);
                self.substitute_stmt(body, map, ctx);
            }
            _ => {}
        }
    }

    fn substitute_expr(&self, expr: &mut Expr, map: &HashMap<String, Type>, ctx: &mut TypeContext) {
        // Substitute the expression's own type if present (from template)
        if let Some(ty) = &expr.ty {
            expr.ty = Some(GenericsEngine::substitute_generics_mut(ty, map, ctx));
        }

        match &mut expr.kind {
            ExprKind::Binary(l, _, r) => {
                self.substitute_expr(l, map, ctx);
                self.substitute_expr(r, map, ctx);
            }
            ExprKind::Unary(_, o)
            | ExprKind::Deref(o)
            | ExprKind::AddrOf(o)
            | ExprKind::Member(o, _) => {
                self.substitute_expr(o, map, ctx);
            }
            ExprKind::Call(callee, args, generics) => {
                self.substitute_expr(callee, map, ctx);
                for arg in args {
                    self.substitute_expr(arg, map, ctx);
                }
                for g in generics {
                    *g = GenericsEngine::substitute_generics_mut(g, map, ctx);
                }
            }
            ExprKind::StructInit(_, fields, generics) => {
                for (_, e) in fields {
                    self.substitute_expr(e, map, ctx);
                }
                for g in generics {
                    *g = GenericsEngine::substitute_generics_mut(g, map, ctx);
                }
            }
            ExprKind::Index(arr, idx) => {
                self.substitute_expr(arr, map, ctx);
                self.substitute_expr(idx, map, ctx);
            }
            ExprKind::Cast(inner, target_ty) => {
                self.substitute_expr(inner, map, ctx);
                *target_ty = GenericsEngine::substitute_generics_mut(target_ty, map, ctx);
            }
            ExprKind::Lit(Lit::Array(elems)) => {
                for e in elems {
                    self.substitute_expr(e, map, ctx);
                }
            }
            _ => {}
        }
    }
}

impl Pass for MonomorphizationPass {
    fn name(&self) -> &str {
        "MonomorphizationPass"
    }

    fn run(&mut self, ctx: &mut TypeContext) {
        // Loop until no new generic requests are generated
        while ctx.has_pending_instantiations() {
            // 1. Process Struct Requests
            let mut struct_requests = Vec::new();
            while let Some(req) = ctx.pop_pending_struct_request() {
                struct_requests.push(req);
            }
            for (name, args) in struct_requests {
                self.instantiate_struct(name, args, ctx);
            }

            // 2. Process Function Requests
            let mut func_requests = Vec::new();
            while let Some(req) = ctx.pop_pending_func_request() {
                func_requests.push(req);
            }
            for (name, args) in func_requests {
                self.instantiate_function(name, args, ctx);
            }
        }
    }
}
