use abyss_hir::hir::{HirExprKind as Hir, HirTable};

use crate::{
    arena::ArenaId,
    nexus::{FloatId, HirId, IntId, NameId},
};

const NONE: u32 = u32::MAX;

#[derive(Default)]
pub struct HirStorage {
    pub table: HirTable,
}

impl HirStorage {
    #[inline(always)]
    pub fn set(&mut self, table: HirTable) {
        self.table = table
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.table.len()
    }

    #[inline(always)]
    pub fn set_root(&mut self, root: HirId) {
        self.table.root = root.0;
    }

    #[inline(always)]
    pub fn root(&self) -> HirId {
        HirId(self.table.root)
    }

    #[inline(always)]
    pub fn kind(&self, id: HirId) -> Hir {
        self.table.kinds[id.0 as usize]
    }

    #[inline(always)]
    pub fn lhs(&self, id: HirId) -> HirId {
        HirId(self.table.lhs[id.0 as usize])
    }

    #[inline(always)]
    pub fn rhs(&self, id: HirId) -> HirId {
        HirId(self.table.rhs[id.0 as usize])
    }

    #[inline(always)]
    pub fn extra(&self, id: HirId) -> HirId {
        HirId(self.table.extra[id.0 as usize])
    }

    #[inline(always)]
    pub fn ident_name(&self, id: HirId) -> NameId {
        NameId(self.table.lhs[id.0 as usize])
    }

    #[inline(always)]
    pub fn reserve(&mut self, capacity: usize) {
        self.table.reserve(capacity);
    }

    // ===============> alloc <===============
    pub fn alloc(&mut self, kind: Hir, lhs: u32, rhs: u32, extra: u32) -> HirId {
        let id = HirId(self.table.len() as u32);
        self.table.kinds.push(kind);
        self.table.lhs.push(lhs);
        self.table.rhs.push(rhs);
        self.table.extra.push(extra);
        id
    }

    #[inline(always)]
    pub fn alloc_ident(&mut self, ident: NameId) -> HirId {
        self.alloc(Hir::Ident, ident.0, NONE, NONE)
    }

    #[inline(always)]
    pub fn alloc_int(&mut self, int_id: IntId) -> HirId {
        self.alloc(Hir::LitInt, int_id.0, NONE, NONE)
    }

    #[inline(always)]
    pub fn alloc_float(&mut self, float_id: FloatId) -> HirId {
        self.alloc(Hir::LitFloat, float_id.0, NONE, NONE)
    }

    #[inline(always)]
    pub fn alloc_binary(&mut self, op: Hir, lhs: HirId, rhs: HirId) -> HirId {
        self.alloc(op, lhs.0, rhs.0, NONE)
    }

    #[inline(always)]
    pub fn alloc_block(&mut self, items: u32) -> HirId {
        self.alloc(Hir::Block, items, NONE, NONE)
    }

    #[inline(always)]
    pub fn alloc_function(&mut self, args: u32, ret: u32, body: u32) -> HirId {
        self.alloc(Hir::Function, args, ret, body)
    }

    #[inline(always)]
    pub fn alloc_arg(&mut self, arg: HirId, ty: u32) -> HirId {
        self.alloc(Hir::Arg, arg.0, ty, NONE)
    }

    #[inline(always)]
    pub fn alloc_var(&mut self, pattern: HirId, ty: Option<HirId>, value: Option<HirId>) -> HirId {
        self.alloc(
            Hir::Var,
            pattern.0,
            ty.unwrap_or(HirId::none()).0,
            value.unwrap_or(HirId::none()).0,
        )
    }
    #[inline(always)]
    pub fn alloc_binding(&mut self, name: HirId, ty: Option<HirId>, value: Option<HirId>) -> HirId {
        self.alloc(
            Hir::Binding,
            name.0,
            ty.unwrap_or(HirId::none()).0,
            value.unwrap_or(HirId::none()).0,
        )
    }

    #[inline(always)]
    pub fn alloc_error(&mut self) -> HirId {
        self.alloc(Hir::Error, 0, NONE, NONE)
    }

    #[inline(always)]
    pub fn alloc_return(&mut self, value: Option<HirId>) -> HirId {
        self.alloc(Hir::Ret, value.unwrap_or(HirId::none()).0, NONE, NONE)
    }

    #[inline(always)]
    pub fn alloc_call(&mut self, lhs: HirId, args: u32) -> HirId {
        self.alloc(Hir::Call, lhs.0, args, NONE)
    }

    #[inline(always)]
    pub fn alloc_if(&mut self, cond: HirId, thenb: HirId, elseb: Option<HirId>) -> HirId {
        self.alloc(Hir::If, cond.0, thenb.0, elseb.unwrap_or(HirId::none()).0)
    }

    #[inline(always)]
    pub fn alloc_while(&mut self, cond: HirId, body: HirId) -> HirId {
        self.alloc(Hir::While, cond.0, body.0, NONE)
    }

    #[inline(always)]
    pub fn alloc_true(&mut self) -> HirId {}
}
