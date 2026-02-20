use abyss_parser::ast::{FunctionDef, StaticDef, StructDef, TypeAlias, UnionDef};

#[derive(Debug, Clone, Default)]
pub struct FlatProgram {
    pub functions: Vec<FunctionDef>,
    pub structs: Vec<StructDef>,
    pub unions: Vec<UnionDef>,
    pub statics: Vec<StaticDef>,
    pub union_struct_defs: Vec<StructDef>,
    pub type_aliases: Vec<TypeAlias>,
}

impl FlatProgram {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
}
