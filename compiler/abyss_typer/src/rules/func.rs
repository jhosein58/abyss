use crate::tyck::{TyCtx, Typer};
use abyss_nexus::{
    arena::ArenaId,
    nexus::{HirId, Nexus, SlotId, SymbolId, TypeId},
};

impl<'a, T: TyCtx> Typer<'a, T> {
    #[inline(always)]
    pub fn synth_call(&mut self, id: HirId) {
        let db = self.ctx.db_mut();

        let slot = db.unify.new_slot(id);

        let lhs = db.hir.lhs(id);
        let lhs_slot = db.unify.get_slot(lhs);

        let lhs_type = db.unify.resolve_type(lhs_slot);

        println!("{:?}", db.types.kind(lhs_type));

        // if db.types.kind(lhs_type) != TyKind::Func {
        //     panic!() // FIXME
        // }

        // let ret_type = db.types.func_return(lhs_type);

        // db.unify.bind_type(&mut db.types, slot, ret_type).unwrap(); // FIXME
    }
}

#[inline(always)]
pub fn check_func(db: &mut Nexus, stack: &mut Vec<SlotId>, id: HirId) {
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

        db.unify
            .bind_type(&mut db.types, slot, TypeId::TYPE)
            .unwrap();

        let val = db.consts.get_type(ret_id);

        if val.is_none() {
            panic!() // FIXME
        }

        slot
    } else {
        SlotId::none()
    };

    stack.push(ty_id);
}

#[inline(always)]
pub fn synth_return(db: &mut Nexus, stack: &mut Vec<SlotId>, id: HirId) {
    let slot = db.unify.new_slot(id);

    db.unify
        .bind_type(&mut db.types, slot, TypeId::NEVER)
        .unwrap(); // ERR

    let val = db.hir.lhs(id);

    if val.is_some() {
        let val_slot = db.unify.get_slot(val);

        if let Some(&l) = stack.last() {
            if l.is_some() {
                db.unify.union(&mut db.types, val_slot, l).unwrap(); // ERR
            }
        }
    }
}

// FIXME: logic comptime va eval kardan type ezaafe beshe
#[inline(always)]
pub fn synth_arg(db: &mut Nexus, id: HirId) {
    let ty_hir_id = db.hir.rhs(id);

    let ident_slot = db.unify.get_slot(db.hir.lhs(id));

    let ty_slot = db.unify.get_slot(ty_hir_id);
    let ty_id = db.unify.resolve_type(ty_slot);

    if ty_id != TypeId::TYPE {
        panic!("not a type");
    }

    let type_value = db.consts.get_type(ty_hir_id);

    if type_value.is_none() {
        panic!() // FIXME
    }

    db.unify
        .bind_type(&mut db.types, ident_slot, type_value)
        .unwrap(); // FIXME
}

#[inline(always)]
pub fn synth_func(db: &mut Nexus, id: HirId) {
    let slot = db.unify.new_slot(id);

    let ret_hir_id = db.hir.rhs(id);

    let ret_ty_id = if ret_hir_id.is_none() {
        db.types.alloc_unit()
    } else {
        let ret_slot = db.unify.get_slot(ret_hir_id);
        db.unify
            .bind_type(&mut db.types, ret_slot, TypeId::TYPE)
            .unwrap();

        db.consts.get_type(ret_hir_id)
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
