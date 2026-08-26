use abyss_nexus::nexus::{HirId, NameId, Nexus, TypeId};

#[inline(always)]
pub fn synth(db: &mut Nexus, id: HirId) {
    let slot = db.unify.new_slot(id);

    let names = db.hir.lhs(id);
    let names = db.get_list_flat(names.0).to_owned();
    let names = names.iter().map(|id| NameId(db.hir.lhs(HirId(*id)).0));

    let types = db.hir.rhs(id);
    let types = db.get_list_flat(types.0).to_owned();
    let types = types.iter().map(|id| db.consts.get_type(HirId(*id)));

    let fields = names.zip(types).collect::<Vec<_>>();

    let tyid = db.types.alloc_struct(&fields);

    db.consts.set_type(id, tyid);

    db.unify
        .bind_type(&mut db.types, slot, TypeId::TYPE)
        .unwrap();
}
