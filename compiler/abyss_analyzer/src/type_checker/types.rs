#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    I32,
    F32,
    Bool,
    Str,
    Cstr,
    Char,
    Unit,
    Infer,
    Error,
}

impl Type {
    pub fn name(&self) -> String {
        match *self {
            Type::I32 => "i32",
            Type::F32 => "f32",
            Type::Bool => "bool",
            Type::Str => "str",
            Type::Cstr => "c_str",
            Type::Char => "char",
            Type::Unit => "Unit",
            Type::Error => "Err",
            Type::Infer => "Infer",
        }
        .to_string()
    }
}
