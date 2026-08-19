// use abyss_nexus::nexus::{HirId, Nexus};

// pub fn synth(db: &mut Nexus, id: HirId) {
//     let nodes = db.get_list_flat(db.hir.lhs(id).0);

//     if let Some(&last_id) = nodes.last() {
//         db.hir_to_type
//             .set(id, db.hir_to_type.get_copy(HirId(last_id)));
//     } else {
//         db.hir_to_type.set(id, db.types.alloc_unit());
//     }
// }
