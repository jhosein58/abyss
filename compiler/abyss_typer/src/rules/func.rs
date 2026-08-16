use abyss_nexus::nexus::{HirId, Nexus};
use abyss_types::TyKind;

pub fn synth_arg(db: &mut Nexus, id: HirId) {
    let ty_hir_id = db.hir.rhs(id);
    let ty_id = db.hir_to_type.get_copy(ty_hir_id);

    if db.types.kind(ty_id) != TyKind::Type {
        panic!("faghat type-e 'Type' morede ghabol vaghe mibashad")
    }
}
