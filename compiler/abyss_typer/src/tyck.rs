use abyss_hir::hir::HirExprKind as Hir;
use abyss_nexus::{
    nexus::{HirId, Nexus},
    ranges::HirRange,
    storages::diagnostics::{DiagnosticKind, DiagnosticMessage, HintMessage},
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

        Hir::LitFloat => {
            let tyid = db.types.alloc_float(32);
            db.hir_to_type.set(id, tyid);
        }

        Hir::BinaryAdd => {
            let lhs_hir_id = db.hir.lhs(id);
            let rhs_hir_id = db.hir.rhs(id);

            let mut lhs_ty = db.hir_to_type.get_copy(lhs_hir_id);
            let rhs_ty = db.hir_to_type.get_copy(rhs_hir_id);

            if lhs_ty != rhs_ty {
                let file_id = db.hir_files.get_copy(id);
                let span = db.hir_spans.get_copy(id);

                let lhs_span = db.hir_spans.get_copy(lhs_hir_id);
                let rhs_span = db.hir_spans.get_copy(rhs_hir_id);

                db.diagnostics.add_label(
                    DiagnosticMessage::TypeMismatchBinOpLhs,
                    file_id,
                    lhs_span,
                    false,
                );

                db.diagnostics.add_label(
                    DiagnosticMessage::TypeMismatchBinOpRhs,
                    file_id,
                    rhs_span,
                    true,
                );

                db.diagnostics.error(
                    DiagnosticKind::TypeMismatch,
                    lhs_ty.0,
                    rhs_ty.0,
                    file_id,
                    span,
                    Some(HintMessage::TypeMismatchBinOp),
                );

                lhs_ty = db.types.alloc_error();
            }

            db.hir_to_type.set(id, lhs_ty);
        }

        _ => {}
    }
}
