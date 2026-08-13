use abyss_nexus::{
    arena::ArenaId,
    nexus::{HirId, Nexus},
};

#[inline(always)]
pub fn check_var(db: &mut Nexus, id: HirId) {
    let ty_hir_id = db.hir.rhs(id);
    let val_hir_id = db.hir.extra(id);

    if ty_hir_id.is_some() && val_hir_id.is_some() {
        db.hir_to_expected.set(val_hir_id, ty_hir_id);
    }
}
