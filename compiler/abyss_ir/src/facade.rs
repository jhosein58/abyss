use crate::ir::{
    IrBinaryOp, IrExpr, IrExprKind, IrFunction, IrLit, IrProgram, IrStmt, IrType, IrUnaryOp,
};
use abyss_diagnostics::Span;

pub struct Ir;

impl Ir {
    #[inline]
    pub fn expr(kind: IrExprKind, ty: IrType) -> IrExpr {
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

    pub fn value_ty() -> IrType {
        IrType::Struct(vec![
            IrType::I8,
            IrType::Union(vec![
                IrType::Unit,
                IrType::F64,
                IrType::Ptr(Box::new(IrType::Unit)),
            ]),
        ])
    }

    pub fn get_type(val: IrExpr) -> IrExpr {
        Self::expr(
            IrExprKind::FieldAccess {
                base: Box::new(val),
                index: 0,
            },
            IrType::I8,
        )
    }

    pub fn generate_dynamic_prelude() -> Vec<IrFunction> {
        let val_ty = Self::value_ty();
        let union_ty = match &val_ty {
            IrType::Struct(fields) => fields[1].clone(),
            _ => unreachable!(),
        };

        // rt_make_number(f64)
        let rt_make_num = IrFunction {
            name: "rt_make_number".into(),
            params: vec![("v".into(), IrType::F64)],
            return_ty: val_ty.clone(),
            body: Some(vec![
                Self::var_dec(
                    "res",
                    Self::expr(
                        IrExprKind::StructInit(vec![
                            Self::expr(IrExprKind::Lit(IrLit::Int(1)), IrType::I8),
                            Self::expr(IrExprKind::Lit(IrLit::Int(0)), IrType::Unit),
                        ]),
                        val_ty.clone(),
                    ),
                ),
                IrStmt::WriteUnion {
                    base: Self::expr(
                        IrExprKind::GetFieldPtr {
                            base: Box::new(Self::var("res")),
                            index: 1,
                        },
                        IrType::Ptr(Box::new(IrType::Unit)),
                    ),
                    index: 1, // F64 field
                    val: Self::var("v"),
                },
                IrStmt::Return(Some(Self::var("res"))),
            ]),
        };

        // rt_make_nil()
        let rt_make_nil = IrFunction {
            name: "rt_make_nil".into(),
            params: vec![],
            return_ty: val_ty.clone(),
            body: Some(vec![
                Self::var_dec(
                    "res",
                    Self::expr(
                        IrExprKind::StructInit(vec![
                            Self::expr(IrExprKind::Lit(IrLit::Int(0)), IrType::I8),
                            Self::expr(IrExprKind::Lit(IrLit::Int(0)), IrType::Unit),
                        ]),
                        val_ty.clone(),
                    ),
                ),
                IrStmt::Return(Some(Self::var("res"))),
            ]),
        };

        // rt_make_func(Ptr)
        let rt_make_func = IrFunction {
            name: "rt_make_func".into(),
            params: vec![("ptr".into(), IrType::Ptr(Box::new(IrType::Unit)))],
            return_ty: val_ty.clone(),
            body: Some(vec![
                Self::var_dec(
                    "res",
                    Self::expr(
                        IrExprKind::StructInit(vec![
                            Self::expr(IrExprKind::Lit(IrLit::Int(2)), IrType::I8),
                            Self::expr(IrExprKind::Lit(IrLit::Int(0)), IrType::Unit),
                        ]),
                        val_ty.clone(),
                    ),
                ),
                IrStmt::WriteUnion {
                    base: Self::expr(
                        IrExprKind::GetFieldPtr {
                            base: Box::new(Self::var("res")),
                            index: 1,
                        },
                        IrType::Ptr(Box::new(IrType::Unit)),
                    ),
                    index: 2, // FuncPtr field
                    val: Self::var("ptr"),
                },
                IrStmt::Return(Some(Self::var("res"))),
            ]),
        };

        let make_math_op = |name: &str, op: IrBinaryOp| -> IrFunction {
            IrFunction {
                name: name.into(),
                params: vec![("a".into(), val_ty.clone()), ("b".into(), val_ty.clone())],
                return_ty: val_ty.clone(),
                body: Some(vec![
                    Self::var_dec(
                        "a_val",
                        Self::expr(
                            IrExprKind::FieldAccess {
                                base: Box::new(Self::expr(
                                    IrExprKind::GetFieldPtr {
                                        base: Box::new(Self::var("a")),
                                        index: 1,
                                    },
                                    IrType::Ptr(Box::new(union_ty.clone())),
                                )),
                                index: 1,
                            },
                            IrType::F64,
                        ),
                    ),
                    Self::var_dec(
                        "b_val",
                        Self::expr(
                            IrExprKind::FieldAccess {
                                base: Box::new(Self::expr(
                                    IrExprKind::GetFieldPtr {
                                        base: Box::new(Self::var("b")),
                                        index: 1,
                                    },
                                    IrType::Ptr(Box::new(union_ty.clone())),
                                )),
                                index: 1,
                            },
                            IrType::F64,
                        ),
                    ),
                    IrStmt::Return(Some(Self::call(
                        "rt_make_number",
                        vec![Self::expr(
                            IrExprKind::Binary(
                                Box::new(Self::var("a_val")),
                                op,
                                Box::new(Self::var("b_val")),
                            ),
                            IrType::F64,
                        )],
                    ))),
                ]),
            }
        };

        let rt_print = IrFunction {
            name: "print".into(),
            params: vec![("v".into(), val_ty.clone())],
            return_ty: val_ty.clone(),
            body: Some(vec![
                Self::var_dec(
                    "num_val",
                    Self::expr(
                        IrExprKind::FieldAccess {
                            base: Box::new(Self::expr(
                                IrExprKind::GetFieldPtr {
                                    base: Box::new(Self::var("v")),
                                    index: 1,
                                },
                                IrType::Ptr(Box::new(union_ty.clone())),
                            )),
                            index: 1,
                        },
                        IrType::F64,
                    ),
                ),
                IrStmt::Expr(Self::call("sys_print_num", vec![Self::var("num_val")])),
                IrStmt::Return(Some(Self::call("rt_make_nil", vec![]))),
            ]),
        };

        vec![
            rt_make_num,
            rt_make_nil,
            rt_make_func,
            rt_print,
            make_math_op("rt_add", IrBinaryOp::Add),
            make_math_op("rt_sub", IrBinaryOp::Sub),
            make_math_op("rt_mul", IrBinaryOp::Mul),
            make_math_op("rt_div", IrBinaryOp::Div),
        ]
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
