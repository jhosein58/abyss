use std::{collections::HashMap, rc::Rc};

use crate::arena_id;

arena_id!(NameId);

#[derive(Default)]
pub struct InternerStorage {
    arena: Vec<Rc<str>>,
    cache: HashMap<Rc<str>, NameId>,
}

impl InternerStorage {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            arena: Vec::with_capacity(capacity),
            cache: HashMap::new(),
        }
    }

    pub fn intern(&mut self, name: &str) -> NameId {
        if let Some(id) = self.cache.get(name) {
            return *id;
        }

        let rc_name: Rc<str> = Rc::from(name);
        let id = NameId(self.arena.len() as u32);

        self.arena.push(rc_name.clone());
        self.cache.insert(rc_name, id);
        id
    }

    pub fn get(&self, id: NameId) -> Option<&str> {
        self.arena.get(id.0 as usize).map(|s| &**s)
    }
}
