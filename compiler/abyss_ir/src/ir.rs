use abyss_diagnostics::Span;

#[derive(Debug, Clone)]
pub enum IrType {
    I32,
    F32,
    Bool,
    Unit,
    Ptr(Box<IrType>),
}

#[derive(Debug, Clone)]
pub struct IrProgram {
    pub functions: Vec<IrFunction>,
}

#[derive(Debug, Clone)]
pub struct IrFunction {
    pub name: String,
    pub params: Vec<(String, IrType)>,
    pub return_ty: IrType,
    pub body: Vec<IrStmt>,
}

#[derive(Debug, Clone)]
pub enum IrStmt {
    VarDec {
        name: String,
        ty: IrType,
        init: Option<IrExpr>,
    },

    Assign {
        target: String,
        val: IrExpr,
    },

    Expr(IrExpr),

    Return(Option<IrExpr>),

    If(IrExpr, Vec<IrStmt>, Vec<IrStmt>), // cond, then, else
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

    Call {
        func_name: String,
        args: Vec<IrExpr>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IrBinaryOp {
    Add,
    Sub,
    Mul,
    Div,

    Eq,  // ==
    Neq, // !=
    Lt,  // <
    Le,  // <=
    Gt,  // >
    Ge,  // >=

    And, // and
    Or,  // or
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IrUnaryOp {
    Neg,   // -x
    Not,   // not x
    Ref,   // &x
    Deref, // *x
}

#[derive(Debug, Clone, PartialEq)]
pub enum IrLit {
    Int(i64),
    Float(f64),
    Bool(bool),
}
