use abyss_nexus::nexus::{HirId, Nexus, TypeId};
use abyss_types::TyKind;

#[inline(always)]
pub fn synth_not(db: &mut Nexus, id: HirId) {
    let slot = db.unify.new_slot(id);

    db.unify
        .bind_type(&mut db.types, slot, TypeId::BOOL)
        .unwrap();

    let child = db.hir.lhs(id);
    let child_slot = db.unify.get_slot(child);

    db.unify
        .bind_type(&mut db.types, child_slot, TypeId::BOOL)
        .unwrap();

    // abcdefg

    db.unify
        .bind_type(&mut db.types, child_slot, TypeId::BOOL)
        .unwrap();
}

#[inline(always)]
pub fn synth_addrof(db: &mut Nexus, id: HirId) {
    let slot = db.unify.new_slot(id);

    let lhs_id = db.hir.lhs(id);

    let lhs_slot = db.unify.get_slot(lhs_id);
    let lhs_ty = db.unify.resolve_type(lhs_slot);

    if db.types.kind(lhs_ty) == TyKind::Type {
        db.unify
            .bind_type(&mut db.types, slot, TypeId::TYPE)
            .unwrap();

        let lhs_ty_val = db.consts.get_type(lhs_id);
        let ty_val = db.types.alloc_ptr(lhs_ty_val);

        db.consts.set_type(id, ty_val);

        return;
    }

    let ptr_ty = db.types.alloc_ptr(lhs_ty);

    db.unify.bind_type(&mut db.types, slot, ptr_ty).unwrap();
}

#[inline(always)]
pub fn synth_deref(db: &mut Nexus, id: HirId) {
    let slot = db.unify.new_slot(id);

    let lhs_id = db.hir.lhs(id);

    let lhs_slot = db.unify.get_slot(lhs_id);
    let lhs_ty = db.unify.resolve_type(lhs_slot);

    if db.types.kind(lhs_ty) == TyKind::Type {
        panic!();
    }

    let ptree = TypeId(db.types.payload(lhs_ty));

    db.unify.bind_type(&mut db.types, slot, ptree).unwrap();
}
