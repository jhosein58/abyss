use abyss_nexus::{
    arena::ArenaId,
    nexus::{HirId, Nexus, SymbolId, TypeId},
};

use crate::diagnostics::report_expected_type;

#[inline(always)]
pub fn check_func(db: &mut Nexus, stack: &mut Vec<TypeId>, id: HirId) {
    let func_id = db.hir.lhs(id);
    let func_sym = SymbolId(db.hir.rhs(id).0);

    synth_func(db, func_id);

    if func_sym.is_some() {
        db.symbol_is_resolving.set(func_sym, false);

        let origin = db.symbols.get_copy(func_sym);
        db.unify.new_slot(origin);
    }

    let ret_id = db.hir.rhs(func_id);

    let ty_id = if ret_id.is_some() {
        let slot = db.unify.get_slot(ret_id);
        let ty_of_ty = db.unify.resolve_type(slot);

        if ty_of_ty.is_some() && ty_of_ty == TypeId::TYPE {
            let val = db.consts.get_type(ret_id);

            if val.is_none() {
                panic!() // FIXME
            }

            val
        } else {
            panic!(); // FIXME
        }
    } else {
        db.types.alloc_unit()
    };

    stack.push(ty_id);
}

#[inline(always)]
pub fn synth_return(db: &mut Nexus, stack: &mut Vec<TypeId>, id: HirId) {
    let slot = db.unify.new_slot(id);
    let never_tyid = db.types.alloc_never();
    db.unify.bind_type(&mut db.types, slot, never_tyid).unwrap(); // FIXME

    let val = db.hir.lhs(id);
    let val_slot = db.unify.get_slot(val);

    if let Some(&l) = stack.last() {
        if l.is_some() {
            db.unify.bind_type(&mut db.types, val_slot, l).unwrap(); // FIXME
        } else {
            let unit_tyid = db.types.alloc_unit();
            db.unify
                .bind_type(&mut db.types, val_slot, unit_tyid)
                .unwrap(); // FIXME
        }
    }
}

// FIXME: logic comptime va eval kardan type ezaafe beshe
#[inline(always)]
pub fn synth_arg(db: &mut Nexus, id: HirId) {
    let slot = db.unify.new_slot(id);

    let ty_hir_id = db.hir.rhs(id);

    let ident_slot = db.unify.new_slot(db.hir.lhs(id));

    let ty_slot = db.unify.get_slot(ty_hir_id);
    let ty_id = db.unify.resolve_type(ty_slot);

    if ty_id != TypeId::TYPE {
        report_expected_type(db, ty_hir_id, ty_id);

        let err_id = db.types.alloc_error();

        db.unify.bind_type(&mut db.types, slot, err_id).unwrap(); // FIXME
        return;
    }

    let type_value = db.consts.get_type(ty_hir_id);

    if type_value.is_none() {
        panic!() // FIXME
    }

    db.consts.set_type(id, type_value);
    db.unify.bind_type(&mut db.types, slot, ty_id).unwrap(); // FIXME
    db.unify.union(&mut db.types, slot, ident_slot).unwrap(); // FIXME
}

#[inline(always)]
pub fn synth_func(db: &mut Nexus, id: HirId) {
    let slot = db.unify.new_slot(id);

    let ret_hir_id = db.hir.rhs(id);
    let real_ret_ty_slot = db.unify.get_slot(ret_hir_id);

    let ret_ty_id = if ret_hir_id.is_none() {
        db.types.alloc_unit()
    } else {
        let real_ret_ty_id = db.unify.resolve_type(real_ret_ty_slot);

        if real_ret_ty_id != TypeId::TYPE {
            report_expected_type(db, ret_hir_id, real_ret_ty_id);
            db.types.alloc_error()
        } else {
            db.consts.get_type(ret_hir_id) // FIXME: is comptime value available ?
        }
    };

    let params = if db.hir.lhs(id).is_some() {
        db.get_list_flat(db.hir.lhs(id).0).to_owned()
    } else {
        vec![] // FIXME: remove allocation
    };

    let params = params
        .iter()
        .map(|p| db.consts.get_type(HirId(*p)))
        .collect::<Vec<TypeId>>(); // FIXME: remove vector allocation

    let func_type = db.types.alloc_func(&params, ret_ty_id);

    db.unify.bind_type(&mut db.types, slot, func_type).unwrap(); // FIXME
}
