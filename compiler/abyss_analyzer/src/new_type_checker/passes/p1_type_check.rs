use std::collections::HashMap;

use abyss_parser::ast::{
    BinaryOp as BinOp, Expr, ExprKind, FunctionBody, FunctionDef, Lit, Stmt, StmtKind, Type,
    UnaryOp,
};

use crate::new_type_checker::{Pass, context::TypeContext, visitor::AstVisitor};

pub struct TypeCheckPass;

impl TypeCheckPass {
    pub fn new() -> Self {
        TypeCheckPass {}
    }
}

impl AstVisitor for TypeCheckPass {
    fn visit_function_def(&mut self, func: &mut FunctionDef, ctx: &mut TypeContext) {
        if let FunctionBody::UserDefined(body) = &mut func.body {
            ctx.set_current_function(func.name.clone());

            for (param_name, param_type) in &func.params {
                if let Err(e) = ctx.define_symbol(param_name.clone(), param_type.clone()) {
                    panic!("Error defining function argument '{}': {}", param_name, e);
                }
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

                if let None = ty {
                    *ty = expr.ty.clone()
                }

                if let Err(s) =
                    ctx.define_symbol(name.to_string(), ty.clone().expect("cannot infer type."))
                {
                    panic!("{s}")
                }
            }

            StmtKind::Assign(ref mut l, ref mut r) => {
                self.visit_expr(l, ctx);
                self.visit_expr(r, ctx);
            }

            StmtKind::Ret(ref mut expr) => {
                self.visit_expr(expr, ctx);
            }

            StmtKind::Block(ref mut stmts) => {
                ctx.enter_scope();
                for stmt in stmts {
                    self.visit_stmt(stmt, ctx);
                }
                ctx.exit_scope();
            }

            StmtKind::If(ref mut cond, ref mut then_branch, ref mut else_branch) => {
                self.visit_expr(cond, ctx);
                self.visit_stmt(then_branch, ctx);

                if let Some(else_branch) = else_branch {
                    self.visit_stmt(else_branch, ctx);
                }
            }

            StmtKind::While(ref mut cond, ref mut body) => {
                self.visit_expr(cond, ctx);
                self.visit_stmt(body, ctx);
            }

            StmtKind::Expr(ref mut expr) => {
                self.visit_expr(expr, ctx);
            }

            _ => {}
        }
    }
}

