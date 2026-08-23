use abyss_nexus::{
    arena::ArenaId,
    nexus::{HirId, Nexus, TypeId},
};

#[inline(always)]
pub fn synth_if(db: &mut Nexus, id: HirId) {
    let slot = db.unify.new_slot(id);

    let cond_id = db.hir.lhs(id);
    let cond_slot = db.unify.get_slot(cond_id);

    db.unify
        .bind_type(&mut db.types, cond_slot, TypeId::BOOL)
        .unwrap();

    let elseb_id = db.hir.extra(id);

    if elseb_id.is_none() {
        db.unify
            .bind_type(&mut db.types, slot, TypeId::UNIT)
            .unwrap();

        return;
    }

    let elseb_slot = db.unify.get_slot(elseb_id);

    let thenb_id = db.hir.rhs(id);
    let thenb_slot = db.unify.get_slot(thenb_id);

    db.unify
        .union(&mut db.types, elseb_slot, thenb_slot)
        .unwrap();

    db.unify.union(&mut db.types, thenb_slot, slot).unwrap();
}
