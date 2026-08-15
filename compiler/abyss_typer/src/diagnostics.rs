use abyss_nexus::nexus::{HirId, Nexus, TypeId};
use abyss_nexus::storages::diagnostics::{DiagnosticKind, DiagnosticMessage, HintMessage};

pub fn report_binop_mismatch(
    db: &mut Nexus,
    op_id: HirId,
    lhs_id: HirId,
    rhs_id: HirId,
    lhs_ty: TypeId,
    rhs_ty: TypeId,
) {
    let file_id = db.hir_files.get_copy(op_id);
    let span = db.hir_spans.get_copy(op_id);

    let lhs_span = db.hir_spans.get_copy(lhs_id);
    let rhs_span = db.hir_spans.get_copy(rhs_id);

    db.diagnostics.add_label(
        DiagnosticMessage::TypeMismatchBinOpRhs,
        file_id,
        rhs_span,
        true,
    );

    db.diagnostics.add_label(
        DiagnosticMessage::TypeMismatchBinOpLhs,
        file_id,
        lhs_span,
        false,
    );

    db.diagnostics.error(
        DiagnosticKind::TypeMismatch,
        lhs_ty.0,
        rhs_ty.0,
        file_id,
        span,
        Some(HintMessage::TypeMismatchBinOp),
    );
}

pub fn report_decl_type_mismatch(
    db: &mut Nexus,
    decl_id: HirId,
    type_ann_id: HirId,
    init_expr_id: HirId,
    expected_ty: TypeId,
    found_ty: TypeId,
) {
    let file_id = db.hir_files.get_copy(decl_id);
    let type_ann_span = db.hir_spans.get_copy(type_ann_id);
    let init_span = db.hir_spans.get_copy(init_expr_id);

    db.diagnostics.add_label(
        DiagnosticMessage::TypeMismatchDeclFound,
        file_id,
        init_span,
        true,
    );

    db.diagnostics.add_label(
        DiagnosticMessage::TypeMismatchDeclExpected,
        file_id,
        type_ann_span,
        false,
    );

    db.diagnostics.error(
        DiagnosticKind::TypeMismatch,
        expected_ty.0,
        found_ty.0,
        file_id,
        init_span,
        Some(HintMessage::TypeMismatchDecl),
    );
}
