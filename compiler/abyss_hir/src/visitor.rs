use abyss_nexus::nexus::Nexus;

use crate::hir::HirProgram;

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
}
