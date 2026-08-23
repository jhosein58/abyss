use abyss_nexus::nexus::{HirId, Nexus};

#[inline(always)]
pub fn synth(db: &mut Nexus, id: HirId) {
    let slot = db.unify.new_slot(id);

    let lhs_id = db.hir.lhs(id);
    let rhs_id = db.hir.rhs(id);

    let lhs_slot = db.unify.get_slot(lhs_id);
    let rhs_slot = db.unify.get_slot(rhs_id);

    match db.unify.union(&mut db.types, lhs_slot, rhs_slot) {
        Err((ta, tb)) => {
            panic!(
                "error in binary, ta: {}, tb: {}",
                db.types.name(ta),
                db.types.name(tb)
            )
        }

        Ok(s) => {
            db.unify.union(&mut db.types, slot, s).unwrap(); // FIXME
        }
    }
}

#[inline(always)]
pub fn synth_logic_and_or(db: &mut Nexus, id: HirId) {
    let slot = db.unify.new_slot(id);
}
