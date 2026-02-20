use std::collections::HashMap;

use abyss_parser::ast::{Expr, FunctionDef, Type};

use crate::type_checker::TypeChecker;

impl TypeChecker {
    pub fn handle_function_call(
        &mut self,
        callee: Expr,
        args: Vec<Expr>,
        explicit_generics: Vec<Type>,
    ) -> (Expr, Type) {
        let func_name = match &callee {
            Expr::Ident(path) => path.join("__"),
            _ => panic!("Complex callee not supported yet"),
        };

        let mut typed_args = Vec::new();
        let mut arg_types = Vec::new();
        for arg in args {
            let (new_arg, ty) = self.infer_expr(arg);
            typed_args.push(new_arg);
            arg_types.push(ty);
        }

        if let Some(func) = self.ctx.get_local_func(&func_name) {
            if func.params.len() != typed_args.len() {
                panic!(
                    "Argument count mismatch for local function '{}': expected {}, found {}",
                    func_name,
                    func.params.len(),
                    typed_args.len()
                );
            }

            for (i, ((_, param_ty), arg_ty)) in func.params.iter().zip(arg_types.iter()).enumerate()
            {
                if param_ty != arg_ty {
                    panic!(
                        "Type mismatch for argument {} in local function '{}': expected {:?}, found {:?}",
                        i + 1,
                        func_name,
                        param_ty,
                        arg_ty
                    );
                }
            }

            return (
                Expr::Call(Box::new(callee), typed_args, explicit_generics),
                func.return_type.clone(),
            );
        }

        if let Some(template) = self.ctx.generic_func_templates.get(&func_name).cloned() {
            let mut final_generics: Vec<Type>;
            if !explicit_generics.is_empty() {
                if explicit_generics.len() != template.generics.len() {
                    panic!("Generic count mismatch for function '{}'", func_name);
                }
                final_generics = explicit_generics;
            } else {
                final_generics = self.mono().infer_generics_from_args(
                    &template.generics,
                    &template.params,
                    &arg_types,
                );
            }

            let empty_map = HashMap::new();
            for g in &mut final_generics {
                self.mono().substitute_type(g, &empty_map);
            }

            let generics_key = format!("{:?}", final_generics);
            let cache_key = (func_name.clone(), generics_key);

            let mangled_name = if let Some(name) = self.ctx.monomorphization_cache.get(&cache_key) {
                name.clone()
            } else {
                let new_name = format!("{}_{}", func_name, self.ctx.monomorphization_cache.len());
                let mut new_func = template.clone();
                new_func.name = new_name.clone();
                new_func.generics.clear();

                self.ctx
                    .monomorphization_cache
                    .insert(cache_key, new_name.clone());

                self.mono().replace_generics_in_func(
                    &mut new_func,
                    &template.generics,
                    &final_generics,
                );
                self.ctx.pending_funcs.push_back(new_func);

                new_name
            };

            let mut ret_ty = template.return_type.clone();
            let mut map = HashMap::new();
            for (name, ty) in template.generics.iter().zip(final_generics.iter()) {
                map.insert(name.clone(), ty.clone());
            }
            self.mono().substitute_type(&mut ret_ty, &map);

            return (
                Expr::Call(
                    Box::new(Expr::Ident(vec![mangled_name])),
                    typed_args,
                    vec![],
                ),
                ret_ty,
            );
        }

        if let Some(func) = self.ctx.concrete_funcs.iter().find(|f| f.name == func_name) {
            return (
                Expr::Call(Box::new(callee), typed_args, explicit_generics),
                func.return_type.clone(),
            );
        }

        if let Some(func) = self.ctx.pending_funcs.iter().find(|f| f.name == func_name) {
            return (
                Expr::Call(Box::new(callee), typed_args, explicit_generics),
                func.return_type.clone(),
            );
        }

        panic!(
            "Undefined function: '{}'. Did you mean to use a full path (e.g. std::str::new)?",
            func_name
        );
    }
    pub fn resolve_method_call(
        &mut self,
        receiver: Box<Expr>,
        method_name: String,
        args: Vec<Expr>,
        generics: Vec<Type>,
    ) -> (Expr, Type) {
        let static_struct_name = if let Expr::Ident(ref path) = *receiver {
            let potential_struct_name = path.join("__");
            let var_name = path.last().unwrap();

            if self.ctx.get_var_type(var_name).is_none() {
                if self
                    .ctx
                    .generic_struct_templates
                    .contains_key(&potential_struct_name)
                    || self
                        .ctx
                        .concrete_structs
                        .iter()
                        .any(|s| s.name == potential_struct_name)
                    || self
                        .ctx
                        .reverse_struct_map
                        .contains_key(&potential_struct_name)
                {
                    Some(potential_struct_name)
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        };

        if let Some(struct_name) = static_struct_name {
            let func_mangled_name = format!("{}__{}", struct_name, method_name);
            return self.handle_function_call(Expr::Ident(vec![func_mangled_name]), args, generics);
        }

        let (new_receiver, mut receiver_ty) = self.infer_expr(*receiver);
        let mut base_receiver_expr = new_receiver;

        while let Type::Pointer(sub) = receiver_ty.clone() {
            receiver_ty = *sub;
            base_receiver_expr = Expr::Deref(Box::new(base_receiver_expr));
        }

        if let Type::Struct(path, struct_generics) = &receiver_ty {
            let current_struct_name = path.last().unwrap();

            let (base_struct_name, base_generics) = if let Some((base, stored_generics)) =
                self.ctx.reverse_struct_map.get(current_struct_name)
            {
                (base.clone(), stored_generics.clone())
            } else {
                (current_struct_name.clone(), struct_generics.clone())
            };

            let func_mangled_name = format!("{}__{}", base_struct_name, method_name);

            let mut combined_generics = Vec::new();
            combined_generics.extend(base_generics);
            combined_generics.extend(generics);

            let mut final_args = Vec::new();
            let mut final_receiver = base_receiver_expr;

            enum SelfPassingMode {
                ByValue,
                ByRef,
                None,
            }

            let mut passing_mode = SelfPassingMode::None;

            let check_self_param = |func_def: &FunctionDef| -> SelfPassingMode {
                if let Some((_, first_param_ty)) = func_def.params.first() {
                    let (inner_ty, is_pointer) = match first_param_ty {
                        Type::Pointer(inner) => (inner.as_ref(), true),
                        t => (t, false),
                    };

                    if let Type::Struct(path, _) = inner_ty {
                        if let Some(struct_name) = path.last() {
                            if struct_name == &base_struct_name {
                                return if is_pointer {
                                    SelfPassingMode::ByRef
                                } else {
                                    SelfPassingMode::ByValue
                                };
                            }
                        }
                    }
                }
                SelfPassingMode::None
            };

            if let Some(template) = self.ctx.generic_func_templates.get(&func_mangled_name) {
                passing_mode = check_self_param(template);
            } else if let Some(func) = self
                .ctx
                .concrete_funcs
                .iter()
                .find(|f| f.name == func_mangled_name)
            {
                passing_mode = check_self_param(func);
            } else if let Some(func) = self
                .ctx
                .pending_funcs
                .iter()
                .find(|f| f.name == func_mangled_name)
            {
                passing_mode = check_self_param(func);
            }

            match passing_mode {
                SelfPassingMode::ByRef => {
                    final_receiver = Expr::AddrOf(Box::new(final_receiver));
                    final_args.push(final_receiver);
                }
                SelfPassingMode::ByValue => {
                    final_args.push(final_receiver);
                }
                SelfPassingMode::None => {}
            }

            final_args.extend(args);

            return self.handle_function_call(
                Expr::Ident(vec![func_mangled_name]),
                final_args,
                combined_generics,
            );
        } else {
            panic!(
                "Method call '{}' on non-struct type {:?}",
                method_name, receiver_ty
            );
        }
    }
}
