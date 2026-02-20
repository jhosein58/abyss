pub mod calls;
pub mod context;
pub mod generics;
pub mod unions;

use crate::{
    hir::FlatProgram,
    type_checker::{
        context::TypeContext,
        generics::{monomorphizer::Monomorphizer, resolver::GenericResolver},
    },
};
use abyss_parser::ast::{BinaryOp, Expr, FunctionBody, FunctionDef, Lit, Stmt, Type};
use std::collections::HashMap;

pub struct TypeChecker {
    ctx: TypeContext,
    generic_resolver: GenericResolver,
}

impl TypeChecker {
    pub fn new() -> Self {
        Self {
            ctx: TypeContext::new(),
            generic_resolver: GenericResolver,
        }
    }

    fn mono(&mut self) -> Monomorphizer<'_> {
        Monomorphizer::new(&mut self.ctx)
    }

    fn are_types_compatible(&self, target: &Type, source: &Type) -> bool {
        if target == source {
            return true;
        }

        match (target, source) {
            (Type::Pointer(_), Type::Pointer(inner)) if **inner == Type::Void => true,

            (Type::Pointer(inner), Type::Pointer(_)) if **inner == Type::Void => true,

            (Type::Union(variants), src_ty) => variants.contains(src_ty),

            _ => false,
        }
    }
    fn is_integer(&self, t: &Type) -> bool {
        matches!(
            t,
            Type::I8
                | Type::I16
                | Type::I32
                | Type::I64
                | Type::Isize
                | Type::U8
                | Type::U16
                | Type::U32
                | Type::U64
                | Type::Usize
                | Type::Char
        )
    }

    fn is_float(&self, t: &Type) -> bool {
        matches!(t, Type::F32 | Type::F64)
    }
    pub fn check(mut self, mut program: FlatProgram) -> FlatProgram {
        for ty in program.type_aliases {
            self.ctx.register_type_alias(ty.name, ty.ty);
        }

        for mut s in program.structs {
            if !s.generics.is_empty() {
                self.generic_resolver.resolve_struct(&mut s);
                self.ctx.generic_struct_templates.insert(s.name.clone(), s);
            } else {
                let empty_map = HashMap::new();
                for (_, field_ty) in &mut s.fields {
                    self.mono().substitute_type(field_ty, &empty_map);

                    self.materialize_union_type(field_ty)
                }
                self.ctx.concrete_structs.push(s);
            }
        }

        for mut func in program.functions {
            if !func.generics.is_empty() {
                self.generic_resolver.resolve_func(&mut func);
                self.ctx
                    .generic_func_templates
                    .insert(func.name.clone(), func);
            } else {
                self.ctx.pending_funcs.push_back(func);
            }
        }

        for static_def in &mut program.statics {
            if let Type::Struct(path, generics) = &mut static_def.ty {
                if !generics.is_empty() {
                    let struct_name = path.join("__");

                    if self.ctx.generic_struct_templates.contains_key(&struct_name) {
                        let concrete_name = self
                            .mono()
                            .monomorphize_struct(&struct_name, generics.clone());

                        static_def.ty = Type::Struct(vec![concrete_name], vec![]);
                    }
                }
            }

            self.ctx
                .register_var(static_def.name.clone(), static_def.ty.clone());

            let (new_expr, _expr_ty) = self.infer_expr(static_def.value.clone());
            static_def.value = new_expr;
        }

        while let Some(mut func) = self.ctx.pending_funcs.pop_front() {
            let empty_map: HashMap<String, Type> = HashMap::new();

            for (_, param_ty) in &mut func.params {
                self.mono().substitute_type(param_ty, &empty_map);
            }

            self.mono()
                .substitute_type(&mut func.return_type, &empty_map);

            if let FunctionBody::UserDefined(stmts) = &mut func.body {
                for stmt in stmts {
                    self.mono().substitute_stmt(stmt, &empty_map);
                }
            }

            self.check_function(&mut func);
            self.ctx.concrete_funcs.push(func);
        }

        let mut new_program = FlatProgram::new();
        new_program.functions = self.ctx.concrete_funcs;
        new_program.structs = self.ctx.concrete_structs;
        new_program.statics = program.statics;
        new_program.unions = self.ctx.concrete_unions;
        new_program.union_struct_defs = self.ctx.union_struct_defs;
        new_program
    }

    fn check_function(&mut self, func: &mut FunctionDef) {
        self.ctx.enter_scope();

        for (param_name, param_type) in &mut func.params {
            if let Type::Union(variants) = param_type {
                let struct_name = self.mono().get_or_create_union_struct(variants);

                *param_type = Type::Struct(vec![struct_name], vec![]);
            }

            self.ctx
                .register_var(param_name.clone(), param_type.clone());
        }

        self.materialize_union_type(&mut func.return_type);

        if let FunctionBody::UserDefined(ref mut stmts) = func.body {
            self.check_stmts(stmts);
        }

        self.ctx.exit_scope();
    }
    fn check_stmts(&mut self, stmts: &mut [Stmt]) {
        for stmt in stmts {
            self.check_stmt(stmt);
        }
    }

    fn check_stmt(&mut self, stmt: &mut Stmt) {
        match stmt {
            Stmt::Let(name, ty_opt, expr_opt) => {
                if let Some(expr) = expr_opt {
                    let (mut new_expr, mut expr_ty) = self.infer_expr(expr.clone());

                    if let Some(Type::Union(variants)) = ty_opt {
                        if !variants.contains(&expr_ty) {
                            for variant in variants.iter() {
                                let is_int_conv =
                                    self.is_integer(variant) && self.is_integer(&expr_ty);
                                let is_float_conv =
                                    self.is_float(variant) && self.is_float(&expr_ty);

                                if is_int_conv || is_float_conv {
                                    new_expr = Expr::Cast(Box::new(new_expr), variant.clone());
                                    expr_ty = variant.clone();
                                    break;
                                }
                            }
                        }
                    }

                    let needs_wrapping = if let Some(Type::Union(variants)) = ty_opt {
                        variants.contains(&expr_ty)
                    } else {
                        false
                    };

                    if needs_wrapping {
                        if let Some(Type::Union(variants)) = ty_opt {
                            let (wrapped_expr, concrete_ty) =
                                self.wrap_let_union_init(new_expr, &expr_ty, variants);

                            new_expr = wrapped_expr;
                            *ty_opt = Some(concrete_ty.clone());
                            expr_ty = concrete_ty;
                        }
                    }

                    *expr = new_expr;

                    match ty_opt {
                        Some(explicit_ty) => {
                            let mut types_match = self.are_types_compatible(explicit_ty, &expr_ty);

                            if !types_match {
                                if let Type::Struct(explicit_path, explicit_generics) = explicit_ty
                                {
                                    if let Type::Struct(expr_path, _) = &expr_ty {
                                        if let Some(concrete_name) = expr_path.last() {
                                            if let Some((base_name, stored_generics)) =
                                                self.ctx.reverse_struct_map.get(concrete_name)
                                            {
                                                let explicit_name_str = explicit_path.join("__");

                                                if &explicit_name_str == base_name
                                                    && explicit_generics == stored_generics
                                                {
                                                    *explicit_ty = expr_ty.clone();
                                                    types_match = true;
                                                }
                                            }
                                        }
                                    }
                                }
                            }

                            if !types_match {
                                let is_int_conversion =
                                    self.is_integer(explicit_ty) && self.is_integer(&expr_ty);
                                let is_float_conversion =
                                    self.is_float(explicit_ty) && self.is_float(&expr_ty);

                                if is_int_conversion || is_float_conversion {
                                    *expr = Expr::Cast(Box::new(expr.clone()), explicit_ty.clone());
                                } else {
                                    let mut has_problem = false;
                                    // If the struct type doesn't match, check type aliases.
                                    if let Type::Struct(name, _) = explicit_ty {
                                        let alias_name = name[0].clone();
                                        if let Some(ty) =
                                            self.ctx.get_type_alias(alias_name).cloned()
                                        {
                                            if self.are_types_compatible(&ty, &expr_ty) {
                                                match ty {
                                                    Type::Union(ref variants) => {
                                                        let struct_name = self
                                                            .mono()
                                                            .get_or_create_union_struct(variants);
                                                        let concrete_ty = Type::Struct(
                                                            vec![struct_name.clone()],
                                                            vec![],
                                                        );

                                                        let (wrapped, _) = self
                                                            .wrap_let_union_init(
                                                                expr.clone(),
                                                                &expr_ty,
                                                                variants,
                                                            );
                                                        *expr = wrapped;
                                                        *explicit_ty = concrete_ty;
                                                    }
                                                    _ => {
                                                        *explicit_ty = ty;
                                                    }
                                                }
                                            } else {
                                                has_problem = true
                                            }
                                        } else {
                                            has_problem = true
                                        }
                                    } else {
                                        has_problem = true
                                    }
                                    if has_problem {
                                        panic!(
                                            "Type mismatch in let binding for '{}': expected {:?}, found {:?}",
                                            name, explicit_ty, expr_ty
                                        );
                                    }
                                }
                            }
                        }
                        None => {
                            *ty_opt = Some(expr_ty.clone());
                        }
                    };

                    let final_ty = ty_opt.as_ref().unwrap().clone();
                    self.ctx.register_var(name.clone(), final_ty);
                } else {
                    match ty_opt {
                        Some(Type::Union(variants)) => {
                            let struct_name = self.mono().get_or_create_union_struct(variants);
                            let concrete_ty = Type::Struct(vec![struct_name], vec![]);
                            self.ctx.register_var(name.clone(), concrete_ty.clone());
                            *ty_opt = Some(concrete_ty);
                        }
                        Some(explicit_ty) => {
                            self.ctx.register_var(name.clone(), explicit_ty.clone());
                        }
                        None => {
                            panic!(
                                "Type annotation required for uninitialized variable '{}'",
                                name
                            );
                        }
                    }
                }
            }

            Stmt::Expr(expr) => {
                let (new_expr, _) = self.infer_expr(expr.clone());
                *stmt = Stmt::Expr(new_expr);
            }
            Stmt::Ret(expr) => {
                let (new_expr, _) = self.infer_expr(expr.clone());
                *stmt = Stmt::Ret(new_expr);
            }
            Stmt::If(cond, then_block, else_block) => {
                let (new_cond, _) = self.infer_expr(cond.clone());
                *cond = new_cond;
                self.check_stmt(then_block);
                if let Some(else_b) = else_block {
                    self.check_stmt(else_b);
                }
            }
            Stmt::While(cond, body) => {
                let (new_cond, _) = self.infer_expr(cond.clone());
                *cond = new_cond;
                self.check_stmt(body);
            }
            Stmt::Block(inner_stmts) => {
                self.ctx.enter_scope();
                self.check_stmts(inner_stmts);
                self.ctx.exit_scope();
            }
            Stmt::FunctionDef(func_def) => {
                self.ctx
                    .register_local_func(func_def.name.clone(), *func_def.clone());

                self.check_function(func_def);
            }
            _ => {}
        }
    }

    fn infer_expr(&mut self, expr: Expr) -> (Expr, Type) {
        match expr {
            Expr::Binary(lhs, BinaryOp::Assign, rhs) => {
                let (new_lhs, lhs_ty) = self.infer_expr(*lhs);
                let (new_rhs, rhs_ty) = self.infer_expr(*rhs);
                let new_rhs = self.try_wrap_rhs_for_union(new_rhs, &rhs_ty, &lhs_ty);
                (
                    Expr::Binary(Box::new(new_lhs), BinaryOp::Assign, Box::new(new_rhs)),
                    lhs_ty,
                )
            }

            Expr::Lit(lit) => self.infer_lit(lit),

            Expr::Ident(path) => {
                let name = path.last().unwrap();
                if let Some(ty) = self.ctx.get_var_type(name) {
                    (Expr::Ident(path), ty)
                } else {
                    panic!("Undefined variable: {}", name);
                }
            }

            Expr::Call(callee, args, generics) => {
                self.handle_function_call(*callee, args, generics)
            }

            Expr::Binary(lhs, op, rhs) => {
                let (new_lhs, ty_lhs) = self.infer_expr(*lhs);
                let (new_rhs, _) = self.infer_expr(*rhs);

                match op {
                    BinaryOp::Eq
                    | BinaryOp::Neq
                    | BinaryOp::Lt
                    | BinaryOp::Gt
                    | BinaryOp::Lte
                    | BinaryOp::Gte => (
                        Expr::Binary(Box::new(new_lhs), op, Box::new(new_rhs)),
                        Type::Bool,
                    ),
                    _ => (
                        Expr::Binary(Box::new(new_lhs), op, Box::new(new_rhs)),
                        ty_lhs,
                    ),
                }
            }
            Expr::Is(inner, check_ty) => {
                let (new_inner, inner_ty) = self.infer_expr(*inner);
                self.resolve_is_expr_with_type(new_inner, inner_ty, check_ty)
            }
            Expr::Cast(inner, target_ty) => {
                let (new_inner, inner_ty) = self.infer_expr(*inner);

                if let Some(result) =
                    self.resolve_union_cast(new_inner.clone(), inner_ty.clone(), target_ty.clone())
                {
                    return result;
                }

                (
                    Expr::Cast(Box::new(new_inner), target_ty.clone()),
                    target_ty,
                )
            }

            Expr::SizeOf(ty) => (Expr::SizeOf(ty.clone()), Type::I64),

            Expr::Index(arr, idx) => {
                let (new_arr, arr_ty) = self.infer_expr(*arr);
                let (new_idx, _) = self.infer_expr(*idx);

                let elem_ty = match arr_ty {
                    Type::Array(inner, _) => *inner,
                    Type::Pointer(inner) => *inner,
                    _ => panic!("Cannot index type {:?}", arr_ty),
                };

                (Expr::Index(Box::new(new_arr), Box::new(new_idx)), elem_ty)
            }

            Expr::StructInit(path, fields, generics) => {
                let mut resolved_fields = Vec::new();
                for (name, val) in fields {
                    let (new_val, ty) = self.infer_expr(val);
                    resolved_fields.push((name, new_val, ty));
                }

                let struct_name = path.join("__");
                let final_struct_name;

                if self.ctx.generic_struct_templates.contains_key(&struct_name) {
                    let final_generics: Vec<Type>;
                    if !generics.is_empty() {
                        final_generics = generics;
                    } else {
                        panic!(
                            "Implicit struct generics logic needed here or explicit generics required"
                        );
                    }
                    final_struct_name = self
                        .mono()
                        .monomorphize_struct(&struct_name, final_generics);
                } else {
                    final_struct_name = struct_name;
                }

                let target_def = self
                    .ctx
                    .concrete_structs
                    .iter()
                    .find(|s| s.name == final_struct_name)
                    .cloned();

                let mut final_fields = Vec::new();

                if let Some(def) = target_def {
                    for (f_name, mut f_expr, f_ty) in resolved_fields {
                        if let Some((_, expected_ty)) =
                            def.fields.iter().find(|(n, _)| n == &f_name)
                        {
                            f_expr = self.try_wrap_struct_field(f_expr, &f_ty, expected_ty)
                        }
                        final_fields.push((f_name, f_expr));
                    }
                } else {
                    for (n, e, _) in resolved_fields {
                        final_fields.push((n, e));
                    }
                }

                (
                    Expr::StructInit(vec![final_struct_name.clone()], final_fields, vec![]),
                    Type::Struct(vec![final_struct_name], vec![]),
                )
            }

            Expr::Member(inner, field_name) => {
                let (new_inner, mut inner_ty) = self.infer_expr(*inner);

                let mut current_expr = new_inner;

                while let Type::Pointer(pointed_to) = inner_ty.clone() {
                    inner_ty = *pointed_to;
                    current_expr = Expr::Deref(Box::new(current_expr));
                }

                if let Type::Struct(path, _) = inner_ty {
                    let struct_name = path.last().unwrap();
                    let def = self
                        .ctx
                        .concrete_structs
                        .iter()
                        .find(|s| &s.name == struct_name)
                        .expect("Struct definition not found");

                    let (_, field_ty) = def
                        .fields
                        .iter()
                        .find(|(n, _)| n == &field_name)
                        .expect("Field not found");

                    (
                        Expr::Member(Box::new(current_expr), field_name),
                        field_ty.clone(),
                    )
                } else {
                    panic!("Accessing member of non-struct type");
                }
            }
            Expr::MethodCall(receiver, method_name, args, generics) => {
                self.resolve_method_call(receiver, method_name, args, generics)
            }

            _ => (expr, Type::Void),
        }
    }

    fn infer_lit(&mut self, lit: Lit) -> (Expr, Type) {
        match lit {
            Lit::Int(_) => (Expr::Lit(lit), Type::I64),
            Lit::Float(_) => (Expr::Lit(lit), Type::F64),
            Lit::Bool(_) => (Expr::Lit(lit), Type::Bool),
            Lit::Str(ref s) => {
                let len = s.len() - 1;
                (Expr::Lit(lit), Type::Array(Box::new(Type::U8), len))
            }

            Lit::Array(exprs) => {
                if exprs.is_empty() {
                    return (
                        Expr::Lit(Lit::Array(exprs)),
                        Type::Array(Box::new(Type::Void), 0),
                    );
                }

                let mut new_exprs = Vec::new();
                let mut first_ty = None;

                for expr in exprs {
                    let (new_expr, ty) = self.infer_expr(expr);
                    new_exprs.push(new_expr);

                    if first_ty.is_none() {
                        first_ty = Some(ty);
                    } else if first_ty.as_ref() != Some(&ty) {
                        panic!(
                            "Array elements mismatch: expected {:?}, found {:?}",
                            first_ty, ty
                        );
                    }
                }

                let len = new_exprs.len();
                let elem_ty = first_ty.unwrap();

                (
                    Expr::Lit(Lit::Array(new_exprs)),
                    Type::Array(Box::new(elem_ty), len),
                )
            }

            Lit::Null => (Expr::Lit(lit), Type::Pointer(Box::new(Type::Void))),
        }
    }
}
