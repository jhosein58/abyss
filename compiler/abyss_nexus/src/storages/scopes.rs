use std::collections::HashMap;

use crate::{
    arena::SideTable,
    nexus::{HirId, NameId, ScopeId},
};

pub struct ScopeData {
    pub parent: Option<ScopeId>,
    pub nodes: HashMap<NameId, HirId>,
}

#[derive(Default)]
pub struct ScopeStorage {
    scopes: Vec<ScopeData>,
    scope_of: SideTable<HirId, ScopeId>,
}

impl ScopeStorage {
    pub fn alloc(&mut self, parent: Option<ScopeId>) -> ScopeId {
        let id = ScopeId(self.scopes.len() as u32);

        self.scopes.push(ScopeData {
            parent,
            nodes: HashMap::new(),
        });

        id
    }

    pub fn bind(&mut self, scope: ScopeId, name: NameId, hir: HirId) {
        self.scopes[scope.0 as usize].nodes.insert(name, hir);
    }

    fn lookup_local(&self, scope: ScopeId, name: NameId) -> Option<HirId> {
        let scope = &self.scopes[scope.0 as usize];
        scope.nodes.get(&name).copied()
    }

    pub fn lookup(&self, scope: ScopeId, name: NameId) -> Option<HirId> {
        self.lookup_local(scope, name).or_else(|| {
            self.scopes[scope.0 as usize]
                .parent
                .map(|p| self.lookup(p, name))?
        })
    }

    pub fn grow_to(&mut self, cap: usize) {
        self.scope_of.grow_to(cap);
    }

    pub fn set(&mut self, hir: HirId, scope: ScopeId) {
        self.scope_of.set(hir, scope);
    }

    pub fn scope_of(&self, hir: HirId) -> ScopeId {
        self.scope_of.get_copy(hir)
    }
}
