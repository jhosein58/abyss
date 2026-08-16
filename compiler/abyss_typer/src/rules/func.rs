use abyss_nexus::{arena::ArenaId, nexus::{HirId, Nexus}};
use abyss_types::TyKind;

use crate::diagnostics::report_expected_type;

// FIXME: logic comptime va eval kardan type ezaafe beshe
pub fn synth_arg(db: &mut Nexus, id: HirId) {
    let ty_hir_id = db.hir.rhs(id);
    let ty_id = db.hir_to_type.get_copy(ty_hir_id);

    if db.types.kind(ty_id) != TyKind::Type {
        report_expected_type(db, ty_hir_id, ty_id);

        db.hir_to_type.set(id, db.types.alloc_error());
        return;
    }

    let type_value = db.hir_to_type_value.get_copy(ty_hir_id);
    db.hir_to_type_value.set(id, type_value);
    db.hir_to_type.set(id, ty_id);
}

pub fn synth_func(db: &mut Nexus, id: HirId) {
    let ret_hir_id = db.hir.lhs(id);

    let ret_ty_id = if ret_hir_id.is_none() {
        db.types.alloc_uint(width)
    }
}
