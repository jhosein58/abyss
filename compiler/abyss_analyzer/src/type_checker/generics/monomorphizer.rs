use crate::type_checker::context::TypeContext;
use abyss_parser::ast::{Expr, FunctionBody, FunctionDef, Lit, Stmt, StructDef, Type, UnionDef};
use std::collections::HashMap;

pub struct Monomorphizer<'a> {
    pub ctx: &'a mut TypeContext,
}

impl<'a> Monomorphizer<'a> {
    pub fn new(ctx: &'a mut TypeContext) -> Self {
        Self { ctx }
    }

    pub fn monomorphize_struct(
        &mut self,
        template_name: &str,
        concrete_generics: Vec<Type>,
    ) -> String {
        let generics_key = format!("{:?}", concrete_generics);
        let cache_key = (template_name.to_string(), generics_key);

        if let Some(name) = self.ctx.monomorphization_cache.get(&cache_key) {
            return name.clone();
        }

        let template = self
            .ctx
            .generic_struct_templates
            .get(template_name)
            .expect(&format!("Template not found: {}", template_name))
            .clone();
        let new_name = format!(
            "{}_{}",
            template_name,
            self.ctx.monomorphization_cache.len()
        );

        self.ctx
            .monomorphization_cache
            .insert(cache_key.clone(), new_name.clone());

        self.ctx.reverse_struct_map.insert(
            new_name.clone(),
            (template_name.to_string(), concrete_generics.clone()),
        );

        let mut new_struct = template.clone();
        new_struct.name = new_name.clone();
        new_struct.generics.clear();

        let mut map = HashMap::new();
        for (name, ty) in template.generics.iter().zip(concrete_generics.iter()) {
            map.insert(name.clone(), ty.clone());
        }

        for (_, field_ty) in &mut new_struct.fields {
            self.substitute_type_helper(field_ty, &map);

            if let Type::Union(variants) = field_ty {
                let union_struct_name = self.get_or_create_union_struct(variants);
                *field_ty = Type::Struct(vec![union_struct_name], vec![]);
            }
        }

        self.ctx.concrete_structs.push(new_struct);

        new_name
    }

    fn substitute_type_helper(&mut self, ty: &mut Type, map: &HashMap<String, Type>) {
        self.substitute_type(ty, map);
    }

    pub fn get_or_create_union_struct(&mut self, types: &[Type]) -> String {
        let mut sorted_types = types.to_vec();
        sorted_types.sort_by_key(|t| t.get_name());

        let id = self.get_union_name(&sorted_types);

        let struct_name = format!("__Union_{}", id);
        let inner_struct_name = format!("__UnionInner_{}", id);

        if !self.ctx.variant_cache.contains_key(&struct_name) {
            self.ctx
                .variant_cache
                .insert(struct_name.clone(), sorted_types.clone());
        }

        if !self
            .ctx
            .concrete_unions
            .iter()
            .any(|u| u.name == inner_struct_name)
        {
            let mut inner_fields = Vec::new();
            for (i, t) in sorted_types.iter().enumerate() {
                inner_fields.push((format!("variant_{}", i), t.clone()));
            }

            let inner_def = UnionDef {
                is_pub: false,
                name: inner_struct_name.clone(),
                fields: inner_fields,
            };
            self.ctx.concrete_unions.push(inner_def);

            let fields = vec![
                ("tag".to_string(), Type::I64),
                (
                    "data".to_string(),
                    Type::Struct(vec![inner_struct_name.clone()], vec![]),
                ),
            ];

            let struct_def = StructDef {
                is_pub: false,
                name: struct_name.clone(),
                generics: vec![],
                fields,
            };
            self.ctx.union_struct_defs.push(struct_def);
        }

        struct_name
    }

    pub fn substitute_type(&mut self, ty: &mut Type, map: &HashMap<String, Type>) {
        match ty {
            Type::Generic(name) => {
                if let Some(concrete) = map.get(name) {
                    *ty = concrete.clone();
                    self.substitute_type(ty, map);
                }
            }
            Type::Pointer(inner) | Type::Array(inner, _) => {
                self.substitute_type(inner, map);
            }
            Type::Function(args, ret, _) => {
                for arg in args {
                    self.substitute_type(arg, map);
                }
                self.substitute_type(ret, map);
            }
            Type::Struct(path, generics) => {
                for g in generics.iter_mut() {
                    self.substitute_type(g, map);
                }

                if !generics.is_empty() {
                    let struct_name = path.join("__");

                    if self.ctx.generic_struct_templates.contains_key(&struct_name) {
                        let mangled_name = self.monomorphize_struct(&struct_name, generics.clone());

                        *path = vec![mangled_name];
                        generics.clear();
                    }
                }
            }

            Type::Union(variants) => {
                for variant in variants {
                    self.substitute_type(variant, map);
                }
            }
            _ => {}
        }
    }

    fn get_union_name(&mut self, types: &[Type]) -> String {
        let mut types_str = types.iter().map(|t| t.get_name()).collect::<Vec<_>>();
        types_str.sort();
        types_str.join("_")
    }

