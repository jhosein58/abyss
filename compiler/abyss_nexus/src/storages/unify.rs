use abyss_types::TyKind;

use crate::{
    arena::{Arena, ArenaId, SideTable},
    nexus::{HirId, SlotId, TypeId},
    storages::types::TypeStorage,
};

#[derive(Default)]
pub struct UnifyStorage {
    pub parents: Arena<SlotId, SlotId>,
    pub ranks: SideTable<SlotId, u8>,
    pub types: SideTable<SlotId, TypeId>,
    pub origins: SideTable<SlotId, HirId>,
    pub hir_to_slot: SideTable<HirId, SlotId>,
}

impl UnifyStorage {
    #[inline(always)]
    pub fn new() -> Self {
        Self::default()
    }

    #[inline]
    pub fn grow_to(&mut self, capacity: usize) {
        self.parents.reserve(capacity);
        self.ranks.grow_to(capacity);
        self.types.grow_to(capacity);
        self.origins.grow_to(capacity);
        self.hir_to_slot.grow_to(capacity);
    }

    #[inline]
    pub fn new_slot(&mut self, origin: HirId) -> SlotId {
        let slot_val = self.parents.len() as u32;
        let slot = self.parents.alloc(SlotId::new(slot_val));

        self.types.set_safe(slot, TypeId::none());
        self.origins.set_safe(slot, origin);
        self.hir_to_slot.set_safe(origin, slot);

        slot
    }

    #[inline]
    pub fn tmp_slot(&mut self) -> SlotId {
        let slot_val = self.parents.len() as u32;
        let slot = self.parents.alloc(SlotId::new(slot_val));

        self.types.set_safe(slot, TypeId::none());

        slot
    }

    #[inline]
    pub fn get_slot(&mut self, id: HirId) -> SlotId {
        self.hir_to_slot.get_copy(id)
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

    #[inline(always)]
    pub fn unify_types(
        &mut self,
        types: &mut TypeStorage,
        a: TypeId,
        b: TypeId,
    ) -> Result<TypeId, (TypeId, TypeId)> {
        if a == b {
            return Ok(a);
        }

        let kind_a = types.kind(a);
        let kind_b = types.kind(b);

        if kind_a == TyKind::Infer && kind_b == TyKind::Infer {
            let slot_a = types.infer_slot(a);
            let slot_b = types.infer_slot(b);
            let new_slot = self.union(types, slot_a, slot_b)?;

            return Ok(types.alloc_infer(new_slot));
        } else if kind_a == TyKind::Infer {
        }

        if kind_a == TyKind::Never {
            return Ok(b);
        }
        if kind_b == TyKind::Never {
            return Ok(a);
        }

        if kind_a == TyKind::Unknown {
            return Ok(b);
        }
        if kind_b == TyKind::Unknown {
            return Ok(a);
        }

        match (kind_a, kind_b) {
            (
                TyKind::UntypedInt,
                TyKind::Int | TyKind::UInt | TyKind::Float | TyKind::UntypedFloat,
            ) => Ok(b),
            (
                TyKind::Int | TyKind::UInt | TyKind::Float | TyKind::UntypedFloat,
                TyKind::UntypedInt,
            ) => Ok(a),

            (TyKind::Array, TyKind::Array) => {
                let len_a = types.get_array_len(a);
                let len_b = types.get_array_len(b);

                if len_a != len_b {
                    return Err((a, b));
                }

                let inner_a = types.get_array_type(a);
                let inner_b = types.get_array_type(b);

                let unified_inner = self.unify_types(types, inner_a, inner_b)?;
                Ok(types.alloc_array(unified_inner, len_a))
            }

            (TyKind::UntypedFloat, TyKind::Float) => Ok(b),
            (TyKind::Float, TyKind::UntypedFloat) => Ok(a),

            (TyKind::Ptr, TyKind::Ptr) => {
                let inner_a = TypeId(types.payload(a));
                let inner_b = TypeId(types.payload(b));
                let unified_inner = self.unify_types(types, inner_a, inner_b)?;
                Ok(types.alloc_ptr(unified_inner))
            }

            // TODO: Func type
            _ => Err((a, b)),
        }
    }

    pub fn union(
        &mut self,
        types: &mut TypeStorage,
        a: SlotId,
        b: SlotId,
    ) -> Result<SlotId, (TypeId, TypeId)> {
        let root_a = self.find(a);
        let root_b = self.find(b);

        if root_a == root_b {
            return Ok(root_a);
        }

        let type_a = self.types.get_copy(root_a);
        let type_b = self.types.get_copy(root_b);

        // jolo giri az vrood 'Never' be graph
        if type_a == TypeId::NEVER || type_b == TypeId::NEVER {
            return Ok(root_a);
        }

        let final_type = match (type_a.is_some(), type_b.is_some()) {
            (true, true) => self.unify_types(types, type_a, type_b)?,
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
            self.ranks.set_safe(root_a, rank_a + 1);
            (root_a, root_b)
        };

        self.parents.set(old_root, new_root);

        if final_type.is_some() {
            self.types.set_safe(new_root, final_type);
        }

        Ok(new_root)
    }

    #[inline]
    pub fn bind_type(
        &mut self,
        types: &mut TypeStorage,
        slot: SlotId,
        ty: TypeId,
    ) -> Result<(), (TypeId, TypeId)> {
        let root = self.find(slot);
        let existing = self.types.get_copy(root);

        if existing.is_some() {
            let unified_type = self.unify_types(types, existing, ty)?;
            self.types.set_safe(root, unified_type);
        } else {
            self.types.set_safe(root, ty);
        }
        Ok(())
    }

    #[inline]
    pub fn resolve_type(&mut self, slot: SlotId) -> TypeId {
        let root = self.find(slot);
        self.types.get_copy(root)
    }

    #[inline]
    pub fn resolve_type_deep(&mut self, types: &mut TypeStorage, slot: SlotId) -> TypeId {
        let tyid = self.resolve_type(slot);

        let kind = types.kind(tyid);

        match kind {
            TyKind::Infer => {
                let inner_slot = SlotId(types.payload(tyid));
                self.resolve_type_deep(types, inner_slot)
            }

            TyKind::Array => {
                let len = types.get_array_len(tyid);
                let inner_ty = types.get_array_type(tyid);

                if types.kind(inner_ty) == TyKind::Infer {
                    let infer_slot = SlotId(types.payload(inner_ty));

                    let ty = self.resolve_type_deep(types, infer_slot);

                    types.alloc_array(ty, len)
                } else {
                    inner_ty
                }
            }
            _ => tyid,
        }
    }
}
