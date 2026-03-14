use std::collections::HashMap;

use crate::types::Type;

#[derive(Debug)]
pub struct TypeEncoder {
    type_to_id: HashMap<Type, i64>,
    id_to_type: HashMap<i64, Type>,
    next_dynamic_id: i64,
}

impl Default for TypeEncoder {
    fn default() -> Self {
        Self::new()
    }
}

impl TypeEncoder {
    pub fn new() -> Self {
        Self {
            type_to_id: HashMap::new(),
            id_to_type: HashMap::new(),
            next_dynamic_id: 100,
        }
    }

    pub fn get_or_create_id(&mut self, ty: &Type) -> i64 {
        let normalized_ty = ty.normalize();

        if let Some(static_id) = normalized_ty.get_static_id() {
            return static_id;
        }

        if let Some(&id) = self.type_to_id.get(&normalized_ty) {
            return id;
        }

        let id = self.next_dynamic_id;
        self.next_dynamic_id += 1;

        self.type_to_id.insert(normalized_ty.clone(), id);
        self.id_to_type.insert(id, normalized_ty);

        id
    }

    pub fn from_id(&self, id: i64) -> Type {
        if id < 100 {
            Type::from_static_id(id)
        } else {
            self.id_to_type.get(&id).cloned().unwrap_or(Type::Error)
        }
    }
}
