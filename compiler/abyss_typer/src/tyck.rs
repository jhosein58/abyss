use abyss_hir::hir::HirExprKind as Hir;
use abyss_nexus::{
    nexus::{HirId, Nexus},
    ranges::HirRange,
};

use crate::rules::{binary, block, declaration, func, ident, literal};

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
        Hir::Ident => ident::synth(db, id),

        // Literlas
        Hir::LitInt => literal::synth_int(db, id),
        Hir::LitFloat => literal::synth_float(db, id),

        // Binary
        Hir::BinaryAdd | Hir::BinarySub | Hir::BinaryMul | Hir::BinaryDiv => binary::synth(db, id),

        Hir::Binding | Hir::Var => declaration::synth(db, id), // FIXME: bayad jodaa beshe be hamrah type 'const'

        Hir::Arg => func::synth_arg(db, id),
        Hir::Function => func::synth_func(db, id),
        Hir::Ret => func::synth_return(db, id),

        Hir::Block => block::synth(db, id),

        _ => {}
    }
}
