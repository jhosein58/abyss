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

    pub fn get_type_tag(&mut self, ty: &Type) -> i64 {
        let (name, id) = match ty {
            Type::U8 => ("TYPE_TAG_U8".to_string(), 1),
            Type::U16 => ("TYPE_TAG_U16".to_string(), 2),
            Type::U32 => ("TYPE_TAG_U32".to_string(), 3),
            Type::U64 => ("TYPE_TAG_U64".to_string(), 4),
            Type::Usize => ("TYPE_TAG_USIZE".to_string(), 5),
            Type::I8 => ("TYPE_TAG_I8".to_string(), 6),
            Type::I16 => ("TYPE_TAG_I16".to_string(), 7),
            Type::I32 => ("TYPE_TAG_I32".to_string(), 8),
            Type::I64 => ("TYPE_TAG_I64".to_string(), 9),
            Type::Isize => ("TYPE_TAG_ISIZE".to_string(), 10),
            Type::F32 => ("TYPE_TAG_F32".to_string(), 11),
            Type::F64 => ("TYPE_TAG_F64".to_string(), 12),
            Type::Bool => ("TYPE_TAG_BOOL".to_string(), 13),
            Type::Char => ("TYPE_TAG_CHAR".to_string(), 14),
            Type::Array(inner, _) if **inner == Type::U8 => ("TYPE_TAG_U8".to_string(), 1),
            _ => {
                let s = format!("{:?}", ty);
                let mut hash: i64 = 0;
                for c in s.bytes() {
                    hash = hash.wrapping_add(c as i64);
                }
                (format!("TYPE_TAG_{}", hash), hash)
            }
        };
        self.used_type_tags.insert(name, id);
        id
    }
}
