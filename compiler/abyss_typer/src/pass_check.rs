use abyss_hir::hir::HirExprKind as Hir;
use abyss_nexus::nexus::{HirId, Nexus};

#[inline(always)]
pub fn check_node(db: &mut Nexus, id: HirId) {
    let kind = db.hir.kind(id);

    match kind {
        // Literlas
        Hir::LitInt => println!("int"),
        Hir::LitFloat => println!("float"),

        // Binary
        Hir::BinaryAdd => println!("add"),

        _ => {}
    }
}
