use abyss_nexus::nexus::{HirId, Nexus, TypeId};

pub fn synth(db: &mut Nexus, id: HirId) {
    let nodes = db.get_list_flat(db.hir.lhs(id).0);
    let last_node = nodes.last().cloned();

    let slot = db.unify.new_slot(id);

    if let Some(last_id) = last_node {
        let ty_slot = db.unify.get_slot(HirId(last_id));

        db.unify.union(&mut db.types, slot, ty_slot).unwrap();
    } else {
        db.unify
            .bind_type(&mut db.types, slot, TypeId::UNIT)
            .unwrap();
    }
}
