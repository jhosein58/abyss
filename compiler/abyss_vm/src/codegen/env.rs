use std::collections::HashMap;

pub struct Env {
    pub vars: HashMap<String, u8>,
    pub next_reg: u8,
}

impl Env {
    pub fn new() -> Self {
        Self {
            vars: HashMap::new(),
            next_reg: 0,
        }
    }

    pub fn alloc_reg(&mut self) -> u8 {
        let r = self.next_reg;
        if r == 255 {
            panic!("Register overflow! A single function used more than 255 registers.");
        }
        self.next_reg += 1;
        r
    }

    pub fn declare_var(&mut self, name: String) -> u8 {
        let r = self.alloc_reg();
        self.vars.insert(name, r);
        r
    }

    pub fn get_var(&self, name: &str) -> u8 {
        *self
            .vars
            .get(name)
            .unwrap_or_else(|| panic!("Variable '{}' not found in scope", name))
    }
}
