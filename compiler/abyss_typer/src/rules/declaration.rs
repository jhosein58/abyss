use abyss_nexus::{
    arena::ArenaId,
    nexus::{HirId, Nexus},
};

use crate::diagnostics::report_decl_type_mismatch;

#[inline(always)]
pub fn check_var(db: &mut Nexus, id: HirId) {
    let ty_hir_id = db.hir.rhs(id);
    let val_hir_id = db.hir.extra(id);

    if ty_hir_id.is_some() && val_hir_id.is_some() {
        db.hir_to_expected.set(val_hir_id, ty_hir_id);
    }
}

#[inline(always)]
pub fn synth_var(db: &mut Nexus, id: HirId) {
    let ty_hir_id = db.hir.rhs(id);
    let val_hir_id = db.hir.extra(id);
    let val_ty_id = db.hir_to_type.get_copy(val_hir_id);

    let ty_id = if ty_hir_id.is_none() {
        val_ty_id
    } else {
        db.hir_to_type.get_copy(ty_hir_id)
    };

    if ty_id != val_ty_id {
        db.hir_to_type.set(db.hir.lhs(id), db.types.alloc_error());
        report_decl_type_mismatch(db, id, ty_hir_id, val_hir_id, ty_id, val_ty_id);
        return;
    }

    db.hir_to_type.set(db.hir.lhs(id), ty_id);
}
