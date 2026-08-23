use abyss_nexus::nexus::{HirId, Nexus};

#[inline(always)]
pub fn synth_int(db: &mut Nexus, id: HirId) {
    let tyid = db.types.alloc_untyped_int();
    let slot = db.unify.new_slot(id);
    db.unify.bind_type(&mut db.types, slot, tyid).unwrap();
}

#[inline(always)]
pub fn synth_float(db: &mut Nexus, id: HirId) {
    let tyid = db.types.alloc_untyped_float();
    let slot = db.unify.new_slot(id);
    db.unify.bind_type(&mut db.types, slot, tyid).unwrap();
}

#[inline(always)]
pub fn synth_bool(db: &mut Nexus, id: HirId) {
    let tyid = db.types.alloc_bool();
    let slot = db.unify.new_slot(id);
    db.unify.bind_type(&mut db.types, slot, tyid).unwrap();
}
