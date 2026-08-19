use crate::{
    arena::{ArenaId, SideTable},
    nexus::{HirId, RawId, TypeId},
};

#[derive(Default)]
pub struct ConstStorage {
    pub values: SideTable<HirId, RawId>,
}

impl ConstStorage {
    pub fn grow_to(&mut self, new_len: usize) {
        self.values.grow_to(new_len);
    }

    #[inline(always)]
    pub fn is_none(&self, id: HirId) -> bool {
        self.values.get(id).is_none()
    }

    #[inline(always)]
    pub fn is_some(&self, id: HirId) -> bool {
        self.values.get(id).is_some()
    }

    #[inline(always)]
    pub fn set_type(&mut self, id: HirId, ty: TypeId) {
        self.values.set(id, RawId(ty.0));
    }

    #[inline(always)]
    pub fn get_type(&mut self, id: HirId) -> TypeId {
        TypeId(self.values.get_copy(id).0)
    }
}
