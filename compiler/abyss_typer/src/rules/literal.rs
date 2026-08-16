use abyss_nexus::{
    arena::ArenaId,
    nexus::{HirId, Nexus},
};
use abyss_types::TyKind;

#[inline(always)]
pub fn synth_int(db: &mut Nexus, id: HirId) {
    let tyid;

    let expected = db.hir_to_expected.get_copy(id);
    if expected.is_some() {
        let expected_tyid = db.hir_to_type_value.get_copy(expected);

        match db.types.kind(expected_tyid) {
            TyKind::Int => tyid = db.types.alloc_int(db.types.payload(expected_tyid) as u16),
            TyKind::Float => tyid = db.types.alloc_float(db.types.payload(expected_tyid) as u16),
            _ => tyid = db.types.alloc_int(32),
        }
    } else {
        tyid = db.types.alloc_int(32);
    }

    db.hir_to_type.set(id, tyid);
}

#[inline(always)]
pub fn synth_float(db: &mut Nexus, id: HirId) {
    let tyid = db.types.alloc_float(32);
    db.hir_to_type.set(id, tyid);
}
