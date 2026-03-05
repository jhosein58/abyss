use crate::type_checker::tast::TypedExpr;

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
    Signature(Vec<Type>, Box<Type>),
    Array(Box<Type>, Box<TypedExpr>),
    Error,
}

impl Type {
    pub fn name(&self) -> String {
        match *self {
            Type::I32 => "i32".to_string(),
            Type::F32 => "f32".to_string(),
            Type::Bool => "bool".to_string(),
            Type::Str => "str".to_string(),
            Type::Cstr => "c_str".to_string(),
            Type::Char => "char".to_string(),
            Type::Unit => "Unit".to_string(),
            Type::Error => "Err".to_string(),
            Type::Infer => "Infer".to_string(),
            Type::Signature(ref args, ref ret) => {
                let arg_names: Vec<String> = args.iter().map(|arg_type| arg_type.name()).collect();

                let args_str = arg_names.join(", ");

                let ret_name = ret.name();

                format!("fn({}): {}", args_str, ret_name)
            }

            Type::Array(ref ty, ref len) => format!("[{}; {:?}]", ty.name(), len.kind),
        }
    }
}
