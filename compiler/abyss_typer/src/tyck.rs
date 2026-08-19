use abyss_hir::hir::HirExprKind as Hir;
use abyss_nexus::{
    nexus::{HirId, Nexus},
    ranges::HirRange,
};

use crate::rules::{binary, declaration, ident, literal};

pub fn type_check(db: &mut Nexus, range: HirRange) {
    let start = range.start.0;
    let end = range.end.0;

    for offset in 0..=(end - start) {
        synth_node(db, HirId(start + offset));
    }
}

#[inline(always)]
fn synth_node(db: &mut Nexus, id: HirId) {
    let kind = db.hir.kind(id);

    match kind {
        Hir::LitInt => literal::synth_int(db, id),
        Hir::LitFloat => literal::synth_float(db, id),

        Hir::Ident => ident::synth(db, id),

        Hir::Var => declaration::synth(db, id),

        Hir::BinaryAdd | Hir::BinaryMul | Hir::BinarySub | Hir::BinaryDiv => binary::synth(db, id),

        _ => {}
    }
}
