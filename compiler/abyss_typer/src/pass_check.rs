use abyss_hir::hir::HirExprKind as Hir;
use abyss_nexus::nexus::{HirId, Nexus};

use crate::rules::{binary, declaration};

#[inline(always)]
pub fn check_node(db: &mut Nexus, id: HirId) {
    let kind = db.hir.kind(id);

    match kind {
        Hir::Binding | Hir::Var => declaration::check(db, id),

        // Binary
        Hir::BinaryAdd | Hir::BinarySub | Hir::BinaryMul | Hir::BinaryDiv => binary::check(db, id),
        _ => {}
    }
}
