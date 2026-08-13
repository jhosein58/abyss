use abyss_hir::hir::HirExprKind as Hir;
use abyss_nexus::nexus::{HirId, Nexus};

use crate::rules::declaration;

#[inline(always)]
pub fn check_node(db: &mut Nexus, id: HirId) {
    let kind = db.hir.kind(id);

    match kind {
        Hir::Var => declaration::check_var(db, id),

        // Binary
        Hir::BinaryAdd => println!("add"),

        _ => {}
    }
}
