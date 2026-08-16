use abyss_hir::hir::HirExprKind as Hir;
use abyss_nexus::nexus::{HirId, Nexus, SymbolId, TypeId};
use abyss_types::TyKind;

use crate::builder::TypeBuilder;

pub fn build(db: &Nexus, symbol: SymbolId) {
    let range = db.symbol_hir_range.get_copy(symbol);
    build_expr(db, range.end);
}

fn lower_type<TB: TypeBuilder>(db: &Nexus, type_id: TypeId, builder: &mut TB) -> TB::Type {
    match db.types.kind(type_id) {
        TyKind::Unit => builder.type_unit(),
        _ => unimplemented!(),
    }
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
