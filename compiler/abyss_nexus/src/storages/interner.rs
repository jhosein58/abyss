use std::{collections::HashMap, rc::Rc};

use crate::{arena::DirectArena, arena_id};

arena_id!(NameId);

#[derive(Default)]
pub struct InternerStorage {
    arena: DirectArena<NameId, Rc<str>>,
    cache: HashMap<Rc<str>, NameId>,
}

impl InternerStorage {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            arena: DirectArena::with_capacity(capacity),
            cache: HashMap::new(),
        }
    }

    pub fn intern(&mut self, name: &str) -> NameId {
        if let Some(id) = self.cache.get(name) {
            return *id;
        }

        let rc_name: Rc<str> = Rc::from(name);

        let id = self.arena.alloc(rc_name.clone());
        self.cache.insert(rc_name, id);
        id
    }

    pub fn get(&self, id: NameId) -> &str {
        self.arena.get(id)
    }
}
