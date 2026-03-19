use abyss_types::types::Type;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SymbolKind {
    Variable,
    Constant,
}

#[derive(Debug, Clone)]
pub struct SymbolInfo {
    pub ir_name: String,
    pub ty: Type,
    pub kind: SymbolKind,
    pub is_mutable: bool,
    pub is_initialized: bool,
    pub is_native: bool,
    pub is_foldable: bool,
}

impl SymbolInfo {
    pub fn variable(ir_name: String, ty: Type) -> Self {
        Self {
            ir_name,
            ty,
            kind: SymbolKind::Variable,
            is_mutable: false,
            is_initialized: true,
            is_native: false,
            is_foldable: false,
        }
    }

    pub fn constant(ir_name: String, ty: Type, is_foldable: bool) -> Self {
        Self {
            ir_name,
            ty,
            kind: SymbolKind::Constant,
            is_mutable: false,
            is_initialized: true,
            is_native: false,
            is_foldable,
        }
    }

    pub fn native_function(ir_name: String, ty: Type) -> Self {
        Self {
            ir_name,
            ty,
            kind: SymbolKind::Constant,
            is_mutable: false,
            is_initialized: true,
            is_native: true,
            is_foldable: false,
        }
    }

    pub fn is_constant(&self) -> bool {
        self.kind == SymbolKind::Constant
    }
}

pub struct TypeContext {
    scopes: Vec<HashMap<String, SymbolInfo>>,
    name_counters: HashMap<String, usize>,
}

impl TypeContext {
    pub fn new() -> Self {
        Self {
            scopes: vec![HashMap::new()],
            name_counters: HashMap::new(),
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

    fn generate_ir_name(&mut self, original_name: &str) -> String {
        let count = self
            .name_counters
            .entry(original_name.to_string())
            .or_insert(0);
        *count += 1;

        format!("{}_{}", original_name, count)
    }

    pub fn define(&mut self, name: String, mut info: SymbolInfo) -> String {
        if info.ir_name.is_empty() {
            info.ir_name = self.generate_ir_name(&name);
        }

        let assigned_ir_name = info.ir_name.clone();

        let current_scope = self.scopes.last_mut().expect("Scope stack is empty");
        current_scope.insert(name, info);

        assigned_ir_name
    }

    pub fn define_with_type(&mut self, name: String, ty: Type) -> String {
        let ir_name = self.generate_ir_name(&name);
        self.define(name, SymbolInfo::variable(ir_name, ty))
    }

    pub fn define_global(&mut self, name: String, mut info: SymbolInfo) -> String {
        if info.ir_name.is_empty() {
            info.ir_name = self.generate_ir_name(&name);
        }
        let assigned_ir_name = info.ir_name.clone();

        let global_scope = self.scopes.first_mut().expect("Global scope missing");
        global_scope.insert(name, info);

        assigned_ir_name
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
}

impl Default for TypeContext {
    fn default() -> Self {
        Self::new()
    }
}
