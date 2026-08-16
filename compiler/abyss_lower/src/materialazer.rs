use abyss_hir::hir::HirExprKind as Hir;
use abyss_nexus::nexus::{HirId, Nexus, SymbolId};
use abyss_types::TyKind;

pub fn build(db: &Nexus, symbol: SymbolId) {
    let range = db.symbol_hir_range.get_copy(symbol);
    build_expr(db, range.end);
}

fn build_expr(db: &Nexus, id: HirId) {
    let kind = db.hir.kind(id);

    match kind {
        Hir::Binding => {
            let ty_id = db.hir_to_type.get_copy(id);
            if db.types.kind(ty_id) == TyKind::Func {}
        }
        _ => {}
    }
}
