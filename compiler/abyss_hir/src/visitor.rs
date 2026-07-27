use crate::hir::{HirExprKind, HirTable};

pub struct HirVisitor<'a> {
    program: &'a HirTable,
}

impl<'a> HirVisitor<'a> {
    pub fn new(program: &'a HirTable) -> Self {
        Self { program }
    }

    pub fn kind(&self, id: usize) -> HirExprKind {
        self.program.kinds[id]
    }

    pub fn lhs(&self, id: usize) -> usize {
        self.program.lhs[id] as usize
    }

    pub fn rhs(&self, id: usize) -> usize {
        self.program.rhs[id] as usize
    }

    pub fn extra(&self, id: usize) -> usize {
        self.program.extra[id] as usize
    }
}
