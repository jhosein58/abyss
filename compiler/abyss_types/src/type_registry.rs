use std::collections::HashMap;

use crate::types::Type;

pub struct TypeRegistry {
    resolved_types: HashMap<String, Type>,
}

impl TypeRegistry {
    pub fn new() -> Self {
        Self {
            resolved_types: HashMap::new(),
        }
    }

    pub fn register(&mut self, name: String, ty: Type) {
        self.resolved_types.insert(name, ty);
    }

    pub fn get(&self, name: &str) -> Option<&Type> {
        self.resolved_types.get(name)
    }

    pub fn contains(&self, name: &str) -> bool {
        self.resolved_types.contains_key(name)
    }

    pub fn drain_all(&mut self) -> HashMap<String, Type> {
        std::mem::take(&mut self.resolved_types)
    }

    pub fn len(&self) -> usize {
        self.resolved_types.len()
    }

    pub fn is_empty(&self) -> bool {
        self.resolved_types.is_empty()
    }

    pub fn register_name(&mut self, name: String, ty: Type) {
        self.resolved_types.insert(name, ty);
    }

    pub fn get_by_name(&self, name: &str) -> Option<&Type> {
        self.resolved_types.get(name)
    }
}
