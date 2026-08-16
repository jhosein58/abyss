use abyss_nexus::{
    arena::ArenaId,
    nexus::{HirId, Nexus},
};

use crate::diagnostics::report_binop_mismatch;

#[inline(always)]
pub fn check(db: &mut Nexus, id: HirId) {
    let expected = db.hir_to_expected.get_copy(id);

    if expected.is_some() {
        db.hir_to_expected.set(db.hir.lhs(id), expected);
        db.hir_to_expected.set(db.hir.rhs(id), expected);
    }
}

#[inline(always)]
pub fn synth(db: &mut Nexus, id: HirId) {
    let lhs_hir_id = db.hir.lhs(id);
    let rhs_hir_id = db.hir.rhs(id);

    let mut lhs_ty = db.hir_to_type.get_copy(lhs_hir_id);
    let rhs_ty = db.hir_to_type.get_copy(rhs_hir_id);

    if lhs_ty != rhs_ty {
        report_binop_mismatch(db, id, lhs_hir_id, rhs_hir_id, lhs_ty, rhs_ty);
        lhs_ty = db.types.alloc_error();
    }

    db.hir_to_type.set(id, lhs_ty);
}
