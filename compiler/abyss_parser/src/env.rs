use abyss_nexus::nexus::{NameId, SymbolId};

pub struct ScopeEnv {
    bindings: Vec<(NameId, SymbolId)>, // PERF: shayad beshe ino behtar kard
}

impl ScopeEnv {
    pub fn new() -> Self {
        Self {
            bindings: Vec::with_capacity(512),
        }
    }

    #[inline(always)]
    pub fn mark(&self) -> usize {
        self.bindings.len()
    }

    #[inline(always)]
    pub fn define(&mut self, name: NameId, sym_id: SymbolId) {
        self.bindings.push((name, sym_id));
    }

    #[inline(always)]
    pub fn reset(&mut self, mark: usize) {
        self.bindings.truncate(mark);
    }

    #[inline(always)]
    pub fn lookup(&self, name: NameId) -> Option<SymbolId> {
        for (id, sym) in self.bindings.iter().rev() {
            if *id == name {
                return Some(*sym);
            }
        }
        None
    }
}
