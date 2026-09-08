use abyss_nexus::{
    arena::ArenaId,
    nexus::{HirId, Nexus, TypeId},
};
use abyss_types::TyKind;

#[inline(always)]
pub fn synth_array_type(db: &mut Nexus, id: HirId) {
    let slot = db.unify.new_slot(id);

    let inner_ty_id = db.hir.lhs(id);
    let inner_ty = db.consts.get_type(inner_ty_id);

    if inner_ty.is_none() {
        panic!()
    }

    let arr_len = db.hir.rhs(id).0;

    let arr_ty = db.types.alloc_array(inner_ty, arr_len);

    db.consts.set_type(id, arr_ty);

    db.unify
        .bind_type(&mut db.types, slot, TypeId::TYPE)
        .unwrap();
}

#[inline(always)]
pub fn synth_array_init(db: &mut Nexus, id: HirId) {
    let slot = db.unify.new_slot(id);

    let list_id = db.hir.lhs(id).0;

    let list_nodes = db
        .get_list_flat(list_id)
        .iter()
        .map(|n| HirId(*n))
        .collect::<Vec<_>>();

    if list_nodes.is_empty() {
        panic!()
    }

    let f = list_nodes.first().unwrap();
    let f_slot = db.unify.get_slot(*f);

    let mut nodes_iter = list_nodes.iter();
    nodes_iter.next();

    for n in nodes_iter {
        let n_slot = db.unify.get_slot(*n);
        db.unify.union(&mut db.types, f_slot, n_slot).unwrap();
    }

    let f_ty = db.unify.resolve_type(f_slot);
    let arr_len = list_nodes.len() as u32;

    let arr_ty = db.types.alloc_array(f_ty, arr_len);

    db.unify.bind_type(&mut db.types, slot, arr_ty).unwrap()
}

#[inline(always)]
pub fn synth_index(db: &mut Nexus, id: HirId) {
    let slot = db.unify.new_slot(id);

    let lhs_id = db.hir.lhs(id);
    let lhs_slot = db.unify.get_slot(lhs_id);

    let lhs_ty = db.unify.resolve_type(lhs_slot);

    if db.types.kind(lhs_ty) != TyKind::Array {
        panic!()
    }

    let array_inner_ty = db.types.get_array_type(lhs_ty);

    db.unify
        .bind_type(&mut db.types, slot, array_inner_ty)
        .unwrap();
}
