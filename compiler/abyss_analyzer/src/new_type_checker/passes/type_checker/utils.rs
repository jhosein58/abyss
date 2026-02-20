use abyss_parser::ast::Type;
use std::collections::HashMap;

use crate::new_type_checker::context::TypeContext;

pub struct GenericsEngine;

impl GenericsEngine {
    pub fn mangle_name(base: &str, generics: &[Type]) -> String {
        let gen_parts: Vec<String> = generics.iter().map(|t| t.get_name()).collect();
        format!("{}__{}", base, gen_parts.join("_"))
    }

    pub fn instantiate_struct(name: &str, generics: &Vec<Type>, ctx: &mut TypeContext) -> String {
        let mangled_name = Self::mangle_name(name, generics);

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
                .map(|(fname, fty)| (fname.clone(), Self::substitute_generics_mut(fty, &map, ctx)))
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
    pub fn resolve_generic_references(ty: &Type, generic_params: &[String]) -> Type {
        match ty {
            Type::Struct(path, generics) => {
                if path.len() == 1 && generics.is_empty() && generic_params.contains(&path[0]) {
                    return Type::Generic(path[0].clone());
                }

                let new_generics = generics
                    .iter()
                    .map(|t| Self::resolve_generic_references(t, generic_params))
                    .collect();
                Type::Struct(path.clone(), new_generics)
            }

            Type::Pointer(inner) => Type::Pointer(Box::new(Self::resolve_generic_references(
                inner,
                generic_params,
            ))),

            Type::Array(inner, size) => Type::Array(
                Box::new(Self::resolve_generic_references(inner, generic_params)),
                *size,
            ),

            Type::Function(params, ret, gens) => {
                let new_params = params
                    .iter()
                    .map(|p| Self::resolve_generic_references(p, generic_params))
                    .collect();
                let new_ret = Box::new(Self::resolve_generic_references(ret, generic_params));
                Type::Function(new_params, new_ret, gens.clone())
            }

            _ => ty.clone(),
        }
    }

    pub fn substitute_generics_mut(
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
                Type::Pointer(Box::new(Self::substitute_generics_mut(inner, map, ctx)))
            }
            Type::Array(inner, size) => Type::Array(
                Box::new(Self::substitute_generics_mut(inner, map, ctx)),
                *size,
            ),
            Type::Function(params, ret, gens) => {
                let new_params = params
                    .iter()
                    .map(|p| Self::substitute_generics_mut(p, map, ctx))
                    .collect();
                let new_ret = Box::new(Self::substitute_generics_mut(ret, map, ctx));
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
                    .map(|g| Self::substitute_generics_mut(g, map, ctx))
                    .collect();

                let struct_name = path.last().unwrap();

                if ctx.generic_struct_templates.contains_key(struct_name)
                    && !new_generics.is_empty()
                {
                    let mangled_name = Self::instantiate_struct(struct_name, &new_generics, ctx);

                    return Type::Struct(vec![mangled_name], vec![]);
                }
                Type::Struct(path.clone(), new_generics)
            }
            _ => ty.clone(),
        }
    }

    pub fn unify_types(&self, param_ty: &Type, arg_ty: &Type, map: &mut HashMap<String, Type>) {
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
}
