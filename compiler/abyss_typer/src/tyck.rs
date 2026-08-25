use abyss_hir::hir::HirExprKind as Hir;
use abyss_nexus::{
    arena::ArenaId,
    nexus::{HirId, Nexus, SlotId, SymbolId, TypeId},
    ranges::HirRange,
};

use crate::rules::{binary, block, controlflow, declaration, func, literal, structs, unary};

pub trait TyCtx {
    fn db(&self) -> &Nexus;
    fn db_mut(&mut self) -> &mut Nexus;
    fn slot_of(&mut self, sym_id: SymbolId) -> SlotId;
    fn type_of(&mut self, sym_id: SymbolId) -> TypeId;
}

pub struct Typer<'a, T: TyCtx> {
    pub ctx: &'a mut T,
}

impl<'a, T: TyCtx> Typer<'a, T> {
    pub fn new(ctx: &'a mut T) -> Self {
        Self { ctx }
    }

    pub fn type_check(&mut self, range: HirRange) {
        let start = range.start.0 + 1; // skip function name
        let end = range.end.0;

        let mut func_stack = Vec::with_capacity(16);

        for offset in 0..=(end - start) {
            self.synth_node(&mut func_stack, HirId(start + offset));
        }
    }

    #[inline(always)]
    fn synth_node(&mut self, stack: &mut Vec<SlotId>, id: HirId) {
        let db = self.ctx.db_mut();
        let kind = db.hir.kind(id);

        match kind {
            Hir::LitInt => literal::synth_int(db, id),
            Hir::LitFloat => literal::synth_float(db, id),
            Hir::LitBoolTrue | Hir::LitBoolFalse => literal::synth_bool(db, id),

            Hir::Ident => self.synth_ident(id),
            Hir::Wildcard => self.synth_wildcard(id),

            Hir::BinaryAdd | Hir::BinaryMul | Hir::BinarySub | Hir::BinaryDiv => {
                binary::synth(db, id)
            }

            Hir::BinaryAssign => binary::synth_assign(db, id),

            Hir::BinaryGt
            | Hir::BinaryGtEq
            | Hir::BinaryLt
            | Hir::BinaryLtEq
            | Hir::BinaryEqEq
            | Hir::BinaryNeq => binary::synth_binary_comp(db, id),

            Hir::Binding | Hir::Var => declaration::synth(db, id),

            Hir::Arg => func::synth_arg(db, id),
            Hir::MarkerFnStart => func::check_func(db, stack, id),
            Hir::Ret => func::synth_return(db, stack, id),
            Hir::Function => {
                let tyid = db.hir.rhs(id);

                if tyid.is_some() {
                    let tyid = db.consts.get_type(tyid);

                    let block_id = db.hir.extra(id);

                    if block_id.is_some() {
                        let block_slot = db.unify.get_slot(block_id);

                        db.unify.bind_type(&mut db.types, block_slot, tyid).unwrap();
                    }
                }

                stack.pop();
            }
            Hir::Call => self.synth_call(id),

            Hir::Block => block::synth(db, id),

            Hir::If => controlflow::synth_if(db, id),
            Hir::While => controlflow::synth_while(db, id),

            Hir::BinaryAnd | Hir::BinaryOr => binary::synth_logic_and_or(db, id),
            Hir::UnaryNot => unary::synth_not(db, id),

            Hir::Struct => structs::synth(db, id),

            _ => {}
        }
    }
}
