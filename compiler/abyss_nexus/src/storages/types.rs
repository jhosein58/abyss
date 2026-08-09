use abyss_types::{TypeKind, TypeStore};

use crate::nexus::TypeId;

#[derive(Default)]
pub struct TypeStorage {
    store: TypeStore,
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
