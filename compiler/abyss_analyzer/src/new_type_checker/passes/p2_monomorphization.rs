use crate::new_type_checker::Pass;
use crate::new_type_checker::TypeCheckPass;
use crate::new_type_checker::context::TypeContext;
use crate::new_type_checker::visitor::AstVisitor;
use abyss_parser::ast::{Expr, ExprKind, FunctionBody, Lit, Stmt, StmtKind, Type};
use std::collections::HashMap;

pub struct MonomorphizationPass;

impl MonomorphizationPass {
    pub fn new() -> Self {
        Self
    }

    fn rewrite_existing_concretes(&mut self, ctx: &mut TypeContext) {
        let names: Vec<String> = ctx.concrete_funcs.keys().cloned().collect();
        for name in names {
            if let Some(mut func) = ctx.concrete_funcs.remove(&name) {
                self.rewrite_calls_in_body(&mut func.body, ctx);
                ctx.concrete_funcs.insert(name, func);
            }
        }
    }

    fn rewrite_calls_in_body(&self, body: &mut FunctionBody, ctx: &mut TypeContext) {
        if let FunctionBody::UserDefined(stmts) = body {
            for stmt in stmts {
                self.rewrite_calls_in_stmt(stmt, ctx);
            }
        }
    }

    fn rewrite_calls_in_stmt(&self, stmt: &mut Stmt, ctx: &mut TypeContext) {
        match &mut stmt.kind {
            StmtKind::Let(_, _, Some(expr)) | StmtKind::Expr(expr) | StmtKind::Ret(expr) => {
                self.rewrite_calls_in_expr(expr, ctx);
            }
            StmtKind::Assign(lhs, rhs) => {
                self.rewrite_calls_in_expr(lhs, ctx);
                self.rewrite_calls_in_expr(rhs, ctx);
            }
            StmtKind::Block(stmts) => {
                for s in stmts {
                    self.rewrite_calls_in_stmt(s, ctx);
                }
            }
            StmtKind::If(cond, then_b, else_b) => {
                self.rewrite_calls_in_expr(cond, ctx);
                self.rewrite_calls_in_stmt(then_b, ctx);
                if let Some(else_s) = else_b {
                    self.rewrite_calls_in_stmt(else_s, ctx);
                }
            }
            StmtKind::While(cond, body) => {
                self.rewrite_calls_in_expr(cond, ctx);
                self.rewrite_calls_in_stmt(body, ctx);
            }
            _ => {}
        }
    }

    fn rewrite_calls_in_expr(&self, expr: &mut Expr, ctx: &mut TypeContext) {
        match &mut expr.kind {
            ExprKind::Binary(l, _, r) => {
                self.rewrite_calls_in_expr(l, ctx);
                self.rewrite_calls_in_expr(r, ctx);
            }
            ExprKind::Unary(_, e)
            | ExprKind::Cast(e, _)
            | ExprKind::Deref(e)
            | ExprKind::AddrOf(e)
            | ExprKind::Member(e, _) => {
                self.rewrite_calls_in_expr(e, ctx);
            }
            ExprKind::StructInit(_, fields, _) => {
                for (_, e) in fields {
                    self.rewrite_calls_in_expr(e, ctx);
                }
            }
            ExprKind::Index(arr, idx) => {
                self.rewrite_calls_in_expr(arr, ctx);
                self.rewrite_calls_in_expr(idx, ctx);
            }
            ExprKind::Lit(Lit::Array(elems)) => {
                for e in elems {
                    self.rewrite_calls_in_expr(e, ctx);
                }
            }
            ExprKind::Call(callee, args, _) => {
                self.rewrite_calls_in_expr(callee, ctx);
                for arg in args.iter_mut() {
                    self.rewrite_calls_in_expr(arg, ctx);
                }

                if let ExprKind::Ident(path) = &mut callee.kind {
                    if let Some(func_name) = path.last() {
                        if ctx.get_generic_func_template(func_name).is_some() {
                            let arg_types: Vec<Type> = args
                                .iter()
                                .map(|arg| {
                                    arg.ty
                                        .clone()
                                        .expect("Layer 1 (TypeCheck) must run before Layer 2")
                                })
                                .collect();

                            let mangled = self.mangle_name(func_name, &arg_types);

                            *path = vec![mangled];
                        }
                    }
                }
            }
            _ => {}
        }
    }

    fn instantiate_function(
        &self,
        name: String,
        concrete_args: Vec<Type>,
        ctx: &mut TypeContext,
        checker: &mut TypeCheckPass,
    ) {
        let mangled_name = self.mangle_name(&name, &concrete_args);

        if ctx.concrete_funcs.contains_key(&mangled_name) {
            return;
        }

        let template = ctx
            .get_generic_func_template(&name)
            .expect("Template not found")
            .clone();

        if template.generics.len() != concrete_args.len() {
            panic!("Generic count mismatch during instantiation of {}", name);
        }

        let mut type_map = HashMap::new();
        for (gen_name, concrete_ty) in template.generics.iter().zip(concrete_args.iter()) {
            type_map.insert(gen_name.clone(), concrete_ty.clone());
        }

        let mut new_func = template.clone();
        new_func.name = mangled_name.clone();
        new_func.generics.clear();

        for param in &mut new_func.params {
            param.1 = self.substitute_type(&param.1, &type_map, ctx);
        }
        new_func.return_type = self.substitute_type(&new_func.return_type, &type_map, ctx);

        if let FunctionBody::UserDefined(ref mut stmts) = new_func.body {
            for stmt in stmts {
                self.substitute_stmt(stmt, &type_map, ctx);
            }
        }

        checker.visit_function_def(&mut new_func, ctx);

        self.rewrite_calls_in_body(&mut new_func.body, ctx);

        ctx.concrete_funcs.insert(mangled_name, new_func);
    }

