use abyss_hir::hir::{HirExprKind, HirTable};

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

    pub fn root(&self) -> u32 {
        self.table.root
    }

    pub fn kind(&self, id: u32) -> HirExprKind {
        self.table.kinds[id as usize]
    }

    pub fn lhs(&self, id: u32) -> u32 {
        self.table.lhs[id as usize]
    }

    pub fn rhs(&self, id: u32) -> u32 {
        self.table.rhs[id as usize]
    }

    pub fn extra(&self, id: u32) -> u32 {
        self.table.extra[id as usize]
    }
}
