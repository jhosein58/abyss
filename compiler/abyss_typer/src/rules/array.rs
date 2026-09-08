use abyss_nexus::nexus::{HirId, Nexus, TypeId};

#[inline(always)]
pub fn synth_array_type(db: &mut Nexus, id: HirId) {
    let slot = db.unify.new_slot(id);

    db.consts.set_type(id, TypeId::BOOL); // TODO

    db.unify
        .bind_type(&mut db.types, slot, TypeId::TYPE)
        .unwrap();
}
