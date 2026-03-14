use abyss_types::types::Type;
use std::collections::HashMap;

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
    pub is_native: bool,
    pub is_inline: bool,
}

impl SymbolInfo {
    pub fn variable(ty: Type) -> Self {
        Self {
            ty,
            kind: SymbolKind::Variable,
            is_mutable: false,
            is_initialized: true,
            is_native: false,
            is_inline: false,
        }
    }

    pub fn constant(ty: Type, is_inline: bool) -> Self {
        Self {
            ty,
            kind: SymbolKind::Constant,
            is_mutable: false,
            is_initialized: true,
            is_native: false,
            is_inline,
        }
    }

    pub fn native_function(ty: Type) -> Self {
        Self {
            ty,
            kind: SymbolKind::Constant,
            is_mutable: false,
            is_initialized: true,
            is_native: true,
            is_inline: false,
        }
    }

    pub fn is_constant(&self) -> bool {
        self.kind == SymbolKind::Constant
    }
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

    pub fn define(&mut self, name: String, info: SymbolInfo) {
        let current_scope = self.scopes.last_mut().expect("Scope stack is empty");
        current_scope.insert(name, info);
    }

    pub fn define_with_type(&mut self, name: String, ty: Type) {
        self.define(name, SymbolInfo::variable(ty));
    }

    pub fn define_global(&mut self, name: String, info: SymbolInfo) {
        let global_scope = self.scopes.first_mut().expect("Global scope missing");
        global_scope.insert(name, info);
    }

    pub fn lookup(&self, name: &str) -> Option<&SymbolInfo> {
        for scope in self.scopes.iter().rev() {
            if let Some(info) = scope.get(name) {
                return Some(info);
            }
        }
        None
    }

    pub fn lookup_mut(&mut self, name: &str) -> Option<&mut SymbolInfo> {
        for scope in self.scopes.iter_mut().rev() {
            if let Some(info) = scope.get_mut(name) {
                return Some(info);
            }
        }
        None
    }

    pub fn lookup_global(&self, name: &str) -> Option<&SymbolInfo> {
        self.scopes.first()?.get(name)
    }

    pub fn update_type(&mut self, name: &str, new_ty: Type) -> bool {
        for scope in self.scopes.iter_mut().rev() {
            if let Some(info) = scope.get_mut(name) {
                info.ty = new_ty;
                return true;
            }
        }
        false
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

    pub fn is_global_scope(&self) -> bool {
        self.scopes.len() == 1
    }

    pub fn scope_depth(&self) -> usize {
        self.scopes.len()
    }

    pub fn set_inline(&mut self, name: &str, inline: bool) -> bool {
        if let Some(sym) = self.lookup_mut(name) {
            sym.is_inline = inline;
            true
        } else {
            false
        }
    }
}

impl Default for TypeContext {
    fn default() -> Self {
        Self::new()
    }
}
