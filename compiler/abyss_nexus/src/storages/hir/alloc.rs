use crate::{
    nexus::{HirId, IntId, NameId},
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
}
