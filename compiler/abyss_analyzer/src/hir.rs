use abyss_parser::ast::{FunctionDef, StaticDef, StructDef, UnionDef};

#[derive(Debug, Clone)]
pub struct FlatProgram {
    pub functions: Vec<FunctionDef>,
    pub structs: Vec<StructDef>,
    pub unions: Vec<UnionDef>,
    pub statics: Vec<StaticDef>,
    pub union_struct_defs: Vec<StructDef>,
}

impl FlatProgram {
    pub fn new() -> Self {
        Self {
            functions: vec![],
            structs: vec![],
            unions: vec![],
            statics: vec![],
            union_struct_defs: vec![],
        }
    }
}
