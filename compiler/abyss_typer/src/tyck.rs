use abyss_hir::hir::HirExprKind as Hir;
use abyss_nexus::{
    nexus::{HirId, Nexus, TypeId},
    ranges::HirRange,
};

use crate::rules::{binary, block, declaration, func, ident, literal};

pub struct Typer<'db> {
    db: &'db mut Nexus,
}

impl<'db> Typer<'db> {
    pub fn new(db: &'db mut Nexus) -> Self {
        Self { db }
    }
}

pub fn type_check(db: &mut Nexus, range: HirRange) {
    let start = range.start.0;
    let end = range.end.0;

    let mut func_stack = Vec::with_capacity(16);

    for offset in 0..=(end - start) {
        synth_node(db, &mut func_stack, HirId(start + offset));
    }
}

#[inline(always)]
fn synth_node(db: &mut Nexus, stack: &mut Vec<TypeId>, id: HirId) {
    let kind = db.hir.kind(id);

    match kind {
        Hir::LitInt => literal::synth_int(db, id),
        Hir::LitFloat => literal::synth_float(db, id),

        Hir::Ident => ident::synth(db, id),

        Hir::BinaryAdd | Hir::BinaryMul | Hir::BinarySub | Hir::BinaryDiv => binary::synth(db, id),

        Hir::Binding | Hir::Var => declaration::synth(db, id),

        Hir::Arg => func::synth_arg(db, id),
        Hir::MarkerFnStart => func::check_func(db, stack, id),
        Hir::Ret => func::synth_return(db, stack, id),
        Hir::Function => {
            stack.pop();
        }

        Hir::Block => block::synth(db, id),

        _ => {}
    }
}
