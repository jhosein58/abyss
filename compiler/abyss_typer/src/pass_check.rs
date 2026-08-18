use abyss_hir::hir::HirExprKind as Hir;
use abyss_nexus::{
    nexus::{HirId, Nexus},
    ranges::HirRange,
};

use crate::rules::{binary, declaration, func};

#[inline(always)]
pub fn check_all(db: &mut Nexus, range: HirRange) {
    let start = range.start.0;
    let end = range.end.0;

    let mut func_stack = Vec::with_capacity(16);

    for offset in (0..=(end - start)).rev() {
        check_node(db, &mut func_stack, HirId(start + offset));
    }
}

#[inline(always)]
fn check_node(db: &mut Nexus, stack: &mut Vec<HirId>, id: HirId) {
    let kind = db.hir.kind(id);

    match kind {
        Hir::Function => func::check_func(db, stack, id),
        Hir::MarkerFnStart => {
            stack.pop();
        }
        Hir::Ret => func::check_return(db, stack, id),

        Hir::Binding | Hir::Var => declaration::check(db, id),

        // Binary
        Hir::BinaryAdd | Hir::BinarySub | Hir::BinaryMul | Hir::BinaryDiv => binary::check(db, id),
        _ => {}
    }
}
