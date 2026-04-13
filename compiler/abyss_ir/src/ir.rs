use abyss_diagnostics::Span;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum IrType {
    I32,
    F32,
    Bool,
    Unit,
    Ptr(Box<IrType>),
    Array(Box<IrType>, usize),
    Struct(Vec<IrType>),
}

#[derive(Debug, Clone)]
pub struct IrProgram {
    pub functions: Vec<IrFunction>,
    pub globals: Vec<(String, IrType, IrExpr)>,
}

#[derive(Debug, Clone)]
pub struct IrFunction {
    pub name: String,
    pub params: Vec<(String, IrType)>,
    pub return_ty: IrType,
    pub body: Option<Vec<IrStmt>>,
}

#[derive(Debug, Clone)]
pub enum IrStmt {
    VarDec {
        name: String,
        ty: IrType,
        init: Option<IrExpr>,
    },

    ConstDef {
        name: String,
        ty: IrType,
        value: IrExpr,
    },

    Assign {
        target: String,
        val: IrExpr,
    },

    WriteIndex {
        base: IrExpr,
        index: IrExpr,
        val: IrExpr,
    },

    WritePointer {
        ptr: IrExpr,
        val: IrExpr,
    },

    Expr(IrExpr),

    Return(Option<IrExpr>),

    If(IrExpr, Vec<IrStmt>, Vec<IrStmt>), // cond, then, else

    While {
        cond: IrExpr,
        body: Vec<IrStmt>,
    },

    Break,

    WriteField {
        base: IrExpr,
        index: usize,
        val: IrExpr,
    },
}

#[derive(Debug, Clone)]
pub struct IrExpr {
    pub kind: IrExprKind,
    pub ty: IrType,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum IrExprKind {
    Lit(IrLit),
    VarRef(String),

    Unary(IrUnaryOp, Box<IrExpr>),
    Binary(Box<IrExpr>, IrBinaryOp, Box<IrExpr>),

    Cast(Box<IrExpr>, IrType),

    Call {
        func_name: String,
        args: Vec<IrExpr>,
    },

    ArrayInit(Vec<IrExpr>), // [1, 2, 3]

    // [0; 10]
    ArrayRepeat {
        val: Box<IrExpr>,
        count: usize,
    },

    Index(Box<IrExpr>, Box<IrExpr>), // a[b]

    StructInit(Vec<IrExpr>),

    FieldAccess {
        base: Box<IrExpr>,
        index: usize,
    },

    GetIndexPtr {
        base: Box<IrExpr>,
        index: Box<IrExpr>,
    },

    GetFieldPtr {
        base: Box<IrExpr>,
        index: usize,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IrBinaryOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,

    Eq,  // ==
    Neq, // !=
    Lt,  // <
    Le,  // <=
    Gt,  // >
    Ge,  // >=

    And, // and
    Or,  // or

    BitAnd, // &
    BitOr,  // |
    BitXor, // ^
    Shl,    // <<
    Shr,    // >>
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IrUnaryOp {
    Neg,    // -x
    Not,    // not x
    Ref,    // &x
    Deref,  // *x
    BitNot, // ~x
}

#[derive(Debug, Clone, PartialEq)]
pub enum IrLit {
    Int(i64),
    Float(f64),
    Bool(bool),
}
