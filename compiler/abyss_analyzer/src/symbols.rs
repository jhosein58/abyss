use crate::lir::LirType;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct FunctionInfo {
    pub params: Vec<LirType>,
    pub ret_ty: LirType,
}

#[derive(Debug, Clone)]
pub struct VarInfo {
    pub ty: LirType,
    pub is_global: bool,
}

#[derive(Debug, Clone)]
pub struct StructInfo {
    pub fields: Vec<(String, LirType)>,
}

impl StructInfo {
    pub fn get_field_type(&self, field_name: &str) -> Option<LirType> {
        self.fields
            .iter()
            .find(|(name, _)| name == field_name)
            .map(|(_, ty)| ty.clone())
    }
}

pub struct Context {
    pub functions: HashMap<String, FunctionInfo>,
    pub structs: HashMap<String, StructInfo>,

    pub vars: Vec<HashMap<String, VarInfo>>,

    pub globals: HashMap<String, VarInfo>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            functions: HashMap::new(),
            structs: HashMap::new(),
            vars: vec![HashMap::new()],
            globals: HashMap::new(),
        }
    }

    pub fn register_function(&mut self, name: String, params: Vec<LirType>, ret_ty: LirType) {
        self.functions.insert(name, FunctionInfo { params, ret_ty });
    }

    pub fn lookup_function(&self, name: &str) -> Option<&FunctionInfo> {
        self.functions.get(name)
    }

    pub fn register_struct(&mut self, name: String, fields: Vec<(String, LirType)>) {
        self.structs.insert(name, StructInfo { fields });
    }

    pub fn get_struct_field_type(&self, struct_name: &str, field_name: &str) -> Option<LirType> {
        self.structs.get(struct_name)?.get_field_type(field_name)
    }

    pub fn enter_scope(&mut self) {
        self.vars.push(HashMap::new());
    }

    pub fn exit_scope(&mut self) {
        self.vars.pop();
    }

    pub fn declare_var(&mut self, name: String, ty: LirType) {
        if let Some(scope) = self.vars.last_mut() {
            scope.insert(
                name,
                VarInfo {
                    ty,
                    is_global: false,
                },
            );
        }
    }

    pub fn declare_global(&mut self, name: String, ty: LirType) {
        self.globals.insert(
            name,
            VarInfo {
                ty,
                is_global: true,
            },
        );
    }

    pub fn lookup_var(&self, name: &str) -> Option<&VarInfo> {
        for scope in self.vars.iter().rev() {
            if let Some(info) = scope.get(name) {
                return Some(info);
            }
        }

        self.globals.get(name)
    }

    pub fn clear_vars(&mut self) {
        self.vars = vec![HashMap::new()];
    }
}
