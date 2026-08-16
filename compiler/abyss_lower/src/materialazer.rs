use abyss_hir::hir::HirExprKind as Hir;
use abyss_nexus::nexus::{HirId, Nexus, SymbolId, TypeId};
use abyss_types::TyKind;

use crate::builder::{FunctionBuilder, ModuleBuilder, TypeBuilder};

pub fn lower_function<M: ModuleBuilder>(db: &Nexus, module: &mut M, symbol: SymbolId) {
    let id = db.symbol_hir_range.get_copy(symbol).end;
    let func_ty_id = db.hir_to_type.get_copy(id);

    if db.types.kind(func_ty_id) != TyKind::Func {
        panic!("not a function") // FIXME: report a pretty error
    }

    let ret_type = lower_type(db, db.types.func_return(func_ty_id), module);

    let params: Vec<M::Type> = db
        .types
        .func_params(func_ty_id)
        .iter()
        .map(|p| lower_type(db, *p, module))
        .collect();

    let func_id = module.declare_func(&format!("fn_{}", symbol.0), &params, ret_type);

    let mut func_builder = module.define_func(func_id);

    let func_body = db.hir.extra(db.hir.extra(id));

    lower_expr(db, func_body, &mut func_builder);
}

fn lower_type<TB: TypeBuilder>(db: &Nexus, ty_id: TypeId, builder: &mut TB) -> TB::Type {
    match db.types.kind(ty_id) {
        TyKind::Unit => builder.type_unit(),
        TyKind::Int => builder.type_int(db.types.payload(ty_id) as u16),
        TyKind::UInt => builder.type_uint(db.types.payload(ty_id) as u16),
        TyKind::Float => builder.type_float(db.types.payload(ty_id) as u16),
        TyKind::Bool => builder.type_bool(),

        TyKind::Ptr => {
            let pointee = lower_type(db, TypeId(db.types.payload(ty_id)), builder);
            builder.type_ptr(Some(pointee))
        }

        TyKind::Func => {
            let ret_ty = lower_type(db, db.types.func_return(ty_id), builder);

            let param_types: Vec<TB::Type> = db
                .types
                .func_params(ty_id)
                .iter()
                .map(|p| lower_type(db, *p, builder))
                .collect(); // IDEA: shayad beshe allocation ro hazf kard

            builder.type_func(&param_types, ret_ty)
        }
        _ => unimplemented!(),
    }
}

fn lower_expr<B: FunctionBuilder>(db: &Nexus, id: HirId, builder: &mut B) {
    let kind = db.hir.kind(id);

    match kind {
        Hir::Binding => {
            let ty_id = db.hir_to_type.get_copy(id);
            if db.types.kind(ty_id) == TyKind::Func {}
        }
        _ => {}
    }
}
