use std::{collections::HashMap, rc::Rc};

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NameId(pub u32);

#[derive(Default)]
pub struct InternerStorage {
    pub arena: Vec<Rc<str>>,
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
