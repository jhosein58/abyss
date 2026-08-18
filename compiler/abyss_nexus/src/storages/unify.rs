use crate::{
    arena::{Arena, ArenaId, SideTable},
    nexus::{HirId, SlotId, TypeId},
};

#[derive(Default)]
pub struct UnifyStorage {
    pub parents: Arena<SlotId, SlotId>,
    pub ranks: SideTable<SlotId, u8>,
    pub types: SideTable<SlotId, TypeId>,
    pub origins: SideTable<SlotId, HirId>,
}

impl UnifyStorage {
    #[inline(always)]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn grow_to(&mut self, capacity: usize) {
        self.ranks.grow_to(capacity);
        self.types.grow_to(capacity);
        self.origins.grow_to(capacity);
    }

    #[inline]
    pub fn new_slot(&mut self, origin: HirId) -> SlotId {
        let slot_val = self.parents.len() as u32;
        let slot = self.parents.alloc(SlotId::new(slot_val));

        self.types.set(slot, TypeId::none());
        self.origins.set(slot, origin);

        slot
    }

    #[inline]
    pub fn find(&mut self, mut slot: SlotId) -> SlotId {
        loop {
            let parent = self.parents.get_copy(slot);
            if parent == slot {
                return slot;
            }

            let grand = self.parents.get_copy(parent);
            self.parents.set(slot, grand);
            slot = grand;
        }
    }

    pub fn union(&mut self, a: SlotId, b: SlotId) -> Result<SlotId, (TypeId, TypeId)> {
        let root_a = self.find(a);
        let root_b = self.find(b);

        if root_a == root_b {
            return Ok(root_a);
        }

        let type_a = self.types.get_copy(root_a);
        let type_b = self.types.get_copy(root_b);

        let final_type = match (type_a.is_some(), type_b.is_some()) {
            (true, true) => {
                if type_a != type_b {
                    return Err((type_a, type_b));
                }
                type_a
            }
            (true, false) => type_a,
            (false, true) => type_b,
            (false, false) => TypeId::none(),
        };

        let rank_a = self.ranks.get_copy(root_a);
        let rank_b = self.ranks.get_copy(root_b);

        let (new_root, old_root) = if rank_a > rank_b {
            (root_a, root_b)
        } else if rank_a < rank_b {
            (root_b, root_a)
        } else {
            self.ranks.set(root_a, rank_a + 1);
            (root_a, root_b)
        };

        self.parents.set(old_root, new_root);

        if final_type.is_some() {
            self.types.set(new_root, final_type);
        }

        Ok(new_root)
    }

    #[inline]
    pub fn bind_type(&mut self, slot: SlotId, ty: TypeId) -> Result<(), (TypeId, TypeId)> {
        let root = self.find(slot);
        let existing = self.types.get_copy(root);

        if existing.is_some() {
            if existing != ty {
                return Err((existing, ty));
            }
        } else {
            self.types.set(root, ty);
        }
        Ok(())
    }

    #[inline]
    pub fn resolve_type(&mut self, slot: SlotId) -> TypeId {
        let root = self.find(slot);
        self.types.get_copy(root)
    }
}
