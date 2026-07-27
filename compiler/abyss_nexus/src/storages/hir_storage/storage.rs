use abyss_hir::hir::HirTable;

#[derive(Default)]
pub struct HirStorage {
    pub table: HirTable,
}

impl HirStorage {
    pub fn set(&mut self, table: HirTable) {
        self.table = table
    }
}
