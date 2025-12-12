#[derive(Debug)]
pub enum Stmt {
    Let(String, Type, Option<Expr>),
    Assign(Expr, Expr),
    Ret(Expr),
    Break,    // out
    Continue, // next
    If(Expr, Vec<Stmt>, Option<Vec<Stmt>>),
    While(Expr, Vec<Stmt>),
    Expr(Expr),
}

#[derive(Debug, Clone)]
pub enum Expr {
    Lit(Lit),
    Ident(String),
    Binary(Box<Expr>, BinaryOp, Box<Expr>),
    Unary(UnaryOp, Box<Expr>),
    Call(String, Vec<Expr>),
    Index(Box<Expr>, Box<Expr>),
    Deref(Box<Expr>),
    AddrOf(Box<Expr>),
    Cast(Box<Expr>, Type),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    U8,
    I64,
    F64,
    Bool,
    Void,
    Pointer(Box<Type>),
    Array(Box<Type>, usize),
}

#[derive(Debug, Clone)]
pub enum Lit {
    Int(i64),
    Float(f64),
    Bool(bool),
    Str(String),
    Array(Vec<Expr>),
}

#[derive(Debug, Clone, Copy)]
pub enum BinaryOp {
    Add, // +
    Sub, // -
    Mul, // *
    Div, // /
    Mod, // %
    Eq,  // ==
    Neq, // !=
    Lt,  // <
    Gt,  // >
    Lte, // <=
    Gte, // >=
    And, // and
    Or,  // or

    BitAnd, // &
    BitOr,  // |
    BitXor, // ^
    Shl,    // <<
    Shr,    // >>
}

#[derive(Debug, Clone, Copy)]
pub enum UnaryOp {
    Neg,    // -x
    Not,    // not x
    BitNot, // ~x
}

#[derive(Debug)]
pub struct Function {
    pub name: String,
    pub params: Vec<(String, Type)>,
    pub return_type: Type,
    pub body: Option<Vec<Stmt>>,
}

#[derive(Debug)]
pub struct Program {
    pub functions: Vec<Function>,
}
