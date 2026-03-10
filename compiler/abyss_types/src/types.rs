use crate::tast::TypedExpr;

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
    Ptr(Box<Type>),
    Signature(Vec<Type>, Box<Type>, bool), // args, return, is_native
    Array(Box<Type>, Box<TypedExpr>),
    Metatype,
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
            Type::Ptr(ref inner) => format!("&{}", inner.name()),
            Type::Signature(ref args, ref ret, _) => {
                let arg_names: Vec<String> = args.iter().map(|arg_type| arg_type.name()).collect();

                let args_str = arg_names.join(", ");

                let ret_name = ret.name();

                format!("fn({}): {}", args_str, ret_name)
            }

            Type::Metatype => format!("type"),

            Type::Array(ref ty, ref len) => format!("[{}; {:?}]", ty.name(), len.kind),
        }
    }

    pub fn to_id(&self) -> i64 {
        match self {
            Type::I32 => 1,
            Type::F32 => 2,
            Type::Bool => 3,
            Type::Str => 4,
            Type::Unit => 5,
            Type::Metatype => 6,
            _ => 0,
        }
    }

    pub fn from_id(id: i64) -> Type {
        match id {
            1 => Type::I32,
            2 => Type::F32,
            3 => Type::Bool,
            4 => Type::Str,
            5 => Type::Unit,
            6 => Type::Metatype,
            _ => Type::Error,
        }
    }
}
