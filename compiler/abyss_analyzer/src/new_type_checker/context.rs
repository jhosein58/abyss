use abyss_parser::ast::{FunctionDef, StaticDef, StructDef, Type, TypeAlias};
use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Default)]
pub struct ScopeNode {
    pub symbols: HashMap<String, Type>,
    pub children: Vec<ScopeNode>,
}

#[derive(Debug, Default)]
pub struct TypeContext {
    pub concrete_funcs: HashMap<String, FunctionDef>,
    pub concrete_structs: HashMap<String, StructDef>,

    pub type_aliases: HashMap<String, Type>,
    pub statics: HashMap<String, StaticDef>,

    pub generic_func_templates: HashMap<String, FunctionDef>,
    pub generic_struct_templates: HashMap<String, StructDef>,

    pub observed_generic_funcs: HashSet<(String, Vec<Type>)>,
    pub observed_generic_structs: HashSet<(String, Vec<Type>)>,

    pub pending_func_instantiations: Vec<(String, Vec<Type>)>,
    pub pending_struct_instantiations: Vec<(String, Vec<Type>)>,

    pub function_scopes: HashMap<String, ScopeNode>,

    pub current_func_name: Option<String>,
    pub scope_path: Vec<usize>,
    pub visit_counts: Vec<usize>,
}

impl TypeContext {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }

    pub fn get_function(&self, name: &str) -> Option<&FunctionDef> {
        self.concrete_funcs
            .get(name)
            .or_else(|| self.generic_func_templates.get(name))
    }

    pub fn function_exists(&self, name: &str) -> bool {
        self.concrete_funcs.contains_key(name) || self.generic_func_templates.contains_key(name)
    }

    pub fn get_struct(&self, name: &str) -> Option<&StructDef> {
        self.concrete_structs
            .get(name)
            .or_else(|| self.generic_struct_templates.get(name))
    }

    pub fn struct_exists(&self, name: &str) -> bool {
        self.concrete_structs.contains_key(name) || self.generic_struct_templates.contains_key(name)
    }

    pub fn get_type_alias(&self, name: &str) -> Option<&Type> {
        self.type_aliases.get(name)
    }

    pub fn get_static(&self, name: &str) -> Option<&StaticDef> {
        self.statics.get(name)
    }

    pub fn register_concrete_func(&mut self, func: FunctionDef) -> Result<(), String> {
        let name = func.name.clone();
        if self.function_exists(&name) {
            return Err(format!("Function '{}' is already defined", name));
        }
        self.concrete_funcs.insert(name, func);
        Ok(())
    }

    pub fn register_generic_func(&mut self, func: FunctionDef) -> Result<(), String> {
        let name = func.name.clone();
        if self.function_exists(&name) {
            return Err(format!("Generic function '{}' is already defined", name));
        }
        self.generic_func_templates.insert(name, func);
        Ok(())
    }

    pub fn register_concrete_struct(&mut self, struct_def: StructDef) -> Result<(), String> {
        let name = struct_def.name.clone();
        if self.struct_exists(&name) {
            return Err(format!("Struct '{}' is already defined", name));
        }
        self.concrete_structs.insert(name, struct_def);
        Ok(())
    }

    pub fn register_generic_struct(&mut self, struct_def: StructDef) -> Result<(), String> {
        let name = struct_def.name.clone();
        if self.struct_exists(&name) {
            return Err(format!("Generic struct '{}' is already defined", name));
        }
        self.generic_struct_templates.insert(name, struct_def);
        Ok(())
    }

    pub fn register_type_alias(&mut self, alias: TypeAlias) -> Result<(), String> {
        let name = alias.name.clone();
        if self.type_aliases.contains_key(&name) {
            return Err(format!("Type alias '{}' is already defined", name));
        }
        self.type_aliases.insert(name, alias.ty);
        Ok(())
    }

    pub fn register_static(&mut self, static_def: StaticDef) -> Result<(), String> {
        let name = static_def.name.clone();
        if self.statics.contains_key(&name) {
            return Err(format!("Static variable '{}' is already defined", name));
        }
        self.statics.insert(name, static_def);
        Ok(())
    }

    pub fn set_current_function(&mut self, func_name: String) {
        self.current_func_name = Some(func_name.clone());
        self.scope_path.clear();
        self.visit_counts.clear();
        self.visit_counts.push(0);

        self.function_scopes
            .entry(func_name)
            .or_insert_with(ScopeNode::default);
    }

    pub fn enter_scope(&mut self) {
        let func_name = self
            .current_func_name
            .as_ref()
            .expect("Not inside a function!");

        let root = self.function_scopes.get_mut(func_name).unwrap();
        let mut current_node = root;
        for &index in &self.scope_path {
            current_node = &mut current_node.children[index];
        }

        let child_index = *self.visit_counts.last().unwrap();

        if child_index >= current_node.children.len() {
            current_node.children.push(ScopeNode::default());
        }

        self.scope_path.push(child_index);
        self.visit_counts.push(0);
    }

    pub fn exit_scope(&mut self) {
        self.scope_path.pop();
        self.visit_counts.pop();

        if let Some(last) = self.visit_counts.last_mut() {
            *last += 1;
        }
    }

    pub fn define_symbol(&mut self, name: String, ty: Type) -> Result<(), String> {
        let func_name = self
            .current_func_name
            .as_ref()
            .expect("Not inside a function!");

        let root = self.function_scopes.get_mut(func_name).unwrap();
        let mut current_node = root;
        for &index in &self.scope_path {
            current_node = &mut current_node.children[index];
        }

        if current_node.symbols.contains_key(&name) {
            return Err(format!("Variable '{}' already defined in this scope", name));
        }

        current_node.symbols.insert(name, ty);
        Ok(())
    }

    pub fn resolve_symbol(&self, name: &str) -> Option<&Type> {
        let func_name = self.current_func_name.as_ref()?;
        let root = self.function_scopes.get(func_name)?;

        let mut found_type = root.symbols.get(name);

        let mut current_node = root;
        for &index in &self.scope_path {
            current_node = &current_node.children[index];
            if let Some(ty) = current_node.symbols.get(name) {
                found_type = Some(ty);
            }
        }

        found_type
    }

    pub fn register_generic_func_request(&mut self, name: String, concrete_types: Vec<Type>) {
        let key = (name.clone(), concrete_types.clone());
        if !self.observed_generic_funcs.contains(&key) {
            self.observed_generic_funcs.insert(key);
            self.pending_func_instantiations
                .push((name, concrete_types));
        }
    }

    pub fn register_generic_struct_request(&mut self, name: String, concrete_types: Vec<Type>) {
        let key = (name.clone(), concrete_types.clone());
        if !self.observed_generic_structs.contains(&key) {
            self.observed_generic_structs.insert(key);
            self.pending_struct_instantiations
                .push((name, concrete_types));
        }
    }

    pub fn has_pending_instantiations(&self) -> bool {
        !self.pending_struct_instantiations.is_empty()
            || !self.pending_func_instantiations.is_empty()
    }

    pub fn pop_pending_struct_request(&mut self) -> Option<(String, Vec<Type>)> {
        self.pending_struct_instantiations.pop()
    }

    pub fn pop_pending_func_request(&mut self) -> Option<(String, Vec<Type>)> {
        self.pending_func_instantiations.pop()
    }

    pub fn get_generic_func_template(&self, name: &str) -> Option<&FunctionDef> {
        self.generic_func_templates.get(name)
    }

    pub fn get_generic_struct_template(&self, name: &str) -> Option<&StructDef> {
        self.generic_struct_templates.get(name)
    }
}
