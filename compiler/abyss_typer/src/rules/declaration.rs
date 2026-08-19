use abyss_nexus::{
    arena::ArenaId,
    nexus::{HirId, Nexus, TypeId},
};

#[inline(always)]
pub fn synth(db: &mut Nexus, id: HirId) {
    let ident_id = db.hir.lhs(id);
    let type_id = db.hir.rhs(id);
    let value_id = db.hir.extra(id);

    let ident_slot = db.unify.new_slot(ident_id);
    let value_slot = db.unify.get_slot(value_id);

    if value_id.is_some() {
        db.unify
            .union(&mut db.types, ident_slot, value_slot)
            .unwrap();
    }

    if type_id.is_some() {
        let type_slot = db.unify.get_slot(type_id);
        if db.unify.resolve_type(type_slot) != TypeId::TYPE {
            panic!() // FIXME: report error
        }

        let type_value = db.consts.get_type(type_id);

        if type_value.is_none() {
            panic!() // FIXME
        }

        db.unify
            .bind_type(&mut db.types, ident_slot, type_value)
            .unwrap(); // FIXME
    }
}
