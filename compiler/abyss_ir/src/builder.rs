use abyss_analyzer::type_checker::{
    tast::{TypedExpr, TypedExprKind, TypedProgram},
    types::Type,
};
use abyss_parser::ast::{BinaryOp, Lit, UnaryOp};

use crate::ir::{
    IrBinaryOp, IrExpr, IrExprKind, IrFunction, IrLit, IrProgram, IrStmt, IrType, IrUnaryOp,
};
use abyss_diagnostics::Span;

pub struct IrBuilder {
    temp_counter: usize,
}

impl IrBuilder {
    pub fn new() -> Self {
        Self { temp_counter: 0 }
    }

    fn new_temp(&mut self) -> String {
        let name = format!("$tmp{}", self.temp_counter);
        self.temp_counter += 1;
        name
    }

    fn unit_expr(&self, span: Span) -> IrExpr {
        IrExpr {
            kind: IrExprKind::Lit(IrLit::Bool(false)),
            ty: IrType::Unit,
            span,
        }
    }

    // decl.rs

    pub fn build_program(&mut self, program: TypedProgram) -> IrProgram {
        let mut functions = Vec::new();

        for hoisted_func in program.hoisted_functions {
            if let Some(ir_func) = self.build_function(hoisted_func) {
                functions.push(ir_func);
            }
        }

        let mut main_body = Vec::new();
        if let TypedExprKind::Block(stmts) = program.body.kind {
            for stmt in stmts {
                main_body.extend(self.lower_stmt(stmt));
            }
        } else {
            main_body.extend(self.lower_stmt(program.body));
        }

        functions.push(IrFunction {
            name: "main".to_string(),
            params: vec![],
            return_ty: IrType::Unit,
            body: main_body,
        });

        IrProgram { functions }
    }

    fn build_function(&mut self, expr: TypedExpr) -> Option<IrFunction> {
        if let TypedExprKind::FunctionDef {
            name,
            args,
            ret_ty,
            body,
        } = expr.kind
        {
            let mut ir_params = Vec::new();

            for param_expr in args {
                if let TypedExprKind::VarDec(p_name, p_ty, _) = param_expr.kind {
                    ir_params.push((p_name, self.lower_type(&p_ty)));
                } else {
                    panic!("IR Builder: Function arguments must be VarDec.");
                }
            }

            self.temp_counter = 0;

            let mut ir_body = Vec::new();
            match body.kind {
                TypedExprKind::Block(stmts) => {
                    for stmt in stmts {
                        ir_body.extend(self.lower_stmt(stmt));
                    }
                }
                _ => {
                    ir_body.extend(self.lower_stmt(*body));
                }
            }

            return Some(IrFunction {
                name,
                params: ir_params,
                return_ty: self.lower_type(&ret_ty),
                body: ir_body,
            });
        }
        None
    }

    // stmt.rs

    fn lower_stmt(&mut self, expr: TypedExpr) -> Vec<IrStmt> {
        let mut generated_stmts = Vec::new();

        match expr.kind {
            TypedExprKind::VarDec(name, ty, init) => {
                let ir_init = if let Some(init_expr) = init {
                    let (init_stmts, init_val) = self.lower_expr(*init_expr);
                    generated_stmts.extend(init_stmts);
                    Some(init_val)
                } else {
                    None
                };

                generated_stmts.push(IrStmt::VarDec {
                    name,
                    ty: self.lower_type(&ty),
                    init: ir_init,
                });
            }

            TypedExprKind::Binary(left, BinaryOp::Assign, right) => {
                if let TypedExprKind::Ident(name) = left.kind {
                    let (right_stmts, right_val) = self.lower_expr(*right);
                    generated_stmts.extend(right_stmts);

                    generated_stmts.push(IrStmt::Assign {
                        target: name,
                        val: right_val,
                    });
                } else {
                    panic!("Complex assignments not supported yet.");
                }
            }

            TypedExprKind::Ret(val) => {
                let ir_val = if let Some(ret_expr) = val {
                    let (ret_stmts, ret_val) = self.lower_expr(*ret_expr);
                    generated_stmts.extend(ret_stmts);
                    Some(ret_val)
                } else {
                    None
                };
                generated_stmts.push(IrStmt::Return(ir_val));
            }

            _ => {
                let (expr_stmts, val) = self.lower_expr(expr);
                generated_stmts.extend(expr_stmts);
                generated_stmts.push(IrStmt::Expr(val));
            }
        }

        generated_stmts
    }

    // expr.rs

