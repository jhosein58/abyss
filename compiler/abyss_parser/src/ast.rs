use std::hash::{Hash, Hasher};

pub type Path = Vec<String>;

#[derive(Debug, Clone, PartialEq, Default, Eq, Hash)]
pub struct Span {
    pub line: u32,
    pub col: u32,
    pub file_id: u16,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Expr {
    pub kind: ExprKind,
    pub span: Span,
    pub id: u32,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ExprKind {
    Mod(String, Option<Box<Expr>>, bool), // Mod(Name, Body?, is_pub)
    Use(Path, bool),                      // Use(Module, is_pub)
    StructDef(Box<StructDef>),            // define struct
    TraitDef(Box<TraitDef>),              // define trait
    TypeDef(Box<TypeAlias>),              // define type
    FunctionDef(Box<FunctionDef>),        // define function

    VarDecl(Pattern, Type, Option<Box<Expr>>),
    Const(Pattern, Type, Box<Expr>),

    Ret(Option<Box<Expr>>),
    Break,                                       // out
    Continue,                                    // next
    Block(Vec<Expr>, Type),                      // Block(statements, return_type)
    If(Box<Expr>, Box<Expr>, Option<Box<Expr>>), // If(condition, then, else)
    For(Pattern, Box<Expr>, Box<Expr>),          // For(pattern, range, body)
    Range {
        start: Option<Box<Expr>>,
        end: Option<Box<Expr>>,
        step: Option<Box<Expr>>,
        inclusive: bool,
    },
    ForEach(Pattern, Box<Expr>, Box<Expr>), // ForEach(pattern, collection, body)
    While(Box<Expr>, Box<Expr>),            // While(condition, body)
    Defer(Box<Expr>),                       // Defer(expression)

    // ---------------------
    Lit(Lit),
    ArrayInit(Vec<Expr>),
    Ident(Path),
    Binary(Box<Expr>, BinaryOp, Box<Expr>),
    Unary(UnaryOp, Box<Expr>),
    Call(Box<Expr>, Vec<Expr>, Vec<Type>), // (callee)(args, generics)
    Index(Box<Expr>, Box<Expr>),
    Deref(Box<Expr>),
    AddrOf(Box<Expr>),
    Cast(Box<Expr>, Type),
    Is(Box<Expr>, Type),
    Member(Box<Expr>, String),
    StructInit(Path, Vec<(String, Expr)>, Vec<Type>),
    MethodCall(Box<Expr>, String, Vec<Expr>, Vec<Type>),
    SizeOf(Type),
    Match(Box<Expr>, Vec<(Pattern, Expr)>),
    Lambda(Box<FunctionDef>),
    Tuple(Vec<Expr>),
    Then(Box<Expr>, Box<Expr>), // Then(first, second)
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    U8,
    U16,
    U32,
    U64,
    Usize,
    I8,
    I16,
    I32,
    I64,
    Isize,
    F32,
    F64,
    Char,
    Bool,
    Unit,
    Pointer(Box<Type>),
    Const(Box<Type>),
    Array(Box<Type>, usize),
    Struct(Path, Vec<Type>),
    Trait(Path),
    TypeOf(Box<Expr>),
    Generic(String),
    Function(Vec<Type>, Box<Type>), // Function(args, return_type)
    Union(Vec<Type>),
    Infer,
    Tuple(Vec<Type>),
}

impl Type {
    pub fn get_name(&self) -> String {
        match self {
            Type::U8 => "u8".to_string(),
            Type::U16 => "u16".to_string(),
            Type::U32 => "u32".to_string(),
            Type::U64 => "u64".to_string(),
            Type::Usize => "usize".to_string(),
            Type::I8 => "i8".to_string(),
            Type::I16 => "i16".to_string(),
            Type::I32 => "i32".to_string(),
            Type::I64 => "i64".to_string(),
            Type::Isize => "isize".to_string(),
            Type::F32 => "f32".to_string(),
            Type::F64 => "f64".to_string(),
            Type::Char => "char".to_string(),
            Type::Bool => "bool".to_string(),
            Type::Unit => "unit".to_string(),
            Type::Pointer(ty) => format!("ptr_{}", ty.get_name()),
            Type::Const(ty) => format!("const_{}", ty.get_name()),
            Type::Array(ty, size) => format!("Arr_{}_{}", ty.get_name(), size),
            Type::Struct(path, _) => format!("struct_{}", path.join("_")),
            Type::Generic(name) => name.clone(),

            _ => panic!("Type has no Name"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct OrderedFloat(pub f64);

impl PartialEq for OrderedFloat {
    fn eq(&self, other: &Self) -> bool {
        self.0.to_bits() == other.0.to_bits()
    }
}

impl Eq for OrderedFloat {}

impl Hash for OrderedFloat {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.0.to_bits().hash(state);
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Lit {
    Int(i64),
    Float(OrderedFloat),
    Bool(bool),
    Str(String),
    Char(char),
    Null,
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Hash)]
pub enum BinaryOp {
    Assign,       // =
    AssignAdd,    // +=
    AssignSub,    // -=
    AssignMul,    // *=
    AssignDiv,    // /=
    AssignMod,    // %=
    AssignBitAnd, // &=
    AssignBitOr,  // |=
    AssignBitXor, // ^=
    AssignShl,    // <<=
    AssignShr,    // >>=
    Add,          // +
    Sub,          // -
    Mul,          // *
    Div,          // /
    Mod,          // %
    Eq,           // ==
    Neq,          // !=
    Lt,           // <
    Gt,           // >
    Lte,          // <=
    Gte,          // >=
    And,          // and
    Or,           // or
    BitAnd,       // &
    BitOr,        // |
    BitXor,       // ^
    Shl,          // <<
    Shr,          // >>
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum UnaryOp {
    Neg,    // -x
    Not,    // not x
    BitNot, // ~x
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Pattern {
    Ident(String),

    Lit(Lit),

    Tuple(Vec<Pattern>),

    StructDestruct(Path, Vec<(String, Pattern)>),

    VariantDestruct(Path, Vec<Pattern>),

    Wildcard, // let _ =
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Attribute {
    pub name: String,
    pub args: Vec<Expr>,
    pub span: Span,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FunctionDef {
    pub attributes: Vec<Attribute>,
    pub is_pub: bool,
    pub name: String,
    pub generics: Vec<String>,
    pub params: Vec<(String, Type)>,
    pub return_type: Type,
    pub body: FunctionBody,
    pub is_variadic: bool,
    pub external_name: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum FunctionBody {
    UserDefined(Expr),
    Extern,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StructDef {
    pub attributes: Vec<Attribute>,
    pub is_pub: bool,
    pub name: String,
    pub generics: Vec<String>,
    pub fields: Vec<(String, Type)>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TypeAlias {
    pub is_pub: bool,
    pub name: String,
    pub ty: Type,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TraitDef {
    pub attributes: Vec<Attribute>,
    pub is_pub: bool,
    pub name: String,
    pub generics: Vec<String>,
    pub methods: Vec<TraitMethod>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TraitMethod {
    pub signature: FunctionDef,
    pub has_default: bool,
}

#[derive(Debug, Clone)]
pub struct Program {
    pub body: Expr,
}
