use abyss_hir::hir::HirExprKind as Hir;
use abyss_nexus::{
    nexus::{HirId, Nexus},
    ranges::HirRange,
};

pub fn check(db: &mut Nexus, range: HirRange) {
    let start = range.start.0 as usize;
    let end = range.end.0 as usize;

    for offset in 0..=(end - start) {
        let id = HirId((start + offset) as u32);

        dispatch(db, id);
    }
}

#[inline]
fn dispatch(db: &mut Nexus, id: HirId) {
    let kind = db.hir.kind(id);

    match kind {
        Hir::LitInt => {
            let tyid = db.types.alloc_int(32);
            db.hir_to_type.set(id, tyid);
        }

        Hir::BinaryAdd => {
            let lhs_hir_id = db.hir.lhs(id);
            let rhs_hir_id = db.hir.rhs(id);

            let lhs_ty = db.hir_to_type.get(lhs_hir_id);
            let rhs_ty = db.hir_to_type.get(rhs_hir_id);

            if lhs_ty != rhs_ty {
                panic!()
            }

            db.hir_to_type.set(id, *lhs_ty);
        }

        _ => {}
    }
}
