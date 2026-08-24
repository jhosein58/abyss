use abyss_nexus::{
    arena::ArenaId,
    nexus::{HirId, Nexus, TypeId},
};

#[inline(always)]
pub fn synth(db: &mut Nexus, id: HirId) {
    let ident_id = db.hir.lhs(id);
    let type_id = db.hir.rhs(id);
    let value_id = db.hir.extra(id);

    let slot = db.unify.new_slot(id);

    let ident_slot = db.unify.get_slot(ident_id);

    if value_id.is_some() {
        let value_slot = db.unify.get_slot(value_id);

        if value_id.is_some() {
            db.unify
                .union(&mut db.types, ident_slot, value_slot)
                .unwrap();
        }
    }

    if type_id.is_some() {
        let type_slot = db.unify.get_slot(type_id);

        db.unify
            .bind_type(&mut db.types, type_slot, TypeId::TYPE)
            .expect("not a type"); // ERR

        let type_value = db.consts.get_type(type_id);

        if type_value.is_none() {
            panic!() // ERR
        }

        db.unify
            .bind_type(&mut db.types, ident_slot, type_value)
            .unwrap(); // ERR
    }

    db.unify.union(&mut db.types, slot, ident_slot).unwrap(); // ERR
}
