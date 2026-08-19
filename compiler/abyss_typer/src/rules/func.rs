use abyss_nexus::nexus::{HirId, Nexus};

#[inline(always)]
pub fn synth_return(db: &mut Nexus, id: HirId) {
    let slot = db.unify.new_slot(id);
    let never_tyid = db.types.alloc_never();
    db.unify.bind_type(&mut db.types, slot, never_tyid).unwrap(); // FIXME
}

// // FIXME: logic comptime va eval kardan type ezaafe beshe
// #[inline(always)]
// pub fn synth_arg(db: &mut Nexus, id: HirId) {
//     let ty_hir_id = db.hir.rhs(id);
//     let ty_id = db.hir_to_type.get_copy(ty_hir_id);

//     if db.types.kind(ty_id) != TyKind::Type {
//         report_expected_type(db, ty_hir_id, ty_id);

//         db.hir_to_type.set(id, db.types.alloc_error());
//         return;
//     }

//     let type_value = db.hir_to_type_value.get_copy(ty_hir_id);
//     db.hir_to_type_value.set(id, type_value);
//     db.hir_to_type.set(id, ty_id);
// }

// #[inline(always)]
// pub fn synth_func(db: &mut Nexus, id: HirId) {
//     let ret_hir_id = db.hir.rhs(id);

//     let ret_ty_id = if ret_hir_id.is_none() {
//         db.types.alloc_unit()
//     } else {
//         let real_ret_ty_id = db.hir_to_type.get_copy(ret_hir_id);

//         if db.types.kind(real_ret_ty_id) != TyKind::Type {
//             report_expected_type(db, ret_hir_id, real_ret_ty_id);
//             db.types.alloc_error()
//         } else {
//             db.hir_to_type_value.get_copy(ret_hir_id)
//         }
//     };

//     let params = if db.hir.lhs(id).is_some() {
//         db.get_list_flat(db.hir.lhs(id).0)
//     } else {
//         &[]
//     };

//     let params = params
//         .iter()
//         .map(|p| db.hir_to_type_value.get_copy(HirId(*p)))
//         .collect::<Vec<TypeId>>(); // FIXME: remove vector allocation

//     let func_type = db.types.alloc_func(&params, ret_ty_id);

//     db.hir_to_type.set(id, func_type);
// }
