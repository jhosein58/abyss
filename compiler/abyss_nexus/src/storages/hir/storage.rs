use abyss_hir::hir::{HirExprKind, HirTable};

use crate::nexus::HirId;

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
    pub fn kind(&self, id: HirId) -> HirExprKind {
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
    pub fn reserve(&mut self, capacity: usize) {
        self.table.reserve(capacity);
    }
}
