use abyss_nexus::{
    arena::ArenaId,
    nexus::{HirId, Nexus},
};

#[inline(always)]
pub fn synth(db: &mut Nexus, id: HirId) {
    let ident_id = db.hir.lhs(id);
    let value_id = db.hir.extra(id);

    let ident_slot = db.unify.new_slot(ident_id);
    let value_slot = db.unify.get_slot(value_id);

    if value_id.is_some() {
        db.unify
            .union(&mut db.types, ident_slot, value_slot)
            .unwrap();
    }
}
