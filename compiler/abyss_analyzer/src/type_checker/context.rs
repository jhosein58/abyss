use std::collections::HashMap;

use abyss_types::{tast::TypedExpr, types::Type};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SymbolKind {
    Variable,
    Constant,
}

#[derive(Debug, Clone)]
pub struct SymbolInfo {
    pub ty: Type,
    pub kind: SymbolKind,
    pub is_mutable: bool,
    pub is_initialized: bool,
    _is_native: bool,
}

pub struct TypeContext {
    scopes: Vec<HashMap<String, SymbolInfo>>,
    pub resolved_globals: HashMap<String, TypedExpr>,
}

impl TypeContext {
    pub fn new() -> Self {
        Self {
            scopes: vec![HashMap::new()],
            resolved_globals: HashMap::new(),
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

    pub fn define(&mut self, name: String, ty: Type) {
        let current_scope = self.scopes.last_mut().expect("Scope stack is empty");
        current_scope.insert(
            name,
            SymbolInfo {
                ty,
                kind: SymbolKind::Variable,
                is_mutable: false,
                is_initialized: true,
                _is_native: false,
            },
        );
    }

    pub fn define_symbol(&mut self, name: String, ty: Type) {
        let current_scope = self.scopes.last_mut().expect("Scope stack is empty");
        current_scope.insert(
            name,
            SymbolInfo {
                ty,
                kind: SymbolKind::Variable,
                is_mutable: false,
                is_initialized: true,
                _is_native: false,
            },
        );
    }

    pub fn define_global(&mut self, name: String, ty: Type) {
        let global_scope = self.scopes.first_mut().expect("Global scope missing");

        let _is_native = if let Type::Signature(_, _, n) = ty {
            n
        } else {
            false
        };

        global_scope.insert(
            name,
            SymbolInfo {
                ty,
                kind: SymbolKind::Constant,
                is_mutable: false,
                is_initialized: true,
                _is_native,
            },
        );
    }

    pub fn register_resolved_global(&mut self, name: String, expr: TypedExpr) {
        self.resolved_globals.insert(name, expr);
    }

    pub fn lookup(&self, name: &str) -> Option<&SymbolInfo> {
        for scope in self.scopes.iter().rev() {
            if let Some(info) = scope.get(name) {
                return Some(info);
            }
        }
        None
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

    pub fn is_defined_in_current_scope(&self, name: &str) -> bool {
        self.scopes
            .last()
            .map(|s| s.contains_key(name))
            .unwrap_or(false)
    }
}
