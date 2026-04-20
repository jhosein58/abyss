use crate::ir::{
    IrBinaryOp, IrExpr, IrExprKind, IrFunction, IrLit, IrProgram, IrStmt, IrType, IrUnaryOp,
};
use abyss_diagnostics::Span;

pub struct Ir;

impl Ir {
    #[inline]
    fn expr(kind: IrExprKind, ty: IrType) -> IrExpr {
        IrExpr {
            kind,
            ty,
            span: Span {
                file_id: 0,
                start: 0,
                end: 0,
            },
        }
    }

    pub fn program(stmts: Vec<IrStmt>) -> IrProgram {
        IrProgram {
            functions: vec![IrFunction {
                name: "main".into(),
                params: vec![],
                return_ty: IrType::Unit,
                body: Some(stmts),
            }],
            globals: vec![],
        }
    }

    pub fn stmt_expr(expr: IrExpr) -> IrStmt {
        IrStmt::Expr(expr)
    }

    pub fn var_dec(name: impl Into<String>, init: IrExpr) -> IrStmt {
        IrStmt::VarDec {
            name: name.into(),
            ty: init.ty.clone(),
            init: Some(init),
        }
    }

    pub fn assign(target: impl Into<String>, val: IrExpr) -> IrStmt {
        IrStmt::Assign {
            target: target.into(),
            val,
        }
    }

    pub fn if_stmt(cond: IrExpr, then_body: Vec<IrStmt>, else_body: Vec<IrStmt>) -> IrStmt {
        IrStmt::If(cond, then_body, else_body)
    }

    pub fn while_stmt(cond: IrExpr, body: Vec<IrStmt>) -> IrStmt {
        IrStmt::While { cond, body }
    }

    pub fn int(val: i64) -> IrExpr {
        Self::expr(IrExprKind::Lit(IrLit::Int(val)), IrType::I64)
    }

    pub fn float(val: f64) -> IrExpr {
        Self::expr(IrExprKind::Lit(IrLit::Float(val)), IrType::F64)
    }

    pub fn bool(val: bool) -> IrExpr {
        Self::expr(IrExprKind::Lit(IrLit::Bool(val)), IrType::Bool)
    }

    pub fn var(name: impl Into<String>) -> IrExpr {
        Self::expr(IrExprKind::VarRef(name.into()), IrType::Unit)
    }

    pub fn call(func_name: impl Into<String>, args: Vec<IrExpr>) -> IrExpr {
        Self::expr(
            IrExprKind::Call {
                func_name: func_name.into(),
                args,
            },
            IrType::Unit,
        )
    }

    #[inline]
    pub fn binary(left: IrExpr, op: IrBinaryOp, right: IrExpr, ty: IrType) -> IrExpr {
        Self::expr(IrExprKind::Binary(Box::new(left), op, Box::new(right)), ty)
    }

    pub fn add(l: IrExpr, r: IrExpr) -> IrExpr {
        Self::binary(l, IrBinaryOp::Add, r, IrType::I64)
    }
    pub fn sub(l: IrExpr, r: IrExpr) -> IrExpr {
        Self::binary(l, IrBinaryOp::Sub, r, IrType::I64)
    }
    pub fn mul(l: IrExpr, r: IrExpr) -> IrExpr {
        Self::binary(l, IrBinaryOp::Mul, r, IrType::I64)
    }
    pub fn div(l: IrExpr, r: IrExpr) -> IrExpr {
        Self::binary(l, IrBinaryOp::Div, r, IrType::I64)
    }

    pub fn eq(l: IrExpr, r: IrExpr) -> IrExpr {
        Self::binary(l, IrBinaryOp::Eq, r, IrType::Bool)
    }
    pub fn neq(l: IrExpr, r: IrExpr) -> IrExpr {
        Self::binary(l, IrBinaryOp::Neq, r, IrType::Bool)
    }
    pub fn lt(l: IrExpr, r: IrExpr) -> IrExpr {
        Self::binary(l, IrBinaryOp::Lt, r, IrType::Bool)
    }
    pub fn gt(l: IrExpr, r: IrExpr) -> IrExpr {
        Self::binary(l, IrBinaryOp::Gt, r, IrType::Bool)
    }

    #[inline]
    pub fn unary(op: IrUnaryOp, expr: IrExpr, ty: IrType) -> IrExpr {
        Self::expr(IrExprKind::Unary(op, Box::new(expr)), ty)
    }

    pub fn neg(e: IrExpr) -> IrExpr {
        Self::unary(IrUnaryOp::Neg, e, IrType::I64)
    }
    pub fn not(e: IrExpr) -> IrExpr {
        Self::unary(IrUnaryOp::Not, e, IrType::Bool)
    }
}

impl IrExpr {
    pub fn with_span(mut self, file_id: u16, start: usize, end: usize) -> Self {
        self.span = Span {
            file_id,
            start: start as u32,
            end: end as u32,
        };
        self
    }

    pub fn with_ty(mut self, ty: IrType) -> Self {
        self.ty = ty;
        self
    }
}
