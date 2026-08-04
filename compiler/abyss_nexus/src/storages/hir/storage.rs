use abyss_hir::hir::{HirExprKind, HirTable};

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct HirId(pub u32);

#[derive(Default)]
pub struct HirStorage {
    pub table: HirTable,
}

impl HirStorage {
    pub fn set(&mut self, table: HirTable) {
        self.table = table
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.table.len()
    }

    pub fn set_root(&mut self, root: HirId) {
        self.table.root = root.0;
    }

    pub fn root(&self) -> HirId {
        HirId(self.table.root)
    }

    pub fn kind(&self, id: HirId) -> HirExprKind {
        self.table.kinds[id.0 as usize]
    }

    pub fn lhs(&self, id: HirId) -> HirId {
        HirId(self.table.lhs[id.0 as usize])
    }

    pub fn rhs(&self, id: HirId) -> HirId {
        HirId(self.table.rhs[id.0 as usize])
    }

    pub fn extra(&self, id: HirId) -> HirId {
        HirId(self.table.extra[id.0 as usize])
    }
}
