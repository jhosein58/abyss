use abyss_parser::ast::{FunctionDef, StructDef, Type, UnionDef};
use std::collections::{HashMap, VecDeque};
pub struct TypeContext {
    pub concrete_funcs: Vec<FunctionDef>,
    pub concrete_structs: Vec<StructDef>,
    pub concrete_unions: Vec<UnionDef>,
    pub union_struct_defs: Vec<StructDef>,

    pub generic_func_templates: HashMap<String, FunctionDef>,
    pub generic_struct_templates: HashMap<String, StructDef>,

    pub monomorphization_cache: HashMap<(String, String), String>,
    pub reverse_struct_map: HashMap<String, (String, Vec<Type>)>,
    pub variant_cache: HashMap<String, Vec<Type>>,
    pub used_type_tags: HashMap<String, i64>,

    pub scopes: Vec<HashMap<String, Type>>,
    pub local_func_scopes: Vec<HashMap<String, FunctionDef>>,

    pub pending_funcs: VecDeque<FunctionDef>,

    pub type_aliases: HashMap<String, Type>,
}

impl TypeContext {
    pub fn new() -> Self {
        Self {
            concrete_funcs: Vec::new(),
            concrete_structs: Vec::new(),
            concrete_unions: Vec::new(),
            union_struct_defs: Vec::new(),
            generic_func_templates: HashMap::new(),
            generic_struct_templates: HashMap::new(),
            monomorphization_cache: HashMap::new(),
            reverse_struct_map: HashMap::new(),
            variant_cache: HashMap::new(),
            used_type_tags: HashMap::new(),
            scopes: vec![HashMap::new()],
            local_func_scopes: vec![HashMap::new()],
            pending_funcs: VecDeque::new(),
            type_aliases: HashMap::new(),
        }
    }

    pub fn enter_scope(&mut self) {
        self.scopes.push(HashMap::new());
        self.local_func_scopes.push(HashMap::new());
    }

    pub fn exit_scope(&mut self) {
        self.scopes.pop();
        self.local_func_scopes.pop();
    }

    pub fn register_local_func(&mut self, name: String, func: FunctionDef) {
        if let Some(scope) = self.local_func_scopes.last_mut() {
            scope.insert(name, func);
        }
    }

    pub fn get_local_func(&self, name: &str) -> Option<FunctionDef> {
        for scope in self.local_func_scopes.iter().rev() {
            if let Some(func) = scope.get(name) {
                return Some(func.clone());
            }
        }
        None
    }

    pub fn register_var(&mut self, name: String, ty: Type) {
        if let Some(scope) = self.scopes.last_mut() {
            scope.insert(name, ty);
        }
    }
    pub fn get_var_type(&self, name: &str) -> Option<Type> {
        for scope in self.scopes.iter().rev() {
            if let Some(ty) = scope.get(name) {
                return Some(ty.clone());
            }
        }
        None
    }
    pub fn resolve_var(&self, name: &str) -> Option<Type> {
        for scope in self.scopes.iter().rev() {
            if let Some(ty) = scope.get(name) {
                return Some(ty.clone());
            }
        }
        None
    }
    pub fn register_type_alias(&mut self, name: String, ty: Type) {
        self.type_aliases.insert(name, ty);
    }
    pub fn get_type_alias(&mut self, name: String) -> Option<&Type> {
        self.type_aliases.get(&name)
    }
}
