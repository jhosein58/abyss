use std::collections::HashSet;

use abyss_hir::hir::HirExprKind as Hir;
use abyss_nexus::{
    arena::ArenaId,
    nexus::{FloatId, HirId, IntId, NameId, Nexus, SymbolId, TypeId},
};
use abyss_types::TyKind;

use crate::{
    codegen::{CCodeGen, CType, CValue},
    topo_sort::topo_sort,
};

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

    let mut type_queue: HashSet<TypeId> = HashSet::new();

    let ret_type = lower_type(db, ret_ty_id, &mut type_queue);

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
        .map(|p| lower_type(db, *p, &mut type_queue))
        .collect();

    let fn_params: Vec<(&str, CType)> = args_name.iter().map(|s| s.as_str()).zip(params).collect();

    let fn_name = format!("sym_{}", symbol.0);

    let mut compile_queue: HashSet<SymbolId> = HashSet::new();

    ccg.start_function(&fn_name, ret_type, &fn_params);

    let func_node = db.hir.extra(id);
    let func_body = db.hir.extra(func_node);

    let body_value = lower_expr(db, func_body, ccg, &mut compile_queue, &mut type_queue);

    if db.types.kind(ret_ty_id) == TyKind::Unit {
        ccg.gen_return(None);
    } else if let Some(val) = body_value {
        ccg.gen_return(Some(val));
    }

    ccg.end_function();

    for s in compile_queue {
        lower_function(db, ccg, s);
    }

    let sorted = topo_sort(db, &type_queue);

    for t in sorted {
        ccg.def_struct(db, t);
    }
}

