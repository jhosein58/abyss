use abyss_parser::ast::{BinaryOp, Expr, Lit, Type};

use crate::type_checker::TypeChecker;

impl TypeChecker {
    pub fn get_type_tag(&mut self, ty: &Type) -> i64 {
        let (name, id) = match ty {
            Type::U8 => ("TYPE_TAG_U8".to_string(), 1),
            Type::U16 => ("TYPE_TAG_U16".to_string(), 2),
            Type::U32 => ("TYPE_TAG_U32".to_string(), 3),
            Type::U64 => ("TYPE_TAG_U64".to_string(), 4),
            Type::Usize => ("TYPE_TAG_USIZE".to_string(), 5),
            Type::I8 => ("TYPE_TAG_I8".to_string(), 6),
            Type::I16 => ("TYPE_TAG_I16".to_string(), 7),
            Type::I32 => ("TYPE_TAG_I32".to_string(), 8),
            Type::I64 => ("TYPE_TAG_I64".to_string(), 9),
            Type::Isize => ("TYPE_TAG_ISIZE".to_string(), 10),
            Type::F32 => ("TYPE_TAG_F32".to_string(), 11),
            Type::F64 => ("TYPE_TAG_F64".to_string(), 12),
            Type::Bool => ("TYPE_TAG_BOOL".to_string(), 13),
            Type::Char => ("TYPE_TAG_CHAR".to_string(), 14),
            Type::Array(inner, _) if **inner == Type::U8 => ("TYPE_TAG_U8".to_string(), 1),
            _ => {
                let s = format!("{:?}", ty);
                let mut hash: i64 = 0;
                for c in s.bytes() {
                    hash = hash.wrapping_add(c as i64);
                }
                (format!("TYPE_TAG_{}", hash), hash)
            }
        };
        self.ctx.used_type_tags.insert(name, id);
        id
    }

    pub fn wrap_expr_for_union(
        &mut self,
        mut expr: Expr,
        expr_ty: Type,
        variants: &[Type],
        target_struct_name: String,
    ) -> (Expr, Type) {
        if !variants.contains(&expr_ty) {
            for variant in variants {
                let is_int_conv = self.is_integer(variant) && self.is_integer(&expr_ty);
                let is_float_conv = self.is_float(variant) && self.is_float(&expr_ty);

                if is_int_conv || is_float_conv {
                    expr = Expr::Cast(Box::new(expr), variant.clone());
                    break;
                }
            }
        }

        let (_, final_rhs_ty) = self.infer_expr(expr.clone());
        let tag_val = self.get_type_tag(&final_rhs_ty);

        let mut sorted_variants = variants.to_vec();
        sorted_variants.sort_by_key(|t| t.get_name());

        if let Some(variant_index) = sorted_variants.iter().position(|t| t == &final_rhs_ty) {
            let inner_struct_name = target_struct_name.replace("__Union_", "__UnionInner_");

            let inner_init = Expr::UnionInit(
                vec![inner_struct_name.clone()],
                vec![(format!("variant_{}", variant_index), expr)],
            );

            let wrapper_init = Expr::StructInit(
                vec![target_struct_name.clone()],
                vec![
                    ("tag".to_string(), Expr::Lit(Lit::Int(tag_val))),
                    ("data".to_string(), inner_init),
                ],
                vec![],
            );

            let wrapper_type = Type::Struct(vec![target_struct_name], vec![]);
            (wrapper_init, wrapper_type)
        } else {
            panic!(
                "Type mismatch: Cannot assign {:?} to Union Wrapper {:?} (Variants: {:?})",
                final_rhs_ty, target_struct_name, variants
            );
        }
    }

    pub fn materialize_union_type(&mut self, ty: &mut Type) {
        if let Type::Union(variants) = ty {
            let struct_name = self.mono().get_or_create_union_struct(variants);
            *ty = Type::Struct(vec![struct_name], vec![]);
        }
    }

