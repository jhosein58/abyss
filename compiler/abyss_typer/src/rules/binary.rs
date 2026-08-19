use abyss_nexus::nexus::{HirId, Nexus, TypeId};

use crate::diagnostics::report_binop_mismatch;

#[inline(always)]
pub fn synth(db: &mut Nexus, id: HirId) {
    let slot = db.unify.new_slot(id);

    let lhs_id = db.hir.lhs(id);
    let rhs_id = db.hir.rhs(id);

    let lhs_slot = db.unify.get_slot(lhs_id);
    let rhs_slot = db.unify.get_slot(rhs_id);

    match db.unify.union(&mut db.types, lhs_slot, rhs_slot) {
        Err((ta, tb)) => {
            report_binop_mismatch(db, id, lhs_id, rhs_id, ta, tb);
            db.unify
                .bind_type(&mut db.types, slot, TypeId::ERROR)
                .unwrap(); // FIXME
        }

        Ok(s) => {
            db.unify.union(&mut db.types, slot, s).unwrap(); // FIXME
        }
    }
}
