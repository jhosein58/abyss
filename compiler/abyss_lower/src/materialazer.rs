use abyss_nexus::nexus::{HirId, Nexus, SymbolId};

pub fn build(db: &Nexus, symbol: SymbolId) {
    let range = db.symbol_hir_range.get_copy(symbol);

    let HirId(offset) = range.start;
    let HirId(end) = range.end;

    for i in 0..=(end - offset) {
        let _id = HirId(i + offset);

        // println!("{:?}", db.hir.kind(id))
    }
}
