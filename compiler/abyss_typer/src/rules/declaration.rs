use abyss_nexus::{
    arena::ArenaId,
    nexus::{HirId, Nexus},
};
use abyss_types::TyKind;

use crate::diagnostics::{report_decl_type_mismatch, report_expected_type};

#[inline(always)]
pub fn synth(db: &mut Nexus, id: HirId) {
    let ty_hir_id = db.hir.rhs(id);
    let val_hir_id = db.hir.extra(id);
    let val_ty_id = db.hir_to_type.get_copy(val_hir_id);

    let err_id = db.types.alloc_error();

    if ty_hir_id.is_some() {
        let ty_id = db.hir_to_type.get_copy(ty_hir_id);

        if db.types.kind(ty_id) != TyKind::Type {
            db.hir_to_type.set(id, err_id);
            db.hir_to_type.set(id, err_id);
            report_expected_type(db, ty_hir_id, ty_id);
            return;
        }

        let ty_id = db.hir_to_type_value.get_copy(ty_hir_id); // for now, make it work.

        if val_ty_id != ty_id {
            db.hir_to_type.set(db.hir.lhs(id), err_id);
            db.hir_to_type.set(id, err_id);
            report_decl_type_mismatch(db, id, ty_hir_id, val_hir_id, ty_id, val_ty_id);
            return;
        }

        db.hir_to_type.set(db.hir.lhs(id), ty_id);
        db.hir_to_type.set(id, ty_id);
    } else {
        db.hir_to_type.set(db.hir.lhs(id), val_ty_id);
        db.hir_to_type.set(id, val_ty_id);
    }
}
