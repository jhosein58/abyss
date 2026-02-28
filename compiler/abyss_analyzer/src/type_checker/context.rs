use std::collections::HashMap;

use super::types::Type;

#[derive(Debug, Clone)]
pub struct SymbolInfo {
    pub ty: Type,
    pub is_mutable: bool,
    pub is_initialized: bool,
}

pub struct TypeContext {
    scopes: Vec<HashMap<String, SymbolInfo>>,
}

impl TypeContext {
    pub fn new() -> Self {
        Self {
            scopes: vec![HashMap::new()],
        }
    }

    pub fn enter_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    pub fn exit_scope(&mut self) {
        if self.scopes.len() > 1 {
            self.scopes.pop();
        } else {
            panic!("Compiler Bug: Cannot exit global scope!");
        }
    }

    pub fn define(&mut self, name: String, ty: Type, is_mutable: bool) {
        let current_scope = self.scopes.last_mut().expect("Scope stack is empty");

        current_scope.insert(
            name,
            SymbolInfo {
                ty,
                is_mutable,
                is_initialized: true,
            },
        );
    }

    pub fn assign(&mut self, name: &str) -> Result<(), String> {
        for scope in self.scopes.iter_mut().rev() {
            if let Some(info) = scope.get_mut(name) {
                if !info.is_mutable {
                    return Err(format!(
                        "Cannot assign twice to immutable variable '{}'",
                        name
                    ));
                }
                info.is_initialized = true;
                return Ok(());
            }
        }

        Err(format!("Cannot find variable '{}' in this scope.", name))
    }

    pub fn lookup(&self, name: &str) -> Option<&SymbolInfo> {
        for scope in self.scopes.iter().rev() {
            if let Some(info) = scope.get(name) {
                return Some(info);
            }
        }
        None
    }

    pub fn is_defined_in_current_scope(&self, name: &str) -> bool {
        self.scopes
            .last()
            .map(|s| s.contains_key(name))
            .unwrap_or(false)
    }
}
