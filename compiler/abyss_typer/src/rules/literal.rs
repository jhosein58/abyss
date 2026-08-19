use abyss_nexus::nexus::{HirId, Nexus};

#[inline(always)]
pub fn synth_int(db: &mut Nexus, id: HirId) {
    let tyid = db.types.alloc_untyped_int();

    let slot = db.unify.new_slot(id);

    let _ = db.unify.bind_type(slot, tyid);

    db.hir_to_type.set(id, tyid);
}

#[inline(always)]
pub fn synth_float(db: &mut Nexus, id: HirId) {
    let tyid = db.types.alloc_float(32);
    db.hir_to_type.set(id, tyid);
}
