use abyss_analyzer::type_checker::{
    tast::{TypedExpr, TypedExprKind, TypedProgram},
    types::Type,
};
use abyss_parser::ast::{BinaryOp, Lit};

use crate::ir::{IrBinaryOp, IrExpr, IrExprKind, IrFunction, IrLit, IrProgram, IrStmt, IrType};

pub struct IrBuilder;

impl IrBuilder {
    pub fn new() -> Self {
        Self
    }

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
                main_body.push(self.lower_stmt(stmt));
            }
        } else {
            main_body.push(self.lower_stmt(program.body));
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

            let mut ir_body = Vec::new();
            match body.kind {
                TypedExprKind::Block(stmts) => {
                    for stmt in stmts {
                        ir_body.push(self.lower_stmt(stmt));
                    }
                }
                _ => {
                    ir_body.push(self.lower_stmt(*body));
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

    fn lower_stmt(&mut self, expr: TypedExpr) -> IrStmt {
        match expr.kind {
            TypedExprKind::VarDec(name, ty, init) => {
                let ir_init = init.map(|e| self.lower_expr(*e));
                IrStmt::VarDec {
                    name,
                    ty: self.lower_type(&ty),
                    init: ir_init,
                }
            }

            TypedExprKind::Binary(left, BinaryOp::Assign, right) => {
                if let TypedExprKind::Ident(name) = left.kind {
                    IrStmt::Assign {
                        target: name,
                        val: self.lower_expr(*right),
                    }
                } else {
                    panic!(
                        "IR Builder: Complex assignments should be desugared before this phase."
                    );
                }
            }

            TypedExprKind::Ret(val) => {
                let ir_val = val.map(|e| self.lower_expr(*e));
                IrStmt::Return(ir_val)
            }

            _ => IrStmt::Expr(self.lower_expr(expr)),
        }
    }

    fn lower_expr(&mut self, expr: TypedExpr) -> IrExpr {
        let span = expr.span.clone();
        let ir_ty = self.lower_type(&expr.ty);

        let kind = match expr.kind {
            TypedExprKind::Lit(lit) => IrExprKind::Lit(self.lower_lit(lit)),

            TypedExprKind::Ident(name) => IrExprKind::VarRef(name),

            TypedExprKind::FuncRef(name) => IrExprKind::VarRef(name),

            TypedExprKind::Binary(left, op, right) => {
                let ir_op = match op {
                    BinaryOp::Add => IrBinaryOp::Add,
                    BinaryOp::Sub => IrBinaryOp::Sub,
                    BinaryOp::Mul => IrBinaryOp::Mul,
                    BinaryOp::Div => IrBinaryOp::Div,
                    _ => panic!("IR Builder: Unsupported binary op {:?} at this stage.", op),
                };
                IrExprKind::Binary(
                    Box::new(self.lower_expr(*left)),
                    ir_op,
                    Box::new(self.lower_expr(*right)),
                )
            }

            TypedExprKind::Call(func, args) => {
                let func_name = match func.kind {
                    TypedExprKind::Ident(name) => name,
                    TypedExprKind::FuncRef(name) => name,
                    _ => panic!("IR Builder: Dynamic dispatch / complex calls not supported yet."),
                };

                let ir_args = args.into_iter().map(|a| self.lower_expr(a)).collect();

                IrExprKind::Call {
                    func_name,
                    args: ir_args,
                }
            }

            _ => panic!(
                "IR Builder: Unexpected expression kind in flattened TAST: {:?}",
                expr.kind
            ),
        };

        IrExpr {
            kind,
            ty: ir_ty,
            span,
        }
    }

    fn lower_type(&self, ty: &Type) -> IrType {
        match ty {
            Type::I32 => IrType::I32,
            Type::F32 => IrType::F32,
            Type::Unit => IrType::Unit,

            Type::Signature(_, _) => IrType::I32,

            _ => panic!("IR Builder: Unsupported type {:?} for IR generation.", ty),
        }
    }

    fn lower_lit(&self, lit: Lit) -> IrLit {
        match lit {
            Lit::Int(v) => IrLit::Int(v),
            Lit::Float(v) => IrLit::Float(v.0),
            Lit::Bool(v) => IrLit::Bool(v),
            _ => panic!("IR Builder: Unsupported literal in IR."),
        }
    }
}
