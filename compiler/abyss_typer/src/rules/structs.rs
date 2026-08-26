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

#[inline(always)]
pub fn synth_init(db: &mut Nexus, id: HirId) {
    let slot = db.unify.new_slot(id);

    let fields_id = db.hir.lhs(id);
    let fields_id = db
        .get_list_flat(fields_id.0)
        .into_iter()
        .map(|h| HirId(*h))
        .map(|i| NameId(db.hir.lhs(i).0))
        .collect::<Vec<_>>();

    let vals_id = db.hir.rhs(id);
    let vals_types = db
        .get_list_flat(vals_id.0)
        .into_iter()
        .map(|h| HirId(*h))
        .collect::<Vec<_>>();

    let vals_types = vals_types
        .into_iter()
        .map(|t| db.unify.get_slot(t))
        .collect::<Vec<_>>()
        .iter()
        .map(|s| db.unify.resolve_type(*s))
        .collect::<Vec<TypeId>>();

    let fields = fields_id
        .into_iter()
        .zip(vals_types.into_iter())
        .collect::<Vec<_>>();

    let tyid = db.types.alloc_struct(&fields);

    db.unify.bind_type(&mut db.types, slot, tyid).unwrap();
}
