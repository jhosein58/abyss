use abyss_hir::hir::HirExprKind as Hir;
use abyss_nexus::nexus::{FloatId, HirId, IntId, Nexus, SymbolId, TypeId};
use abyss_types::TyKind;

use crate::builder::{FunctionBuilder, ModuleBuilder, TypeBuilder};

pub struct LowerCtx<B: FunctionBuilder> {
    pub vars: Vec<Option<B::Var>>,
}
impl<B: FunctionBuilder> LowerCtx<B> {
    pub fn new(len: usize) -> Self {
        Self {
            vars: vec![None; len],
        }
    }

    pub fn insert(&mut self, sym: SymbolId, var: B::Var) {
        self.vars[sym.0 as usize] = Some(var);
    }

    pub fn get(&self, sym: SymbolId) -> B::Var {
        self.vars[sym.0 as usize].expect("Variable not defined!")
    }
}

pub fn lower_function<M: ModuleBuilder>(db: &Nexus, module: &mut M, symbol: SymbolId) -> M::FuncId {
    let id = db.symbol_hir_range.get_copy(symbol).end;
    let func_ty_id = db.hir_to_type.get_copy(id);

    if db.types.kind(func_ty_id) != TyKind::Func {
        panic!("not a function");
    }

    let ret_ty_id = db.types.func_return(func_ty_id);
    let ret_type = lower_type(db, ret_ty_id, module);

    let params: Vec<M::Type> = db
        .types
        .func_params(func_ty_id)
        .iter()
        .map(|p| lower_type(db, *p, module))
        .collect();

    let func_id = module.declare_func(&format!("fn_{}", symbol.0), &params, ret_type);
    let mut func_builder = module.define_func(func_id);

    let entry_block = func_builder.create_block();
    func_builder.switch_to_block(entry_block);

    let func_node = db.hir.extra(id);
    let func_body = db.hir.extra(func_node);

    let mut ctx = LowerCtx::new(db.symbols.len());
    let body_value = lower_expr(db, &mut ctx, func_body, &mut func_builder);

    if db.types.kind(ret_ty_id) == TyKind::Unit {
        func_builder.ins_ret(None);
    } else {
        func_builder.ins_ret(body_value);
    }

    func_builder.finish();

    func_id
}

fn lower_expr<B: FunctionBuilder>(
    db: &Nexus,
    ctx: &mut LowerCtx<B>,
    id: HirId,
    builder: &mut B,
) -> Option<B::Value> {
    let kind = db.hir.kind(id);

    match kind {
        Hir::LitInt => {
            let int_id = db.hir.lhs(id);
            let val = db.ints.get_copy(IntId(int_id.0));

            let ty_id = db.hir_to_type.get_copy(id);
            let cl_ty = lower_type(db, ty_id, builder);

            if db.types.kind(ty_id) == TyKind::Int {
                Some(builder.ins_iconst(cl_ty, val))
            } else {
                Some(builder.ins_fconst(cl_ty, val as f64))
            }
        }

        Hir::LitFloat => {
            let float_id = db.hir.lhs(id);
            let val = db.floats.get_copy(FloatId(float_id.0));

            let ty_id = db.hir_to_type.get_copy(id);
            let cl_ty = lower_type(db, ty_id, builder);

            Some(builder.ins_fconst(cl_ty, val))
        }

        Hir::BinaryAdd => {
            let lhs_id = db.hir.lhs(id);
            let rhs_id = db.hir.rhs(id);

            let lhs_val = lower_expr(db, ctx, lhs_id, builder)?;
            let rhs_val = lower_expr(db, ctx, rhs_id, builder)?;

            match db.types.kind(db.hir_to_type.get_copy(id)) {
                TyKind::Int | TyKind::UInt => Some(builder.ins_iadd(lhs_val, rhs_val)),
                TyKind::Float => Some(builder.ins_fadd(lhs_val, rhs_val)),

                _ => None,
            }
        }

        Hir::BinarySub => {
            let lhs_id = db.hir.lhs(id);
            let rhs_id = db.hir.rhs(id);

            let lhs_val = lower_expr(db, ctx, lhs_id, builder)?;
            let rhs_val = lower_expr(db, ctx, rhs_id, builder)?;

            match db.types.kind(db.hir_to_type.get_copy(id)) {
                TyKind::Int | TyKind::UInt => Some(builder.ins_isub(lhs_val, rhs_val)),
                TyKind::Float => Some(builder.ins_fsub(lhs_val, rhs_val)),

                _ => None,
            }
        }

        Hir::Var => {
            let symbol_hir_id = db.hir.lhs(id);
            let symbol_type = db.hir_to_type.get_copy(symbol_hir_id);
            let symbol_id = db.hir_to_symbol.get_copy(symbol_hir_id);

            let lty = lower_type(db, symbol_type, builder);
            let var = builder.declare_var(lty);

            let value = lower_expr(db, ctx, db.hir.extra(id), builder);
            builder.def_var(var, value?);

            ctx.insert(symbol_id, var);

            None
        }

        Hir::Ident => {
            let sym = db.hir_to_symbol.get_copy(id);

            let var = ctx.get(sym);

            Some(builder.use_var(var))
        }

        Hir::Block => {
            let nodes = db.get_list_flat(db.hir.lhs(id).0);
            let mut last_value = None;

            for &node_id in nodes {
                last_value = lower_expr(db, ctx, HirId(node_id), builder);
            }

            last_value
        }

        _ => None,
    }
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
