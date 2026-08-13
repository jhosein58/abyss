use std::u32;

use crate::{
    nexus::{FloatId, HirId, IntId, NameId},
    storages::hir::storage::HirStorage,
};
use abyss_hir::hir::HirExprKind as Hir;

impl HirStorage {
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
        self.alloc(Hir::Ident, ident.0, 0, 0)
    }

    #[inline(always)]
    pub fn alloc_int(&mut self, int_id: IntId) -> HirId {
        self.alloc(Hir::LitInt, int_id.0, 0, 0)
    }

    #[inline(always)]
    pub fn alloc_float(&mut self, float_id: FloatId) -> HirId {
        self.alloc(Hir::LitFloat, float_id.0, 0, 0)
    }

    #[inline(always)]
    pub fn alloc_binary(&mut self, op: Hir, lhs: HirId, rhs: HirId) -> HirId {
        self.alloc(op, lhs.0, rhs.0, 0)
    }

    #[inline(always)]
    pub fn alloc_block(&mut self, items: u32) -> HirId {
        self.alloc(Hir::Block, items, 0, 0)
    }

    #[inline(always)]
    pub fn alloc_function(&mut self, args: u32, ret: u32, body: u32) -> HirId {
        self.alloc(Hir::Function, args, ret, body)
    }

    #[inline(always)]
    pub fn alloc_arg(&mut self, arg: HirId, ty: u32) -> HirId {
        self.alloc(Hir::Arg, arg.0, ty, 0)
    }

    #[inline(always)]
    pub fn alloc_var(&mut self, pattern: HirId, ty: Option<HirId>, value: Option<HirId>) -> HirId {
        self.alloc(
            Hir::Var,
            pattern.0,
            ty.unwrap_or(HirId(u32::MAX)).0,
            value.unwrap_or(HirId(u32::MAX)).0,
        )
    }
    #[inline(always)]
    pub fn alloc_binding(&mut self, name: HirId, ty: Option<HirId>, value: Option<HirId>) -> HirId {
        self.alloc(
            Hir::Binding,
            name.0,
            ty.unwrap_or(HirId(u32::MAX)).0,
            value.unwrap_or(HirId(u32::MAX)).0,
        )
    }
}
