use abyss_nexus::nexus::{HirId, Nexus, TypeId};

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
