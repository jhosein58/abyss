use abyss_types::types::{StructField, Type};

#[derive(Debug, Clone)]
pub struct FieldMapping {
    pub field_name: String,
    pub duck_index: usize,
    pub concrete_index: usize,
}

#[derive(Debug, Clone)]
pub struct StructuralMatch {
    pub field_mappings: Vec<FieldMapping>,
}

pub fn is_duck_type(ty: &Type) -> bool {
    match ty {
        Type::Struct(_) => true,
        Type::Ptr(inner) => is_duck_type(inner),
        _ => false,
    }
}

pub fn ptr_depth(ty: &Type) -> usize {
    match ty {
        Type::Ptr(inner) => 1 + ptr_depth(inner),
        _ => 0,
    }
}

pub fn peel_all_ptrs(ty: &Type) -> &Type {
    match ty {
        Type::Ptr(inner) => peel_all_ptrs(inner),
        other => other,
    }
}

pub fn check_structural_compat(concrete_ty: &Type, duck_ty: &Type) -> Option<StructuralMatch> {
    if ptr_depth(concrete_ty) != ptr_depth(duck_ty) {
        return None;
    }

    let concrete_inner = peel_all_ptrs(concrete_ty);
    let duck_inner = peel_all_ptrs(duck_ty);

    let concrete_fields = extract_struct_fields(concrete_inner)?;
    let duck_fields = extract_struct_fields(duck_inner)?;

    let mut mappings = Vec::with_capacity(duck_fields.len());

    for (duck_idx, duck_field) in duck_fields.iter().enumerate() {
        let concrete_idx = concrete_fields.iter().position(|cf| {
            cf.name == duck_field.name && field_types_compatible(&cf.ty, &duck_field.ty)
        });

        match concrete_idx {
            Some(idx) => mappings.push(FieldMapping {
                field_name: duck_field.name.clone(),
                duck_index: duck_idx,
                concrete_index: idx,
            }),
            None => return None,
        }
    }

    Some(StructuralMatch {
        field_mappings: mappings,
    })
}

fn extract_struct_fields(ty: &Type) -> Option<&Vec<StructField>> {
    match ty {
        Type::Struct(fields) => Some(fields),
        Type::Alias(_, inner) => extract_struct_fields(inner),
        _ => None,
    }
}

fn field_types_compatible(concrete: &Type, duck: &Type) -> bool {
    let c = concrete.underlying_type();
    let d = duck.underlying_type();
    c == d
}
