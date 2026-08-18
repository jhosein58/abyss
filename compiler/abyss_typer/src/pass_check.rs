use abyss_hir::hir::HirExprKind as Hir;
use abyss_nexus::{
    nexus::{HirId, Nexus},
    ranges::HirRange,
};

use crate::rules::{binary, declaration};

#[inline(always)]
pub fn check_all(db: &mut Nexus, range: HirRange) {
    let start = range.start.0;
    let end = range.end.0;

    for offset in (0..=(end - start)).rev() {
        check_node(db, HirId(start + offset));
    }
}

#[inline(always)]
fn check_node(db: &mut Nexus, id: HirId) {
    let kind = db.hir.kind(id);

    match kind {
        Hir::Binding | Hir::Var => declaration::check(db, id),

        // Binary
        Hir::BinaryAdd | Hir::BinarySub | Hir::BinaryMul | Hir::BinaryDiv => binary::check(db, id),
        _ => {}
    }
}
