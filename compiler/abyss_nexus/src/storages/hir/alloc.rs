use crate::storages::{
    hir::storage::{HirId, HirStorage},
    literals::storage::IntId,
};
use abyss_hir::hir::HirExprKind;

impl HirStorage {
    pub fn alloc(&mut self, kind: HirExprKind, lhs: u32, rhs: u32, extra: u32) -> HirId {
        let id = HirId(self.table.len() as u32);
        self.table.kinds.push(kind);
        self.table.lhs.push(lhs);
        self.table.rhs.push(rhs);
        self.table.extra.push(extra);
        id
    }

    pub fn alloc_int(&mut self, int_id: IntId) -> HirId {
        let hir_id = HirId(self.table.len() as u32);

        self.alloc(HirExprKind::LitInt, int_id.0, 0, 0);

        hir_id
    }

    pub fn alloc_binary(&mut self, op: HirExprKind, lhs: HirId, rhs: HirId) -> HirId {
        let hir_id = HirId(self.table.len() as u32);

        self.alloc(op, lhs.0, rhs.0, 0);

        hir_id
    }
}
