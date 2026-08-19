use abyss_nexus::{
    arena::ArenaId,
    nexus::{HirId, Nexus},
};

pub fn synth(db: &mut Nexus, id: HirId) {
    let nodes = db.get_list_flat(db.hir.lhs(id).0);
    let last_node = nodes.last().cloned();

    let slot = db.unify.new_slot(id);

    let ty;

    if let Some(last_id) = last_node {
        let ty_slot = db.unify.get_slot(HirId(last_id));

        if ty_slot.is_some() {
            ty = db.unify.resolve_type(ty_slot);
        } else {
            ty = db.types.alloc_unknown(); // FIXME
        }
    } else {
        ty = db.types.alloc_unit();
    }

    db.unify.bind_type(&mut db.types, slot, ty).unwrap(); // FIXME
}