fn lower_expr(
    db: &mut Nexus,
    id: HirId,
    ccg: &mut CCodeGen,
    queue: &mut HashSet<SymbolId>,
    type_queue: &mut HashSet<TypeId>,
) -> Option<CValue> {
    let kind = db.hir.kind(id);

    match kind {
        // Literals
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

        Hir::LitBoolTrue => Some(ccg.literal(&format!("true"))),
        Hir::LitBoolFalse => Some(ccg.literal(&format!("false"))),

        Hir::Ident => {
            let sym_id = db.hir_to_symbol.get_copy(id);

            let ty = get_type(db, id);

            if db.types.kind(ty) == TyKind::Func {
                if db.types.func_is_extern(ty) {
                    let name_id = NameId(db.hir.lhs(id).0);
                    let name = db.interner.get(name_id);

                    return Some(CValue(name.to_owned()));
                }
                queue.insert(sym_id);
            }

            Some(CValue(format!("sym_{}", sym_id.0)))
        }

        Hir::Var => {
            let ident_hir_id = db.hir.lhs(id);

            let sym_id = db.hir_to_symbol.get_copy(ident_hir_id);

            let ty = get_type(db, ident_hir_id);
            let ty = lower_type(db, ty, type_queue);

            let init_id = db.hir.extra(id);
            let init = if init_id.is_some() {
                lower_expr(db, init_id, ccg, queue, type_queue)
            } else {
                None
            };

            Some(ccg.create_variable(&format!("sym_{}", sym_id.0), ty, init))
        }

        Hir::BinaryAssign => {
            let rhs = db.hir.rhs(id);
            let rhs = lower_expr(db, rhs, ccg, queue, type_queue).unwrap();

            let lhs = db.hir.lhs(id);

            if db.hir.kind(lhs) == Hir::Wildcard {
                return None;
            }

            let lhs = lower_expr(db, lhs, ccg, queue, type_queue).unwrap();

            Some(ccg.assign(lhs, rhs))
        }

        Hir::BinaryAdd => {
            let lhs = db.hir.lhs(id);
            let lhs = lower_expr(db, lhs, ccg, queue, type_queue).unwrap();

            let rhs = db.hir.rhs(id);
            let rhs = lower_expr(db, rhs, ccg, queue, type_queue).unwrap();

            Some(ccg.add(lhs, rhs))
        }

        Hir::BinarySub => {
            let lhs = db.hir.lhs(id);
            let lhs = lower_expr(db, lhs, ccg, queue, type_queue).unwrap();

            let rhs = db.hir.rhs(id);
            let rhs = lower_expr(db, rhs, ccg, queue, type_queue).unwrap();

            Some(ccg.sub(lhs, rhs))
        }
        Hir::BinaryMul => {
            let lhs = db.hir.lhs(id);
            let lhs = lower_expr(db, lhs, ccg, queue, type_queue).unwrap();

            let rhs = db.hir.rhs(id);
            let rhs = lower_expr(db, rhs, ccg, queue, type_queue).unwrap();

            Some(ccg.mul(lhs, rhs))
        }
        Hir::BinaryDiv => {
            let lhs = db.hir.lhs(id);
            let lhs = lower_expr(db, lhs, ccg, queue, type_queue).unwrap();

            let rhs = db.hir.rhs(id);
            let rhs = lower_expr(db, rhs, ccg, queue, type_queue).unwrap();

            Some(ccg.div(lhs, rhs))
        }

        Hir::BinaryLt => {
            let lhs = db.hir.lhs(id);
            let lhs = lower_expr(db, lhs, ccg, queue, type_queue).unwrap();

            let rhs = db.hir.rhs(id);
            let rhs = lower_expr(db, rhs, ccg, queue, type_queue).unwrap();

            Some(ccg.cmp_lt(lhs, rhs))
        }

        Hir::BinaryLtEq => {
            let lhs = db.hir.lhs(id);
            let lhs = lower_expr(db, lhs, ccg, queue, type_queue).unwrap();

            let rhs = db.hir.rhs(id);
            let rhs = lower_expr(db, rhs, ccg, queue, type_queue).unwrap();

            Some(ccg.cmp_lte(lhs, rhs))
        }

        Hir::BinaryGt => {
            let lhs = db.hir.lhs(id);
            let lhs = lower_expr(db, lhs, ccg, queue, type_queue).unwrap();

            let rhs = db.hir.rhs(id);
            let rhs = lower_expr(db, rhs, ccg, queue, type_queue).unwrap();

            Some(ccg.cmp_gt(lhs, rhs))
        }

        Hir::BinaryGtEq => {
            let lhs = db.hir.lhs(id);
            let lhs = lower_expr(db, lhs, ccg, queue, type_queue).unwrap();

            let rhs = db.hir.rhs(id);
            let rhs = lower_expr(db, rhs, ccg, queue, type_queue).unwrap();

            Some(ccg.cmp_gte(lhs, rhs))
        }

        Hir::BinaryEqEq => {
            let lhs = db.hir.lhs(id);
            let lhs = lower_expr(db, lhs, ccg, queue, type_queue).unwrap();

            let rhs = db.hir.rhs(id);
            let rhs = lower_expr(db, rhs, ccg, queue, type_queue).unwrap();

            Some(ccg.cmp_eq(lhs, rhs))
        }

        Hir::BinaryNeq => {
            let lhs = db.hir.lhs(id);
            let lhs = lower_expr(db, lhs, ccg, queue, type_queue).unwrap();

            let rhs = db.hir.rhs(id);
            let rhs = lower_expr(db, rhs, ccg, queue, type_queue).unwrap();

            Some(ccg.cmp_neq(lhs, rhs))
        }

        Hir::Call => {
            let callee = db.hir.lhs(id);
            let callee = lower_expr(db, callee, ccg, queue, type_queue).unwrap();

            let mut vec = vec![];

            let args = db.hir.rhs(id);
            let args = db
                .get_list_flat(args.0)
                .into_iter()
                .map(|a| HirId(*a))
                .collect::<Vec<_>>();

            for a in args {
                vec.push(lower_expr(db, a, ccg, queue, type_queue).unwrap());
            }

            Some(ccg.call(callee, &vec))
        }

        Hir::Ret => {
            let lhs = db.hir.lhs(id);

            if lhs.is_none() {
                ccg.gen_return(None);
            } else {
                let v = lower_expr(db, lhs, ccg, queue, type_queue);
                ccg.gen_return(v);
            }

            return None;
        }

        Hir::Block => {
            let items = db.get_list_flat(db.hir.lhs(id).0).to_owned();
            let count = items.len();
            let mut last_val = None;

            for (idx, n) in items.into_iter().enumerate() {
                let val = lower_expr(db, HirId(n), ccg, queue, type_queue);

                let is_last = idx + 1 == count;
                if !is_last {
                    ccg.expr(val);
                } else {
                    last_val = val;
                }
            }

            last_val
        }

        Hir::If => {
            let cond_id = db.hir.lhs(id);
            let cond_v = lower_expr(db, cond_id, ccg, queue, type_queue).unwrap();

            let if_ty = get_type(db, id);
            let if_ty = lower_type(db, if_ty, type_queue);

            let thenb_id = db.hir.rhs(id);
            let elseb_id = db.hir.extra(id);

            Some(ccg.gen_if_else(
                db,
                queue,
                type_queue,
                cond_v,
                if_ty,
                |builder, db, q, tq| {
                    if let Some(v) = lower_expr(db, thenb_id, builder, q, tq) {
                        return v;
                    } else {
                        CValue::empty()
                    }
                },
                |builder, db, q, tq| {
                    if elseb_id.is_some() {
                        if let Some(v) = lower_expr(db, elseb_id, builder, q, tq) {
                            return v;
                        } else {
                            CValue::empty()
                        }
                    } else {
                        CValue::empty()
                    }
                },
            ))
        }

        Hir::While => {
            let cond_id = db.hir.lhs(id);
            let cond_v = lower_expr(db, cond_id, ccg, queue, type_queue);

            let body_id = db.hir.rhs(id);

            ccg.gen_while(cond_v.unwrap(), |builder| {
                lower_expr(db, body_id, builder, queue, type_queue);
            });

            None
        }

        Hir::StructInit => {
            let ty = get_type(db, id);
            let ty = lower_type(db, ty, type_queue);

            let names = db.hir.lhs(id);
            let names = db
                .get_list_flat(names.0)
                .iter()
                .map(|n| db.hir.lhs(HirId(*n)).0)
                .collect::<Vec<_>>();

            let vals = db.hir.rhs(id).0;
            let vals = db
                .get_list_flat(vals)
                .iter()
                .map(|v| HirId(*v))
                .collect::<Vec<_>>();

            let vlas = vals
                .iter()
                .map(|v| lower_expr(db, *v, ccg, queue, type_queue).unwrap())
                .collect::<Vec<_>>();

            Some(ccg.gen_struct_init(&names, &vlas, ty))
        }

        Hir::Cast => {
            let ty_id = get_type(db, id);
            let ty = lower_type(db, ty_id, type_queue);

            let lhs_id = db.hir.lhs(id);
            let lhs_v = lower_expr(db, lhs_id, ccg, queue, type_queue);

            if db.types.kind(ty_id) == TyKind::Struct {
                Some(lhs_v.unwrap())
            } else {
                Some(ccg.gen_csat(lhs_v.unwrap(), ty))
            }
        }

        Hir::Member => {
            let lhs_id = db.hir.lhs(id);
            let lhs_v = lower_expr(db, lhs_id, ccg, queue, type_queue);

            let rhs_id = db.hir.rhs(id);
            let field_name = NameId(db.hir.lhs(rhs_id).0);

            Some(ccg.gen_member(lhs_v.unwrap(), field_name))
        }

        _ => None,
    }
}