    fn lower_expr(&mut self, expr: TypedExpr) -> (Vec<IrStmt>, IrExpr) {
        let span = expr.span.clone();
        let ir_ty = self.lower_type(&expr.ty);
        let mut generated_stmts = Vec::new();

        let kind = match expr.kind {
            TypedExprKind::Lit(lit) => IrExprKind::Lit(self.lower_lit(lit)),

            TypedExprKind::Ident(name) | TypedExprKind::FuncRef(name) => IrExprKind::VarRef(name),

            TypedExprKind::Unary(op, inner_expr) => {
                let ir_op = match op {
                    UnaryOp::Neg => IrUnaryOp::Neg,
                    UnaryOp::Not => IrUnaryOp::Not,
                    UnaryOp::AddrOf => IrUnaryOp::Ref,
                    UnaryOp::Deref => IrUnaryOp::Deref,
                    _ => panic!("Unsupported unary op: {:?}", op),
                };

                let (inner_stmts, inner_val) = self.lower_expr(*inner_expr);
                generated_stmts.extend(inner_stmts);

                IrExprKind::Unary(ir_op, Box::new(inner_val))
            }

            TypedExprKind::Binary(left, op, right) => {
                let ir_op = match op {
                    // Arithmetic
                    BinaryOp::Add => IrBinaryOp::Add,
                    BinaryOp::Sub => IrBinaryOp::Sub,
                    BinaryOp::Mul => IrBinaryOp::Mul,
                    BinaryOp::Div => IrBinaryOp::Div,
                    // Comparison
                    BinaryOp::Eq => IrBinaryOp::Eq,
                    BinaryOp::Neq => IrBinaryOp::Neq,
                    BinaryOp::Lt => IrBinaryOp::Lt,
                    BinaryOp::Lte => IrBinaryOp::Le,
                    BinaryOp::Gt => IrBinaryOp::Gt,
                    BinaryOp::Gte => IrBinaryOp::Ge,
                    // Logical
                    BinaryOp::And => IrBinaryOp::And,
                    BinaryOp::Or => IrBinaryOp::Or,
                    _ => panic!("Unsupported binary op in IR Builder: {:?}", op),
                };

                let (left_stmts, left_val) = self.lower_expr(*left);
                let (right_stmts, right_val) = self.lower_expr(*right);

                generated_stmts.extend(left_stmts);
                generated_stmts.extend(right_stmts);

                IrExprKind::Binary(Box::new(left_val), ir_op, Box::new(right_val))
            }

            TypedExprKind::Call(func, args) => {
                let func_name = match func.kind {
                    TypedExprKind::Ident(name) | TypedExprKind::FuncRef(name) => name,
                    _ => panic!("Dynamic dispatch not supported."),
                };

                let mut ir_args = Vec::new();
                for arg in args {
                    let (arg_stmts, arg_val) = self.lower_expr(arg);
                    generated_stmts.extend(arg_stmts);
                    ir_args.push(arg_val);
                }

                IrExprKind::Call {
                    func_name,
                    args: ir_args,
                }
            }

            TypedExprKind::If(cond, then_branch, else_branch) => {
                let (cond_stmts, cond_val) = self.lower_expr(*cond);
                generated_stmts.extend(cond_stmts);

                let is_void = expr.ty == Type::Unit;

                if is_void {
                    let then_stmts = self.lower_stmt(*then_branch);
                    let else_stmts = if let Some(else_b) = else_branch {
                        self.lower_stmt(*else_b)
                    } else {
                        vec![]
                    };

                    generated_stmts.push(IrStmt::If(cond_val, then_stmts, else_stmts));

                    return (generated_stmts, self.unit_expr(span));
                } else {
                    let temp_var = self.new_temp();
                    generated_stmts.push(IrStmt::VarDec {
                        name: temp_var.clone(),
                        ty: ir_ty.clone(),
                        init: None,
                    });

                    let (mut then_stmts, then_val) = self.lower_expr(*then_branch);
                    then_stmts.push(IrStmt::Assign {
                        target: temp_var.clone(),
                        val: then_val,
                    });

                    let mut else_stmts = Vec::new();
                    if let Some(else_b) = else_branch {
                        let (e_stmts, e_val) = self.lower_expr(*else_b);
                        else_stmts.extend(e_stmts);
                        else_stmts.push(IrStmt::Assign {
                            target: temp_var.clone(),
                            val: e_val,
                        });
                    }

                    generated_stmts.push(IrStmt::If(cond_val, then_stmts, else_stmts));

                    return (
                        generated_stmts,
                        IrExpr {
                            kind: IrExprKind::VarRef(temp_var),
                            ty: ir_ty,
                            span,
                        },
                    );
                }
            }

            TypedExprKind::Block(stmts) => {
                let mut last_val = self.unit_expr(span.clone());
                let stmts_len = stmts.len();
                for (i, stmt) in stmts.into_iter().enumerate() {
                    let is_last = i == stmts_len - 1;
                    if is_last {
                        let (s, v) = self.lower_expr(stmt);
                        generated_stmts.extend(s);
                        last_val = v;
                    } else {
                        generated_stmts.extend(self.lower_stmt(stmt));
                    }
                }
                return (generated_stmts, last_val);
            }

            _ => panic!("Unexpected expression kind: {:?}", expr.kind),
        };

        (
            generated_stmts,
            IrExpr {
                kind,
                ty: ir_ty,
                span,
            },
        )
    }

    // utils.rs

    fn lower_type(&self, ty: &Type) -> IrType {
        match ty {
            Type::I32 => IrType::I32,
            Type::F32 => IrType::F32,
            Type::Bool => IrType::Bool,
            Type::Unit => IrType::Unit,
            Type::Ptr(inner) => IrType::Ptr(Box::new(self.lower_type(inner))),
            Type::Signature(_, _) => IrType::I32,
            _ => panic!("Unsupported type {:?} for IR generation.", ty),
        }
    }

    fn lower_lit(&self, lit: Lit) -> IrLit {
        match lit {
            Lit::Int(v) => IrLit::Int(v),
            Lit::Float(v) => IrLit::Float(v.0),
            Lit::Bool(v) => IrLit::Bool(v),
            _ => panic!("Unsupported literal."),
        }
    }
}
