use abyss_hir::hir::HirExprKind as Hir;
use abyss_nexus::{
    arena::ArenaId,
    nexus::{HirId, Nexus, SymbolId, TypeId},
};
use abyss_types::TyKind;

use crate::codegen::{CCodeGen, CType, CValue};

fn get_type(db: &mut Nexus, id: HirId) -> TypeId {
    if id.is_none() {
        return TypeId::none();
    }

    let slot = db.unify.get_slot(id);

    if slot.is_none() {
        return TypeId::none();
    }

    db.unify.resolve_type(slot)
}

pub fn lower_function(db: &mut Nexus, ccg: &mut CCodeGen, symbol: SymbolId) {
    let id = db.symbol_hir_range.get_copy(symbol).end;

    if db.hir.kind(id) != Hir::Binding {
        panic!("syntax should be like this: ident :: (...) ... {{ ... }}");
    }

    let func_id = db.hir.extra(id);
    let func_ty_id = get_type(db, func_id);

    if db.types.kind(func_ty_id) != TyKind::Func {
        panic!("not a function");
    }

    let ret_ty_id = db.types.func_return(func_ty_id);
    let ret_type = lower_type(db, ret_ty_id);

    let params: Vec<CType> = db
        .types
        .func_params(func_ty_id)
        .iter()
        .map(|p| lower_type(db, *p))
        .collect();

    let fn_name = &format!("fn_{}", symbol.0);

    ccg.declare_function(fn_name, ret_type.clone());
    ccg.start_function(fn_name, ret_type);

    let func_node = db.hir.extra(id);
    let func_body = db.hir.extra(func_node);

    let body_value = lower_expr(db, func_body, ccg);

    if db.types.kind(ret_ty_id) == TyKind::Unit {
        ccg.gen_return(None);
    } else if let Some(val) = body_value {
        ccg.gen_return(Some(val));
    }

    ccg.end_function();
}

fn lower_expr(db: &mut Nexus, id: HirId, ccg: &mut CCodeGen) -> Option<CValue> {
    let kind = db.hir.kind(id);

    match kind {
        Hir::LitInt => None,

        Hir::Block => None,

        _ => None,
    }
}

fn lower_type(db: &Nexus, ty_id: TypeId) -> CType {
    match db.types.kind(ty_id) {
        TyKind::Unit => CType::Void,

        _ => unimplemented!(),
    }
}
