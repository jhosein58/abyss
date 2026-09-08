use abyss_nexus::{
    arena::ArenaId,
    nexus::{HirId, Nexus, TypeId},
};

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
