use std::collections::HashMap;

use abyss_analyzer::type_checker::{
    tast::{TypedExpr, TypedExprKind, TypedProgram},
    types::Type,
};
use abyss_parser::ast::{BinaryOp, Lit};

use crate::{Instruction, OpCode};

#[derive(Debug)]
pub struct VmBuilder {
    pub instructions: Vec<Instruction>,
    pub constants: Vec<u64>,
}

impl VmBuilder {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
            constants: Vec::new(),
        }
    }
    pub fn add_const_i64(&mut self, val: i64) -> u8 {
        self.constants.push(val as u64);
        (self.constants.len() - 1) as u8
    }
    pub fn add_const_f64(&mut self, val: f64) -> u8 {
        self.constants.push(val.to_bits());
        (self.constants.len() - 1) as u8
    }
    pub fn emit(&mut self, op: OpCode, a: u8, b: u8, c: u8) {
        self.instructions.push(Instruction { op, a, b, c });
    }
}

pub struct Compiler {
    pub builder: VmBuilder,
    locals: HashMap<String, u8>,
    next_free_reg: u8,
}

impl Compiler {
    pub fn new() -> Self {
        Self {
            builder: VmBuilder::new(),
            locals: HashMap::new(),
            next_free_reg: 0,
        }
    }

    fn alloc_reg(&mut self) -> u8 {
        let reg = self.next_free_reg;
        self.next_free_reg += 1;
        reg
    }

    pub fn compile_program(&mut self, program: &TypedProgram) {
        self.compile_expr(&program.body);
        self.builder.emit(OpCode::Halt, 0, 0, 0);
    }

    pub fn compile_expr(&mut self, expr: &TypedExpr) -> u8 {
        match &expr.kind {
            TypedExprKind::Lit(lit) => {
                let res_reg = self.alloc_reg();
                match lit {
                    Lit::Int(val) => {
                        let const_idx = self.builder.add_const_i64(*val);
                        self.builder.emit(OpCode::LoadConst, res_reg, const_idx, 0);
                    }
                    Lit::Float(val) => {
                        let const_idx = self.builder.add_const_f64(val.0);
                        self.builder.emit(OpCode::LoadConst, res_reg, const_idx, 0);
                    }
                    _ => unimplemented!("Only Int and Float literals are supported for now."),
                }
                res_reg
            }

            TypedExprKind::Ident(name) => {
                if let Some(&reg) = self.locals.get(name) {
                    reg
                } else {
                    panic!("Codegen Error: Variable '{}' not found!", name);
                }
            }

            TypedExprKind::VarDec(name, _ty, Some(init_expr)) => {
                let val_reg = self.compile_expr(init_expr);
                self.locals.insert(name.clone(), val_reg);
                val_reg
            }

            TypedExprKind::Binary(left, op, right) => {
                let left_reg = self.compile_expr(left);
                let right_reg = self.compile_expr(right);
                let res_reg = self.alloc_reg();

                let is_float = matches!(expr.ty, Type::F32);

                match op {
                    BinaryOp::Add => {
                        let opc = if is_float { OpCode::AddF } else { OpCode::AddI };
                        self.builder.emit(opc, res_reg, left_reg, right_reg);
                    }
                    BinaryOp::Sub => {
                        let opc = if is_float { OpCode::SubF } else { OpCode::SubI };
                        self.builder.emit(opc, res_reg, left_reg, right_reg);
                    }
                    BinaryOp::Mul => {
                        let opc = if is_float { OpCode::MulF } else { OpCode::MulI };
                        self.builder.emit(opc, res_reg, left_reg, right_reg);
                    }
                    BinaryOp::Div => {
                        let opc = if is_float { OpCode::DivF } else { OpCode::DivI };
                        self.builder.emit(opc, res_reg, left_reg, right_reg);
                    }
                    _ => unimplemented!("Binary op {:?} not supported yet.", op),
                }
                res_reg
            }

            // TypedExprKind::Unary(op, operand) => {
            //     let op_reg = self.compile_expr(operand);
            //     let res_reg = self.alloc_reg();
            //     let is_float = matches!(expr.ty, Type::F32);

            //     match op {
            //         UnaryOp::Neg => {
            //             let opc = if is_float { OpCode::NegF } else { OpCode::NegI };
            //             self.builder.emit(opc, res_reg, op_reg, 0); // رجیستر سوم استفاده نمی‌شود
            //         }
            //         _ => unimplemented!("Unary op {:?} not supported yet.", op),
            //     }
            //     res_reg
            // }
            TypedExprKind::Block(stmts) => {
                let mut last_reg = 0;
                for stmt in stmts {
                    last_reg = self.compile_expr(stmt);
                }
                last_reg
            }

            _ => unimplemented!("Expr kind {:?} not supported yet.", expr.kind),
        }
    }
}
