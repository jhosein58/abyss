use crate::ir::{IrBinaryOp, IrLit, IrType, IrUnaryOp};

#[derive(Debug, Clone)]
pub struct MirProgram {
    pub functions: Vec<MirFunction>,
    pub globals: Vec<(String, IrType, MirExpr)>,
}

#[derive(Debug, Clone)]
pub struct MirFunction {
    pub name: String,
    pub params: Vec<(String, IrType)>,
    pub return_ty: IrType,
    pub body: MirExpr,
}

#[derive(Debug, Clone)]
pub struct MirExpr {
    pub kind: MirExprKind,
    pub ty: IrType,
}

impl MirExpr {
    pub fn new(kind: MirExprKind, ty: IrType) -> Self {
        Self { kind, ty }
    }
}

#[derive(Debug, Clone)]
pub enum MirExprKind {
    Lit(IrLit),
    VarRef(String),

    Unary(IrUnaryOp, Box<MirExpr>),
    Binary(Box<MirExpr>, IrBinaryOp, Box<MirExpr>),
    Cast(Box<MirExpr>, IrType),

    Block(Vec<MirExpr>),
    If {
        cond: Box<MirExpr>,
        then_b: Box<MirExpr>,
        else_b: Option<Box<MirExpr>>,
    },
    While {
        cond: Box<MirExpr>,
        body: Box<MirExpr>,
    },
    Break,
    Continue,
    Return(Option<Box<MirExpr>>),

    Call {
        func_name: String,
        args: Vec<MirExpr>,
    },

    VarDec {
        name: String,
        is_mut: bool,
        init: Box<MirExpr>,
    },

    Assign {
        target: Box<MirExpr>,
        val: Box<MirExpr>,
    },

    ArrayInit(Vec<MirExpr>),
    ArrayRepeat {
        val: Box<MirExpr>,
        count: usize,
    },
    Index {
        base: Box<MirExpr>,
        index: Box<MirExpr>,
    },
    StructInit(Vec<MirExpr>),
    FieldAccess {
        base: Box<MirExpr>,
        index: usize,
    },
}