pub fn lower_type(db: &Nexus, ty_id: TypeId, queue: &mut HashSet<TypeId>) -> CType {
    match db.types.kind(ty_id) {
        TyKind::Unit => CType::Void,
        TyKind::Int => match db.types.payload(ty_id) {
            8 => CType::I8,
            16 => CType::I16,
            32 => CType::I32,
            64 => CType::I64,
            128 => CType::I128,
            _ => unimplemented!(),
        },

        TyKind::UInt => match db.types.payload(ty_id) {
            8 => CType::U8,
            16 => CType::U16,
            32 => CType::U32,
            64 => CType::U64,
            128 => CType::U128,
            _ => unimplemented!(),
        },

        TyKind::Float => match db.types.payload(ty_id) {
            16 => CType::F16,
            32 => CType::F32,
            64 => CType::F64,
            128 => CType::F128,
            _ => unimplemented!(),
        },

        TyKind::Bool => CType::Bool,

        TyKind::Struct => {
            let fields = db.types.get_struct_fields(ty_id);

            fn add_to_queue_with_deps(db: &Nexus, ty: TypeId, queue: &mut HashSet<TypeId>) {
                let kind = db.types.kind(ty);

                match kind {
                    TyKind::Struct => {
                        queue.insert(ty);

                        let fields = db.types.get_struct_fields(ty);

                        for (_, f_ty) in fields {
                            add_to_queue_with_deps(db, f_ty, queue);
                        }
                    }

                    _ => {}
                }
            }

            queue.insert(ty_id);
            for (_, t) in fields {
                add_to_queue_with_deps(db, t, queue);
            }

            CType::Struct(db.types.name(ty_id))
        }

        _ => unimplemented!(),
    }
}
