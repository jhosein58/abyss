use abyss_nexus::nexus::Nexus;

use crate::hir::{HirExprKind, HirProgram};

pub struct HirCursor<'a> {
    nexus: &'a Nexus,
    program: &'a HirProgram,
    pointer_stack: Vec<usize>,
}

impl<'a> HirCursor<'a> {
    pub fn new(nexus: &'a Nexus, program: &'a HirProgram) -> Self {
        Self {
            nexus,
            program,
            pointer_stack: vec![program.root as usize],
        }
    }

    pub fn current_type(&self) -> HirExprKind {
        self.program.kinds[self.pointer_stack.last().copied().unwrap()]
    }

    pub fn left(&self) -> HirExprKind {
        self.program.kinds[self.program.lhs[self.pointer_stack.last().copied().unwrap()] as usize]
    }

    pub fn right(&self) -> HirExprKind {
        self.program.kinds[self.program.rhs[self.pointer_stack.last().copied().unwrap()] as usize]
    }
}