    fn instantiate_struct(&self, name: String, args: Vec<Type>, ctx: &mut TypeContext) {
        let mangled_name = self.mangle_name(&name, &args);

        if ctx.concrete_structs.contains_key(&mangled_name) {
            return;
        }

        let template = match ctx.get_generic_struct_template(&name) {
            Some(t) => t.clone(),
            None => panic!("Generic struct template '{}' not found!", name),
        };

        let mut type_map = HashMap::new();
        for (gen_name, concrete_ty) in template.generics.iter().zip(args.iter()) {
            type_map.insert(gen_name.clone(), concrete_ty.clone());
        }

        let mut new_struct = template.clone();
        new_struct.name = mangled_name.clone();
        new_struct.generics.clear();

        for (_, field_ty) in &mut new_struct.fields {
            *field_ty = self.substitute_type(field_ty, &type_map, ctx);
        }

        ctx.concrete_structs.insert(mangled_name, new_struct);
    }

    fn mangle_name(&self, name: &str, args: &Vec<Type>) -> String {
        let mut s = String::from(name);
        for arg in args {
            s.push_str("__");
            s.push_str(&self.mangle_type(arg));
        }
        s
    }

    fn mangle_type(&self, ty: &Type) -> String {
        ty.get_name()
    }
    fn substitute_type(
        &self,
        ty: &Type,
        map: &HashMap<String, Type>,
        ctx: &mut TypeContext,
    ) -> Type {
        match ty {
            Type::Generic(name) => map.get(name).cloned().unwrap_or(ty.clone()),

            Type::Pointer(inner) => Type::Pointer(Box::new(self.substitute_type(inner, map, ctx))),

            Type::Array(inner, size) => {
                Type::Array(Box::new(self.substitute_type(inner, map, ctx)), *size)
            }

            Type::Struct(path, generics) => {
                if path.len() == 1 && generics.is_empty() {
                    if let Some(concrete_ty) = map.get(&path[0]) {
                        return concrete_ty.clone();
                    }
                }

                let new_generics: Vec<Type> = generics
                    .iter()
                    .map(|g| self.substitute_type(g, map, ctx))
                    .collect();

                let struct_name = path.last().unwrap();

                if ctx.get_generic_struct_template(struct_name).is_some()
                    && !new_generics.is_empty()
                {
                    ctx.register_generic_struct_request(struct_name.clone(), new_generics.clone());
                    let mangled = self.mangle_name(struct_name, &new_generics);
                    return Type::Struct(vec![mangled], vec![]);
                }

                Type::Struct(path.clone(), new_generics)
            }

            Type::Function(params, ret, gens) => {
                let new_params = params
                    .iter()
                    .map(|p| self.substitute_type(p, map, ctx))
                    .collect();
                let new_ret = Box::new(self.substitute_type(ret, map, ctx));
                Type::Function(new_params, new_ret, gens.clone())
            }

            _ => ty.clone(),
        }
    }

    fn substitute_stmt(&self, stmt: &mut Stmt, map: &HashMap<String, Type>, ctx: &mut TypeContext) {
        match &mut stmt.kind {
            StmtKind::Let(_, ty_opt, expr_opt) => {
                if let Some(ty) = ty_opt {
                    *ty = self.substitute_type(ty, map, ctx);
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
        if let Some(ty) = &expr.ty {
            expr.ty = Some(self.substitute_type(ty, map, ctx));
        }

        match &mut expr.kind {
            ExprKind::Binary(l, _, r) => {
                self.substitute_expr(l, map, ctx);
                self.substitute_expr(r, map, ctx);
            }
            ExprKind::Unary(_, o) | ExprKind::Deref(o) | ExprKind::AddrOf(o) => {
                self.substitute_expr(o, map, ctx);
            }
            ExprKind::Call(callee, args, generics) => {
                self.substitute_expr(callee, map, ctx);
                for arg in args {
                    self.substitute_expr(arg, map, ctx);
                }
                for g in generics {
                    *g = self.substitute_type(g, map, ctx);
                }
            }
            ExprKind::StructInit(_, fields, generics) => {
                for (_, e) in fields {
                    self.substitute_expr(e, map, ctx);
                }
                for g in generics {
                    *g = self.substitute_type(g, map, ctx);
                }
            }
            ExprKind::Index(arr, idx) => {
                self.substitute_expr(arr, map, ctx);
                self.substitute_expr(idx, map, ctx);
            }
            ExprKind::Cast(inner, target_ty) => {
                self.substitute_expr(inner, map, ctx);
                *target_ty = self.substitute_type(target_ty, map, ctx);
            }
            ExprKind::Member(obj, _) => {
                self.substitute_expr(obj, map, ctx);
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
        self.rewrite_existing_concretes(ctx);

        let mut checker = TypeCheckPass::new();

        while ctx.has_pending_instantiations() {
            let mut struct_requests = Vec::new();
            while let Some(req) = ctx.pop_pending_struct_request() {
                struct_requests.push(req);
            }
            for (name, args) in struct_requests {
                self.instantiate_struct(name, args, ctx);
            }

            let mut func_requests = Vec::new();
            while let Some(req) = ctx.pop_pending_func_request() {
                func_requests.push(req);
            }
            for (name, args) in func_requests {
                self.instantiate_function(name, args, ctx, &mut checker);
            }
        }
    }
}