impl TypeCheckPass {
    pub fn visit_expr(&mut self, expr: &mut Expr, ctx: &mut TypeContext) {
        match expr.kind {
            ExprKind::Lit(ref mut lit) => {
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
                        .map(|(_, t)| self.resolve_generic_references(t, &func_def.generics))
                        .collect();

                    let return_type =
                        self.resolve_generic_references(&func_def.return_type, &func_def.generics);

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
                } else {
                    panic!("Undefined variable or symbol: {}", name);
                }
            }
            ExprKind::Binary(ref mut left, ref op, ref mut right) => {
                self.visit_expr(left, ctx);
                self.visit_expr(right, ctx);
                expr.ty = Some(self.check_binary(left, op, right));
            }
            ExprKind::Unary(ref op, ref mut operand) => {
                self.visit_expr(operand, ctx);
                expr.ty = Some(self.check_unary(op, operand));
            }
            ExprKind::Call(ref mut callee, ref mut args, ref generics) => {
                expr.ty = Some(self.check_call(callee, args, generics, ctx));
            }
            ExprKind::StructInit(ref mut path, ref mut fields, ref generics) => {
                expr.ty = Some(self.check_struct_init(path, fields, generics, ctx));
            }
            ExprKind::Member(ref mut object, ref field_name) => {
                self.visit_expr(object, ctx);
                expr.ty = Some(self.check_member_access(object, field_name, ctx));
            }
            ExprKind::Index(ref mut arr, ref mut idx) => {
                self.visit_expr(arr, ctx);
                self.visit_expr(idx, ctx);
                expr.ty = Some(self.check_index(arr, idx));
            }
            ExprKind::Cast(ref mut inner, ref target_ty) => {
                self.visit_expr(inner, ctx);
                // TODO: Add validation if cast is allowed
                expr.ty = Some(target_ty.clone());
            }
            ExprKind::Deref(ref mut inner) => {
                self.visit_expr(inner, ctx);
                if let Some(Type::Pointer(inner_ty)) = &inner.ty {
                    expr.ty = Some(*inner_ty.clone());
                } else {
                    panic!("Cannot dereference non-pointer type: {:?}", inner.ty);
                }
            }
            ExprKind::AddrOf(ref mut inner) => {
                self.visit_expr(inner, ctx);
                if let Some(ty) = &inner.ty {
                    expr.ty = Some(Type::Pointer(Box::new(ty.clone())));
                } else {
                    panic!("Cannot take address of expression with unknown type");
                }
            }
            _ => todo!("ExprKind not implemented in TypeCheckPass"),
        }
    }

    fn infer_lit_type(&mut self, lit: &mut Lit, ctx: &mut TypeContext) -> Type {
        match lit {
            Lit::Int(_) => Type::I32,
            Lit::Float(_) => Type::F64,
            Lit::Bool(_) => Type::Bool,
            Lit::Str(s) => Type::Array(Box::new(Type::U8), s.len() - 2),
            Lit::Null => Type::Pointer(Box::new(Type::Void)),
            Lit::Array(elements) => {
                if elements.is_empty() {
                    panic!("Empty array literals need explicit type hint (not implemented yet)");
                }

                for item in elements.iter_mut() {
                    self.visit_expr(item, ctx);
                }

                let first_ty = elements[0].ty.clone().unwrap();
                for elem in elements.iter().skip(1) {
                    let ty = elem.ty.clone().unwrap();
                    if ty != first_ty {
                        panic!("Array literal must have homogeneous types");
                    }
                }
                Type::Array(Box::new(first_ty), elements.len())
            }
        }
    }

    fn check_binary(&self, left: &Expr, op: &BinOp, right: &Expr) -> Type {
        let left_ty = left.ty.as_ref().unwrap();
        let right_ty = right.ty.as_ref().unwrap();

        if left_ty != right_ty {
            panic!(
                "Binary operation types mismatch: {:?} vs {:?}",
                left_ty, right_ty
            );
        }

        match op {
            BinOp::Add
            | BinOp::Sub
            | BinOp::Mul
            | BinOp::Div
            | BinOp::Mod
            | BinOp::BitAnd
            | BinOp::BitOr
            | BinOp::BitXor
            | BinOp::Shl
            | BinOp::Shr => left_ty.clone(),
            BinOp::Eq
            | BinOp::Neq
            | BinOp::Lt
            | BinOp::Lte
            | BinOp::Gt
            | BinOp::Gte
            | BinOp::And
            | BinOp::Or => Type::Bool,
            _ => panic!("Binary op {:?} not supported", op),
        }
    }

    fn check_unary(&self, op: &UnaryOp, operand: &Expr) -> Type {
        let ty = operand.ty.as_ref().unwrap();
        match op {
            UnaryOp::Neg => {
                if matches!(ty, Type::I32 | Type::I64 | Type::F32 | Type::F64) {
                    ty.clone()
                } else {
                    panic!("Cannot negate type {:?}", ty);
                }
            }
            UnaryOp::Not => {
                if matches!(
                    ty,
                    Type::Bool | Type::I32 | Type::I64 | Type::U32 | Type::U64
                ) {
                    ty.clone()
                } else {
                    panic!("Cannot apply NOT to type {:?}", ty);
                }
            }
            _ => panic!("Unary op {:?} not supported", op),
        }
    }
    fn check_call(
        &mut self,
        callee: &mut Expr,
        args: &mut Vec<Expr>,
        explicit_generics: &Vec<Type>,
        ctx: &mut TypeContext,
    ) -> Type {
        self.visit_expr(callee, ctx);
        let callee_ty = callee.ty.as_ref().expect("Could not resolve callee type");

        let callee_name = match &callee.kind {
            ExprKind::Ident(path) => Some(self.path_to_string(path)),
            _ => None,
        };

        let is_variadic = if let Some(name) = &callee_name {
            if let Some(func) = ctx.concrete_funcs.get(name) {
                func.is_variadic
            } else if let Some(func) = ctx.generic_func_templates.get(name) {
                func.is_variadic
            } else {
                false
            }
        } else {
            false
        };

        if let Type::Function(param_types, ret_type, generic_params_decl) = callee_ty {
            if is_variadic {
                if args.len() < param_types.len() {
                    panic!(
                        "Variadic function expects at least {} arguments, got {}",
                        param_types.len(),
                        args.len()
                    );
                }
            } else {
                if args.len() != param_types.len() {
                    panic!(
                        "Argument count mismatch: expected {}, got {}",
                        param_types.len(),
                        args.len()
                    );
                }
            }

            for arg in args.iter_mut() {
                self.visit_expr(arg, ctx);
            }

            let mut generic_map = std::collections::HashMap::new();

            if !explicit_generics.is_empty() {
                if explicit_generics.len() != generic_params_decl.len() {
                    panic!("Generic count mismatch in call");
                }
                for (name_ty, concrete_ty) in
                    generic_params_decl.iter().zip(explicit_generics.iter())
                {
                    if let Type::Generic(name) = name_ty {
                        generic_map.insert(name.clone(), concrete_ty.clone());
                    }
                }
            } else if !generic_params_decl.is_empty() {
                for (param_ty, arg) in param_types.iter().zip(args.iter()) {
                    let arg_ty = arg.ty.as_ref().unwrap();
                    self.unify_types(param_ty, arg_ty, &mut generic_map);
                }

                for gen_decl in generic_params_decl {
                    if let Type::Generic(name) = gen_decl {
                        if !generic_map.contains_key(name) {
                            panic!("Could not infer generic type '{}'", name);
                        }
                    }
                }
            }

            for (i, (param_ty, arg)) in param_types.iter().zip(args.iter()).enumerate() {
                let concrete_param_ty = self.substitute_generics_mut(param_ty, &generic_map, ctx);
                let arg_ty = arg.ty.as_ref().unwrap();
                if !self.check_type_compatibility(&concrete_param_ty, arg_ty) {
                    panic!(
                        "Type mismatch at arg {}: expected {:?}, found {:?} (Coercion failed)",
                        i, concrete_param_ty, arg_ty
                    );
                }
            }

            if !generic_params_decl.is_empty() {
                if let Some(name) = callee_name {
                    let mut concrete_types = Vec::new();
                    for gen_decl in generic_params_decl {
                        if let Type::Generic(n) = gen_decl {
                            concrete_types.push(generic_map.get(n).unwrap().clone());
                        }
                    }
                    ctx.register_generic_func_request(name, concrete_types);
                }
            }

            self.substitute_generics_mut(ret_type, &generic_map, ctx)
        } else {
            panic!("Attempted to call a non-function type");
        }
    }

    fn check_struct_init(
        &mut self,
        path: &mut Vec<String>,
        fields: &mut Vec<(String, Expr)>,
        generics: &Vec<Type>,
        ctx: &mut TypeContext,
    ) -> Type {
        let struct_name = path.last().unwrap().clone();

        let (struct_generics_decl, raw_fields_decl) =
            if let Some(def) = ctx.concrete_structs.get(&struct_name) {
                (def.generics.clone(), def.fields.clone())
            } else if let Some(def) = ctx.generic_struct_templates.get(&struct_name) {
                (def.generics.clone(), def.fields.clone())
            } else {
                panic!("Struct definition not found: {}", struct_name);
            };

        let struct_fields_decl: Vec<(String, Type)> = raw_fields_decl
            .iter()
            .map(|(name, ty)| {
                (
                    name.clone(),
                    self.resolve_generic_references(ty, &struct_generics_decl),
                )
            })
            .collect();

        let mut type_map = std::collections::HashMap::new();
        let mut concrete_generics = generics.clone();

        for (_, expr) in fields.iter_mut() {
            self.visit_expr(expr, ctx);
        }

        if !struct_generics_decl.is_empty() {
            if concrete_generics.is_empty() {
                for (field_name, expr) in fields.iter() {
                    if let Some((_, expected_def_ty)) =
                        struct_fields_decl.iter().find(|(n, _)| n == field_name)
                    {
                        let actual_ty = expr.ty.as_ref().unwrap();
                        self.unify_types(expected_def_ty, actual_ty, &mut type_map);
                    }
                }

                for gen_name in &struct_generics_decl {
                    if let Some(ty) = type_map.get(gen_name) {
                        concrete_generics.push(ty.clone());
                    } else {
                        panic!(
                            "Could not infer generic type '{}' for struct '{}'",
                            gen_name, struct_name
                        );
                    }
                }
            } else {
                if concrete_generics.len() != struct_generics_decl.len() {
                    panic!("Generic count mismatch for struct init {}", struct_name);
                }
                for (name, ty) in struct_generics_decl.iter().zip(concrete_generics.iter()) {
                    type_map.insert(name.clone(), ty.clone());
                }
            }

            ctx.register_generic_struct_request(struct_name.clone(), concrete_generics.clone());
        }

        for (field_name, expr) in fields.iter_mut() {
            let expected_base_ty = struct_fields_decl
                .iter()
                .find(|(n, _)| n == field_name)
                .map(|(_, t)| t)
                .expect(&format!(
                    "Field {} does not exist in {}",
                    field_name, struct_name
                ));

            let expected_ty = self.substitute_generics_mut(expected_base_ty, &type_map, ctx);
            let actual_ty = expr.ty.as_ref().unwrap();

            if !self.check_type_compatibility(&expected_ty, actual_ty) {
                panic!(
                    "Field '{}' type mismatch: expected {:?}, found {:?}",
                    field_name, expected_ty, actual_ty
                );
            }
        }

        if !struct_generics_decl.is_empty() {
            let mangled_name = self.instantiate_struct(&struct_name, &concrete_generics, ctx);

            *path = vec![mangled_name.clone()];

            return Type::Struct(vec![mangled_name], vec![]);
        }

        Type::Struct(path.clone(), concrete_generics)
    }

    fn check_member_access(
        &self,
        object: &mut Expr,
        field_name: &str,
        ctx: &mut TypeContext,
    ) -> Type {
        let obj_ty = object.ty.as_ref().expect("Object type inference failed");

        let mut actual_ty = obj_ty;
        if let Type::Pointer(inner) = obj_ty {
            actual_ty = inner;
        }

        if let Type::Struct(path, concrete_generics) = actual_ty {
            let struct_name = path.last().unwrap();

            let (struct_generics_decl, struct_fields_decl) =
                if let Some(def) = ctx.concrete_structs.get(struct_name) {
                    (def.generics.clone(), def.fields.clone())
                } else if let Some(def) = ctx.generic_struct_templates.get(struct_name) {
                    (def.generics.clone(), def.fields.clone())
                } else {
                    panic!("Struct definition not found: {}", struct_name);
                };

            if let Some((_, field_ty)) = struct_fields_decl.iter().find(|(n, _)| n == field_name) {
                let mut field_map = std::collections::HashMap::new();
                for (gen_name, concrete) in
                    struct_generics_decl.iter().zip(concrete_generics.iter())
                {
                    field_map.insert(gen_name.clone(), concrete.clone());
                }
                self.substitute_generics_mut(field_ty, &field_map, ctx)
            } else {
                panic!("Struct {} has no field named {}", struct_name, field_name);
            }
        } else {
            panic!("Cannot access member of non-struct type: {:?}", actual_ty);
        }
    }

    fn check_index(&self, arr: &Expr, idx: &Expr) -> Type {
        let idx_ty = idx.ty.as_ref().unwrap();
        match idx_ty {
            Type::I32 | Type::I64 | Type::Usize | Type::U32 | Type::U64 => {}
            _ => panic!("Array index must be an integer, found {:?}", idx_ty),
        }

        let arr_ty = arr.ty.as_ref().unwrap();
        match arr_ty {
            Type::Array(inner, _) => *inner.clone(),
            Type::Pointer(inner) => *inner.clone(),
            _ => panic!("Cannot index type {:?}", arr_ty),
        }
    }
    fn mangle_name(&self, base: &str, generics: &[Type]) -> String {
        let gen_parts: Vec<String> = generics.iter().map(|t| t.get_name()).collect();
        format!("{}__{}", base, gen_parts.join("_"))
    }
    fn instantiate_struct(
        &self,
        name: &str,
        generics: &Vec<Type>,
        ctx: &mut TypeContext,
    ) -> String {
        let mangled_name = self.mangle_name(name, generics);

        if ctx.concrete_structs.contains_key(&mangled_name) {
            return mangled_name;
        }

        if let Some(template) = ctx.generic_struct_templates.get(name).cloned() {
            let mut map = HashMap::new();
            for (gen_decl, concrete) in template.generics.iter().zip(generics.iter()) {
                map.insert(gen_decl.clone(), concrete.clone());
            }

            let new_fields: Vec<(String, Type)> = template
                .fields
                .iter()
                .map(|(fname, fty)| (fname.clone(), self.substitute_generics_mut(fty, &map, ctx)))
                .collect();

            let mut new_def = template.clone();
            new_def.name = mangled_name.clone();
            new_def.fields = new_fields;
            new_def.generics = vec![];
            ctx.concrete_structs.insert(mangled_name.clone(), new_def);

            ctx.register_generic_struct_request(name.to_string(), generics.clone());
        }

        mangled_name
    }

    fn substitute_generics_mut(
        &self,
        ty: &Type,
        map: &HashMap<String, Type>,
        ctx: &mut TypeContext,
    ) -> Type {
        match ty {
            Type::Generic(name) => {
                if let Some(concrete) = map.get(name) {
                    concrete.clone()
                } else {
                    ty.clone()
                }
            }
            Type::Pointer(inner) => {
                Type::Pointer(Box::new(self.substitute_generics_mut(inner, map, ctx)))
            }
            Type::Array(inner, size) => Type::Array(
                Box::new(self.substitute_generics_mut(inner, map, ctx)),
                *size,
            ),
            Type::Function(params, ret, gens) => {
                let new_params = params
                    .iter()
                    .map(|p| self.substitute_generics_mut(p, map, ctx))
                    .collect();
                let new_ret = Box::new(self.substitute_generics_mut(ret, map, ctx));
                Type::Function(new_params, new_ret, gens.clone())
            }
            Type::Struct(path, generics) => {
                if path.len() == 1 && generics.is_empty() {
                    if let Some(concrete) = map.get(&path[0]) {
                        return concrete.clone();
                    }
                }

                let new_generics: Vec<Type> = generics
                    .iter()
                    .map(|g| self.substitute_generics_mut(g, map, ctx))
                    .collect();

                let struct_name = path.last().unwrap();

                if ctx.generic_struct_templates.contains_key(struct_name)
                    && !new_generics.is_empty()
                {
                    let mangled_name = self.instantiate_struct(struct_name, &new_generics, ctx);

                    return Type::Struct(vec![mangled_name], vec![]);
                }
                Type::Struct(path.clone(), new_generics)
            }
            _ => ty.clone(),
        }
    }

    fn unify_types(&self, param_ty: &Type, arg_ty: &Type, map: &mut HashMap<String, Type>) {
        match (param_ty, arg_ty) {
            (Type::Generic(name), _) => {
                if let Some(existing) = map.get(name) {
                    if existing != arg_ty {
                        panic!(
                            "Generic type conflict for '{}': {:?} vs {:?}",
                            name, existing, arg_ty
                        );
                    }
                } else {
                    map.insert(name.clone(), arg_ty.clone());
                }
            }
            (Type::Pointer(p_inner), Type::Pointer(a_inner)) => {
                self.unify_types(p_inner, a_inner, map);
            }
            (Type::Array(p_inner, _), Type::Array(a_inner, _)) => {
                self.unify_types(p_inner, a_inner, map);
            }
            (Type::Struct(_, p_gens), Type::Struct(_, a_gens)) => {
                if p_gens.len() == a_gens.len() {
                    for (p, a) in p_gens.iter().zip(a_gens.iter()) {
                        self.unify_types(p, a, map);
                    }
                }
            }
            _ => {}
        }
    }

    fn path_to_string(&self, path: &Vec<String>) -> String {
        path.join("::")
    }

    fn resolve_generic_references(&self, ty: &Type, generic_params: &[String]) -> Type {
        match ty {
            Type::Struct(path, generics) => {
                if path.len() == 1 && generics.is_empty() && generic_params.contains(&path[0]) {
                    return Type::Generic(path[0].clone());
                }

                let new_generics = generics
                    .iter()
                    .map(|t| self.resolve_generic_references(t, generic_params))
                    .collect();
                Type::Struct(path.clone(), new_generics)
            }

            Type::Pointer(inner) => Type::Pointer(Box::new(
                self.resolve_generic_references(inner, generic_params),
            )),

            Type::Array(inner, size) => Type::Array(
                Box::new(self.resolve_generic_references(inner, generic_params)),
                *size,
            ),

            Type::Function(params, ret, gens) => {
                let new_params = params
                    .iter()
                    .map(|p| self.resolve_generic_references(p, generic_params))
                    .collect();
                let new_ret = Box::new(self.resolve_generic_references(ret, generic_params));
                Type::Function(new_params, new_ret, gens.clone())
            }

            _ => ty.clone(),
        }
    }

    fn check_type_compatibility(&self, expected: &Type, actual: &Type) -> bool {
        if expected == actual {
            return true;
        }

        match (expected, actual) {
            (Type::Const(inner_expected), _) => {
                self.check_type_compatibility(inner_expected, actual)
            }

            (Type::Pointer(inner_expected), Type::Array(inner_actual, _)) => {
                self.check_type_compatibility(inner_expected, inner_actual)
            }

            (Type::Char, Type::U8) => true,
            (Type::U8, Type::Char) => true,

            (Type::Pointer(inner), Type::Pointer(_)) => {
                if let Type::Void = **inner {
                    true
                } else {
                    false
                }
            }

            _ => false,
        }
    }
}

impl Pass for TypeCheckPass {
    fn name(&self) -> &str {
        "TypeCheckPass"
    }

    fn run(&mut self, ctx: &mut TypeContext) {
        let func_names: Vec<String> = ctx.concrete_funcs.keys().cloned().collect();

        for name in func_names {
            if let Some(mut func) = ctx.concrete_funcs.remove(&name) {
                self.visit_function_def(&mut func, ctx);
                ctx.concrete_funcs.insert(name, func);
            }
        }
    }
}