    pub fn replace_generics_in_func(
        &mut self,
        func: &mut FunctionDef,
        generic_names: &[String],
        concrete_types: &[Type],
    ) {
        let mut map = HashMap::new();
        for (name, ty) in generic_names.iter().zip(concrete_types.iter()) {
            map.insert(name.clone(), ty.clone());
        }

        for (_, ty) in &mut func.params {
            self.substitute_type(ty, &map);
        }
        self.substitute_type(&mut func.return_type, &map);

        if let FunctionBody::UserDefined(stmts) = &mut func.body {
            for stmt in stmts {
                self.substitute_stmt(stmt, &map);
            }
        }
    }

    pub fn substitute_stmt(&mut self, stmt: &mut Stmt, map: &HashMap<String, Type>) {
        match stmt {
            Stmt::Let(_, ty_opt, expr_opt) => {
                if let Some(ty) = ty_opt {
                    self.substitute_type(ty, map);
                }
                if let Some(expr) = expr_opt {
                    self.substitute_expr(expr, map);
                }
            }
            Stmt::Assign(lhs, rhs) => {
                self.substitute_expr(lhs, map);
                self.substitute_expr(rhs, map);
            }
            Stmt::Expr(expr) | Stmt::Ret(expr) => {
                self.substitute_expr(expr, map);
            }
            Stmt::If(cond, then_block, else_block) => {
                self.substitute_expr(cond, map);
                self.substitute_stmt(then_block, map);
                if let Some(e) = else_block {
                    self.substitute_stmt(e, map);
                }
            }
            Stmt::While(cond, body) => {
                self.substitute_expr(cond, map);
                self.substitute_stmt(body, map);
            }
            Stmt::Block(stmts) => {
                for s in stmts {
                    self.substitute_stmt(s, map);
                }
            }
            _ => {}
        }
    }

    fn substitute_expr(&mut self, expr: &mut Expr, map: &HashMap<String, Type>) {
        match expr {
            Expr::Call(callee, args, generics) => {
                self.substitute_expr(callee, map);
                for arg in args {
                    self.substitute_expr(arg, map);
                }
                for g in generics {
                    self.substitute_type(g, map);
                }
            }
            Expr::Binary(l, _, r) => {
                self.substitute_expr(l, map);
                self.substitute_expr(r, map);
            }
            Expr::Unary(_, inner)
            | Expr::Deref(inner)
            | Expr::AddrOf(inner)
            | Expr::Member(inner, _) => {
                self.substitute_expr(inner, map);
            }
            Expr::Cast(inner, ty) => {
                self.substitute_expr(inner, map);
                self.substitute_type(ty, map);
            }

            Expr::StructInit(_, fields, generics) => {
                for (_, e) in fields {
                    self.substitute_expr(e, map);
                }
                for g in generics {
                    self.substitute_type(g, map);
                }
            }
            Expr::SizeOf(ty) => {
                self.substitute_type(ty, map);
            }
            Expr::Lit(Lit::Array(exprs)) => {
                for e in exprs {
                    self.substitute_expr(e, map);
                }
            }
            _ => {}
        }
    }

    pub fn infer_generics_from_args(
        &self,
        generic_names: &[String],
        param_defs: &[(String, Type)],
        arg_types: &[Type],
    ) -> Vec<Type> {
        let mut resolved_map: HashMap<String, Type> = HashMap::new();

        for ((_, param_type), arg_type) in param_defs.iter().zip(arg_types.iter()) {
            self.match_types(param_type, arg_type, &mut resolved_map);
        }

        let mut result = Vec::new();
        for name in generic_names {
            match resolved_map.get(name) {
                Some(ty) => result.push(ty.clone()),
                None => panic!("Could not infer generic type '{}'", name),
            }
        }
        result
    }

    fn match_types(&self, param_ty: &Type, arg_ty: &Type, map: &mut HashMap<String, Type>) {
        match (param_ty, arg_ty) {
            (Type::Generic(name), concrete) => {
                if let Some(existing) = map.get(name) {
                    if existing != concrete {}
                } else {
                    map.insert(name.clone(), concrete.clone());
                }
            }
            (Type::Pointer(p_inner), Type::Pointer(a_inner)) => {
                self.match_types(p_inner, a_inner, map);
            }
            (Type::Array(p_inner, _), Type::Array(a_inner, _)) => {
                self.match_types(p_inner, a_inner, map);
            }

            (Type::Struct(p_path, p_generics), Type::Struct(a_path, a_generics)) => {
                if p_generics.len() == a_generics.len() {
                    for (p, a) in p_generics.iter().zip(a_generics.iter()) {
                        self.match_types(p, a, map);
                    }
                } else {
                    let a_name = a_path.last().unwrap();
                    if let Some((base_name, base_generics)) =
                        self.ctx.reverse_struct_map.get(a_name)
                    {
                        let p_name = p_path.last().unwrap();
                        if p_path.join("__") == *base_name || p_name == base_name {
                            for (p_gen, concrete_gen) in p_generics.iter().zip(base_generics.iter())
                            {
                                self.match_types(p_gen, concrete_gen, map);
                            }
                        }
                    }
                }
            }
            _ => {}
        }
    }
}
