use abyss_hir::hir::HirExprKind as Hir;
use abyss_nexus::{
    arena::ArenaId,
    nexus::{FloatId, HirId, IntId, Nexus, SymbolId, TypeId},
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

    let args_id = db.hir.lhs(func_id);
    let args_name = if args_id.is_some() {
        db.get_list_flat(args_id.0).to_owned()
    } else {
        vec![]
    };

    let args_name: Vec<_> = args_name
        .iter()
        .map(|a| db.hir.lhs(HirId(*a)))
        .map(|a| db.hir_to_symbol.get_copy(a))
        .map(|a| format!("sym_{}", a.0))
        .collect();

    let params: Vec<CType> = db
        .types
        .func_params(func_ty_id)
        .iter()
        .map(|p| lower_type(db, *p))
        .collect();

    let fn_params: Vec<(&str, CType)> = args_name.iter().map(|s| s.as_str()).zip(params).collect();

    let fn_name = format!("sym_{}", symbol.0);

    let mut compile_queue: Vec<SymbolId> = vec![];

    ccg.start_function(&fn_name, ret_type, &fn_params);

    let func_node = db.hir.extra(id);
    let func_body = db.hir.extra(func_node);

    let body_value = lower_expr(db, func_body, ccg, &mut compile_queue);

    if db.types.kind(ret_ty_id) == TyKind::Unit {
        ccg.gen_return(None);
    } else if let Some(val) = body_value {
        ccg.gen_return(Some(val));
    }

    ccg.end_function();

    for s in compile_queue {
        lower_function(db, ccg, s);
    }
}

fn lower_expr(
    db: &mut Nexus,
    id: HirId,
    ccg: &mut CCodeGen,
    queue: &mut Vec<SymbolId>,
) -> Option<CValue> {
    let kind = db.hir.kind(id);

    match kind {
        Hir::LitInt => {
            let lhs = db.hir.lhs(id).0;
            let value = db.ints.get_copy(IntId(lhs));

            Some(ccg.literal(&format!("{}", value)))
        }

        Hir::LitFloat => {
            let lhs = db.hir.lhs(id).0;
            let value = db.floats.get_copy(FloatId(lhs));

            Some(ccg.literal(&format!("{}", value)))
        }

        Hir::Ident => {
            let sym_id = db.hir_to_symbol.get_copy(id);

            let ty = get_type(db, id);

            if db.types.kind(ty) == TyKind::Func {
                queue.push(sym_id);
            }

            Some(CValue(format!("sym_{}", sym_id.0)))
        }

        Hir::Var => {
            let ident_hir_id = db.hir.lhs(id);

            let sym_id = db.hir_to_symbol.get_copy(ident_hir_id);

            let ty = get_type(db, ident_hir_id);
            let ty = lower_type(db, ty);

            let init_id = db.hir.extra(id);
            let init = lower_expr(db, init_id, ccg, queue);

            Some(ccg.create_variable(&format!("sym_{}", sym_id.0), ty, init))
        }

        Hir::Call => {
            let callee = db.hir.lhs(id);
            let callee = lower_expr(db, callee, ccg, queue).unwrap();

            let mut vec = vec![];

            let args = db.hir.rhs(id);
            let args = db
                .get_list_flat(args.0)
                .into_iter()
                .map(|a| HirId(*a))
                .collect::<Vec<_>>();

            for a in args {
                vec.push(lower_expr(db, a, ccg, queue).unwrap());
            }

            Some(ccg.call(callee, &vec))
        }

        Hir::Ret => {
            let lhs = db.hir.lhs(id);

            if lhs.is_none() {
                ccg.gen_return(None);
            } else {
                let v = lower_expr(db, lhs, ccg, queue);
                ccg.gen_return(v);
            }

            return None;
        }

        Hir::Block => {
            let items = db.get_list_flat(db.hir.lhs(id).0).to_owned();
            let count = items.len();
            let mut last_val = None;

            for (idx, n) in items.into_iter().enumerate() {
                let val = lower_expr(db, HirId(n), ccg, queue);

                let is_last = idx + 1 == count;
                if !is_last {
                    ccg.expr(val);
                } else {
                    last_val = val;
                }
            }

            last_val
        }

        _ => None,
    }
}

fn lower_type(db: &Nexus, ty_id: TypeId) -> CType {
    match db.types.kind(ty_id) {
        TyKind::Unit => CType::Void,
        TyKind::Int => CType::I32,

        _ => unimplemented!(),
    }
}