    pub fn try_wrap_rhs_for_union(&mut self, rhs_expr: Expr, rhs_ty: &Type, lhs_ty: &Type) -> Expr {
        if let Type::Struct(names, _) = lhs_ty {
            let struct_name = &names[0];
            if struct_name.starts_with("__Union_") && !struct_name.contains("__UnionInner_") {
                if rhs_ty != lhs_ty {
                    if let Some(variants) = self.ctx.variant_cache.get(struct_name).cloned() {
                        let (wrapped, _) = self.wrap_expr_for_union(
                            rhs_expr,
                            rhs_ty.clone(),
                            &variants,
                            struct_name.clone(),
                        );
                        return wrapped;
                    }
                }
            }
        }

        if let Type::Union(variants) = lhs_ty {
            if variants.contains(rhs_ty) {
                let struct_name = self.mono().get_or_create_union_struct(variants);
                let (wrapped, _) =
                    self.wrap_expr_for_union(rhs_expr, rhs_ty.clone(), variants, struct_name);
                return wrapped;
            }
        }

        rhs_expr
    }
    pub fn resolve_is_expr_with_type(
        &mut self,
        new_inner: Expr,
        inner_ty: Type,
        check_ty: Type,
    ) -> (Expr, Type) {
        if let Type::Struct(path, _) = &check_ty {
            if let Some(alias_name) = path.first() {
                if let Some(aliased_ty) = self.ctx.get_type_alias(alias_name.clone()).cloned() {
                    return self.resolve_is_expr_with_type(new_inner, inner_ty, aliased_ty);
                }
            }
        }

        if let Type::Union(variants) = &inner_ty {
            if !variants.contains(&check_ty) {
                return (Expr::Lit(Lit::Bool(false)), Type::Bool);
            }
            let tag_access = Expr::Member(Box::new(new_inner), "tag".to_string());
            let target_tag = self.get_type_tag(&check_ty);
            let comparison = Expr::Binary(
                Box::new(tag_access),
                BinaryOp::Eq,
                Box::new(Expr::Lit(Lit::Int(target_tag))),
            );
            return (comparison, Type::Bool);
        }

        if let Type::Struct(path, _) = &inner_ty {
            if let Some(struct_name) = path.last() {
                if struct_name.starts_with("__Union_") {
                    let inner_union_name = struct_name.replace("__Union_", "__UnionInner_");

                    let union_contains_type = self
                        .ctx
                        .concrete_unions
                        .iter()
                        .find(|u| u.name == inner_union_name)
                        .map(|u| u.fields.iter().any(|(_, f_ty)| f_ty == &check_ty))
                        .unwrap_or(false);

                    if union_contains_type {
                        let tag_access = Expr::Member(Box::new(new_inner), "tag".to_string());
                        let target_tag = self.get_type_tag(&check_ty);
                        let comparison = Expr::Binary(
                            Box::new(tag_access),
                            BinaryOp::Eq,
                            Box::new(Expr::Lit(Lit::Int(target_tag))),
                        );
                        return (comparison, Type::Bool);
                    } else {
                        return (Expr::Lit(Lit::Bool(false)), Type::Bool);
                    }
                }
            }
        }

        let result = inner_ty == check_ty;
        (Expr::Lit(Lit::Bool(result)), Type::Bool)
    }

    pub(crate) fn resolve_union_cast(
        &mut self,
        new_inner: Expr,
        inner_ty: Type,
        target_ty: Type,
    ) -> Option<(Expr, Type)> {
        if let Type::Union(variants) = &target_ty {
            let is_variant = variants
                .iter()
                .any(|v| self.are_types_compatible(v, &inner_ty));

            if is_variant {
                let struct_name = self.mono().get_or_create_union_struct(variants);

                let (wrapped_expr, wrapped_ty) =
                    self.wrap_expr_for_union(new_inner, inner_ty, variants, struct_name);
                return Some((wrapped_expr, wrapped_ty));
            }
        }

        if let Type::Union(variants) = &inner_ty {
            if variants.contains(&target_ty) {
                let data_access = Expr::Member(Box::new(new_inner), "data".to_string());
                let mut sorted_variants = variants.clone();
                sorted_variants.sort_by_key(|t| t.get_name());

                let variant_index = sorted_variants
                    .iter()
                    .position(|t| t == &target_ty)
                    .expect("Target type not in union");

                return Some((
                    Expr::Member(Box::new(data_access), format!("variant_{}", variant_index)),
                    target_ty.clone(),
                ));
            }
        }

        if let Type::Struct(path, _) = &inner_ty {
            if let Some(struct_name) = path.last() {
                if struct_name.starts_with("__Union_") {
                    let inner_union_name = struct_name.replace("__Union_", "__UnionInner_");

                    if let Some(union_def) = self
                        .ctx
                        .concrete_unions
                        .iter()
                        .find(|u| u.name == inner_union_name)
                    {
                        if let Some((field_name, _)) =
                            union_def.fields.iter().find(|(_, f_ty)| f_ty == &target_ty)
                        {
                            let data_access = Expr::Member(Box::new(new_inner), "data".to_string());

                            return Some((
                                Expr::Member(Box::new(data_access), field_name.clone()),
                                target_ty.clone(),
                            ));
                        }
                    }
                }
            }
        }

        None
    }

    pub fn try_wrap_struct_field(&mut self, f_expr: Expr, f_ty: &Type, expected_ty: &Type) -> Expr {
        if let Type::Struct(names, _) = expected_ty {
            if let Some(inner_name) = names.first() {
                if inner_name.starts_with("__Union_") && !inner_name.contains("__UnionInner_") {
                    if f_ty != expected_ty {
                        if let Some(variants) = self.ctx.variant_cache.get(inner_name).cloned() {
                            let (wrapped, _) = self.wrap_expr_for_union(
                                f_expr,
                                f_ty.clone(),
                                &variants,
                                inner_name.clone(),
                            );
                            return wrapped;
                        }
                    }
                }
            }
        }
        f_expr
    }

    pub fn wrap_let_union_init(
        &mut self,
        expr: Expr,
        expr_ty: &Type,
        variants: &[Type],
    ) -> (Expr, Type) {
        let struct_name = self.mono().get_or_create_union_struct(variants);
        let inner_struct_name = struct_name.replace("__Union_", "__UnionInner_");

        let tag_val = self.get_type_tag(expr_ty);

        let mut sorted_variants = variants.to_vec();
        sorted_variants.sort_by_key(|t| t.get_name());

        let variant_index = sorted_variants.iter().position(|t| t == expr_ty).unwrap();

        let inner_init = Expr::UnionInit(
            vec![inner_struct_name],
            vec![(format!("variant_{}", variant_index), expr)],
        );

        let new_expr = Expr::StructInit(
            vec![struct_name.clone()],
            vec![
                ("tag".to_string(), Expr::Lit(Lit::Int(tag_val))),
                ("data".to_string(), inner_init),
            ],
            vec![],
        );

        let concrete_ty = Type::Struct(vec![struct_name], vec![]);
        (new_expr, concrete_ty)
    }
}
