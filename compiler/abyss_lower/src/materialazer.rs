use abyss_hir::hir::HirExprKind as Hir;
use abyss_nexus::nexus::{HirId, Nexus, SymbolId};

pub fn build(db: &Nexus, symbol: SymbolId) {
    let range = db.symbol_hir_range.get_copy(symbol);
    build_expr(db, range.end);
}

fn build_expr(db: &Nexus, id: HirId) {
    let kind = db.hir.kind(id);

    match kind {
        Hir::Binding => {
            let value_type = db.hir.extra(id)
        }
        _ => {}
    }
}
