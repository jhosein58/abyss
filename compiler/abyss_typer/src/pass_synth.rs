use abyss_hir::hir::HirExprKind as Hir;
use abyss_nexus::nexus::{HirId, Nexus};

use crate::rules::{binary, declaration, ident, literal};

#[inline(always)]
pub fn synth_node(db: &mut Nexus, id: HirId) {
    let kind = db.hir.kind(id);

    match kind {
        Hir::Ident => ident::synth(db, id),

        // Literlas
        Hir::LitInt => literal::synth_int(db, id),
        Hir::LitFloat => literal::synth_float(db, id),

        // Binary
        Hir::BinaryAdd => binary::synth_add(db, id),

        Hir::Var => declaration::synth_var(db, id),

        _ => {}
    }
}
