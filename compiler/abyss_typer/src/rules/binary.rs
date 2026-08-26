use abyss_hir::hir::HirExprKind;
use abyss_nexus::nexus::{HirId, Nexus, TypeId};

#[inline(always)]
pub fn synth(db: &mut Nexus, id: HirId) {
    let slot = db.unify.new_slot(id);

    let lhs_id = db.hir.lhs(id);
    let rhs_id = db.hir.rhs(id);

    let lhs_slot = db.unify.get_slot(lhs_id);
    let rhs_slot = db.unify.get_slot(rhs_id);

    match db.unify.union(&mut db.types, lhs_slot, rhs_slot) {
        Err((ta, tb)) => {
            panic!(
                "error in binary, ta: {}, tb: {}",
                db.types.name(ta),
                db.types.name(tb)
            )
        }

        Ok(s) => {
            db.unify.union(&mut db.types, slot, s).unwrap(); // FIXME
        }
    }
}

#[inline(always)]
pub fn synth_logic_and_or(db: &mut Nexus, id: HirId) {
    let slot = db.unify.new_slot(id);
    db.unify
        .bind_type(&mut db.types, slot, TypeId::BOOL)
        .unwrap();

    let lhs_id = db.hir.lhs(id);
    let lhs_slot = db.unify.get_slot(lhs_id);
    db.unify
        .bind_type(&mut db.types, lhs_slot, TypeId::BOOL)
        .unwrap();

    let rhs_id = db.hir.lhs(id);
    let rhs_slot = db.unify.get_slot(rhs_id);
    db.unify
        .bind_type(&mut db.types, rhs_slot, TypeId::BOOL)
        .unwrap();
}

#[inline(always)]
pub fn synth_assign(db: &mut Nexus, id: HirId) {
    let slot = db.unify.new_slot(id);

    let lhs_id = db.hir.lhs(id);
    let rhs_id = db.hir.rhs(id);

    let lhs_slot = db.unify.get_slot(lhs_id);
    let rhs_slot = db.unify.get_slot(rhs_id);

    if db.hir.kind(lhs_id) == HirExprKind::Wildcard {
        db.unify
            .bind_type(&mut db.types, slot, TypeId::UNIT)
            .unwrap();

        return;
    }

    db.unify.union(&mut db.types, lhs_slot, rhs_slot).unwrap();
    db.unify.union(&mut db.types, slot, lhs_slot).unwrap();
}

#[inline(always)]
pub fn synth_binary_comp(db: &mut Nexus, id: HirId) {
    let slot = db.unify.new_slot(id);

    let lhs_id = db.hir.lhs(id);
    let rhs_id = db.hir.rhs(id);

    let lhs_slot = db.unify.get_slot(lhs_id);
    let rhs_slot = db.unify.get_slot(rhs_id);

    db.unify.union(&mut db.types, lhs_slot, rhs_slot).unwrap();
    db.unify
        .bind_type(&mut db.types, slot, TypeId::BOOL)
        .unwrap();
}

#[inline(always)]
pub fn synth_cast(db: &mut Nexus, id: HirId) {
    let slot = db.unify.new_slot(id);
    let lhs_id = db.hir.lhs(id);
    let lhs_slot = db.unify.get_slot(lhs_id);

    let rhs_id = db.hir.rhs(id);
    let rhs_type = db.consts.get_type(rhs_id);

    db.unify
        .bind_type(&mut db.types, lhs_slot, rhs_type)
        .unwrap();

    db.unify.union(&mut db.types, slot, lhs_slot).unwrap();
}
