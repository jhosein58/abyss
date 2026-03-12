use crate::tast::TypedExpr;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    // Primitive types
    I1,
    I8,
    I16,
    I32,
    I64,

    U8,
    U16,
    U32,
    U64,

    F32,
    F64,

    Bool,
    Str,
    Cstr,
    Char,
    Unit,

    // Special types
    Never,
    Infer,
    Unknown,

    Ptr(Box<Type>),
    Signature(Vec<Type>, Box<Type>, bool), // args, return, is_native
    Array(Box<Type>, Box<TypedExpr>),      // [Type; Length]
    Struct(Vec<StructField>),              // [a: i32, str, c: bool]
    Union(Vec<Type>),                      // i32 | str
    Alias(String, Box<Type>),
    Metatype,
    Error,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StructField {
    pub name: String,
    pub ty: Type,
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
            Type::Struct(ref fields) => {
                let field_names: Vec<String> = fields
                    .iter()
                    .map(|f| format!("{}: {}", f.name, f.ty.name()))
                    .collect();
                format!("[{}]", field_names.join(", "))
            }
            Type::Union(ref types) => {
                let type_names: Vec<String> = types.iter().map(|t| t.name()).collect();
                type_names.join(" | ")
            }
            Type::Alias(ref name, _) => name.clone(),

            _ => panic!(),
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

    pub fn underlying_type(&self) -> Type {
        match self {
            Type::Alias(_, base_ty) => base_ty.underlying_type(),
            other => other.clone(),
        }
    }
    pub fn is_assignable_from(&self, target: &Type, source: &Type) -> bool {
        if target == source {
            return true;
        }

        match (target, source) {
            (Type::Alias(name1, _), Type::Alias(name2, _)) => name1 == name2,

            (Type::Alias(_, _), _) => false,

            (t, Type::Alias(_, inner_src)) => self.is_assignable_from(t, inner_src),

            _ => false,
        }
    }
}
