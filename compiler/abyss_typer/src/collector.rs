use abyss_hir::hir::HirExprKind;
use abyss_nexus::{nexus::Nexus, storages::symbols::storage::SymbolSpan};

pub fn collect(db: &mut Nexus) {
    let _ = visit(db, db.hir.root());
}

fn visit(db: &mut Nexus, id: u32) -> u32 /* first_id */ {
    let kind = db.hir.kind(id);

    match kind {
        // Literals
        HirExprKind::LitInt
        | HirExprKind::LitFloat
        | HirExprKind::LitBool
        | HirExprKind::LitStr
        | HirExprKind::LitCstr
        | HirExprKind::LitChar
        | HirExprKind::Ident => id,

        // Binary
        HirExprKind::BinaryAdd
        | HirExprKind::BinarySub
        | HirExprKind::BinaryMul
        | HirExprKind::BinaryDiv
        | HirExprKind::BinaryMod
        | HirExprKind::BinaryEq
        | HirExprKind::BinaryNeq
        | HirExprKind::BinaryLt
        | HirExprKind::BinaryGt
        | HirExprKind::BinaryLte
        | HirExprKind::BinaryGte
        | HirExprKind::BinaryAnd
        | HirExprKind::BinaryOr
        | HirExprKind::BinaryBitAnd
        | HirExprKind::BinaryPipe
        | HirExprKind::BinaryBitXor
        | HirExprKind::BinaryShl
        | HirExprKind::BinaryShr
        | HirExprKind::BinaryCollon
        | HirExprKind::BinaryConstDef => visit_binary(db, id),

        HirExprKind::Block => visit_block(db, id),
        HirExprKind::Signature => visit_signature(db, id),
        HirExprKind::Def => visit_def(db, id),

        _ => unimplemented!(),
    }
}

fn visit_list(db: &mut Nexus, list: &[u32]) -> u32 {
    let mut min_id = u32::MAX;

    for &item in list {
        let first_id = visit(db, item);
        if first_id < min_id {
            min_id = first_id;
        }
    }

    min_id
}

fn visit_block(db: &mut Nexus, id: u32) -> u32 {
    let items = db.get_list_flat(db.hir.lhs(id)).to_vec();
    visit_list(db, &items)
}

fn visit_signature(db: &mut Nexus, id: u32) -> u32 {
    let lhs_id = db.hir.lhs(id);
    let rhs_id = db.hir.rhs(id);
    let extra_id = db.hir.extra(id);
    let args = db.get_list_flat(lhs_id).to_vec();

    let args_min = visit_list(db, &args);

    let rhs_min = if rhs_id != u32::MAX {
        visit(db, rhs_id)
    } else {
        u32::MAX
    };

    let extra_min = visit(db, extra_id);

    [args_min, rhs_min, extra_min]
        .into_iter()
        .filter(|&v| v != u32::MAX)
        .min()
        .unwrap_or(id)
}

fn visit_def(db: &mut Nexus, id: u32) -> u32 {
    let ident_id = db.hir.lhs(id);
    let rhs_id = db.hir.rhs(id);

    if HirExprKind::Ident != db.hir.kind(ident_id) {
        panic!("Expected Ident in Def node"); // FIXME: use diagnostic
    }

    let symbol_id = db.hir.lhs(ident_id);

    let ident_min = visit(db, ident_id);
    let rhs_min = visit(db, rhs_id);

    let min_id = ident_min.min(rhs_min);

    db.symbols.define(
        symbol_id,
        SymbolSpan {
            start: min_id,
            end: id,
        },
    );

    min_id
}

fn visit_binary(db: &mut Nexus, id: u32) -> u32 {
    visit(db, db.hir.lhs(id)).min(visit(db, db.hir.rhs(id)))
}
