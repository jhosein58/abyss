use std::collections::HashMap;

use abyss_types::{TypeKind, TypeStore};

use crate::nexus::TypeId;

pub enum TypeKey {
    Unknown,

    Int(u16), // integer type with bit width
    UInt(u16),
    Float(u16),

    Bool,
}

#[derive(Default)]
pub struct TypeStorage {
    store: TypeStore,
    interned: HashMap<TypeKey, TypeId>,
}

impl TypeStorage {
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.store.len()
    }

    #[inline(always)]
    pub fn reserve(&mut self, additional: usize) {
        self.store.reserve(additional);
    }

    #[inline(always)]
    pub fn kind(&self, idx: TypeId) -> TypeKind {
        self.store.kinds[idx.0 as usize]
    }
}
