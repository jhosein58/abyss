use super::IrCompiler;
use super::env::Env;
use crate::{Instruction, OpCode};
use abyss_ir::ir::{IrBinaryOp, IrExpr, IrExprKind, IrLit, IrType, IrUnaryOp};

impl IrCompiler {
    pub(crate) fn compile_expr(&mut self, env: &mut Env, expr: &IrExpr, target: Option<u8>) -> u8 {
        match &expr.kind {
            IrExprKind::Lit(lit) => self.compile_literal(env, lit, target),
            IrExprKind::VarRef(name) => self.compile_var_ref(env, name, target),
            IrExprKind::Unary(op, inner) => self.compile_unary(env, op, inner, target),
            IrExprKind::Binary(l, op, r) => self.compile_binary(env, l, op, r, &expr.ty, target),
            IrExprKind::Call { func_name, args } => self.compile_call(env, func_name, args, target),
            IrExprKind::NativeCall { func_index, args } => {
                self.compile_native_call(env, *func_index, args, target)
            }
        }
    }

    fn compile_literal(&mut self, env: &mut Env, lit: &IrLit, target: Option<u8>) -> u8 {
        let val_u64 = match lit {
            IrLit::Int(i) => *i as u64,
            IrLit::Float(f) => f.to_bits(),
            IrLit::Bool(b) => {
                if *b {
                    1
                } else {
                    0
                }
            }
        };
        let const_idx = self.add_const(val_u64);
        let dest_reg = target.unwrap_or_else(|| env.alloc_reg());

        self.emit(Instruction {
            op: OpCode::LoadConst,
            a: dest_reg,
            b: const_idx,
            c: 0,
        });
        dest_reg
    }

    fn compile_var_ref(&mut self, env: &mut Env, name: &str, target: Option<u8>) -> u8 {
        if let Some(&reg) = env.vars.get(name) {
            if let Some(t) = target {
                if t != reg {
                    self.emit(Instruction {
                        op: OpCode::Move,
                        a: t,
                        b: reg,
                        c: 0,
                    });
                }
                return t;
            }
            return reg;
        }

        if let Some(&func_const_idx) = self.func_const_indices.get(name) {
            let dest_reg = target.unwrap_or_else(|| env.alloc_reg());
            self.emit(Instruction {
                op: OpCode::LoadConst,
                a: dest_reg,
                b: func_const_idx,
                c: 0,
            });
            return dest_reg;
        }
        panic!("Variable or function '{}' not found", name);
    }

    fn compile_unary(
        &mut self,
        env: &mut Env,
        op: &IrUnaryOp,
        inner: &IrExpr,
        target: Option<u8>,
    ) -> u8 {
        let r_inner = self.compile_expr(env, inner, None);
        let dest_reg = target.unwrap_or_else(|| env.alloc_reg());

        match op {
            IrUnaryOp::Not => {
                self.emit(Instruction {
                    op: OpCode::Not,
                    a: dest_reg,
                    b: r_inner,
                    c: 0,
                });
            }
            IrUnaryOp::Neg => {
                let zero_reg = self.emit_load_zero(env);
                let is_float = matches!(inner.ty, IrType::F32);
                let op_code = if is_float { OpCode::SubF } else { OpCode::SubI };
                self.emit(Instruction {
                    op: op_code,
                    a: dest_reg,
                    b: zero_reg,
                    c: r_inner,
                });
            }
            IrUnaryOp::Ref => {
                let size_reg = env.alloc_reg();
                let size_idx = self.add_const(8);
                self.emit(Instruction {
                    op: OpCode::LoadConst,
                    a: size_reg,
                    b: size_idx,
                    c: 0,
                });
                self.emit(Instruction {
                    op: OpCode::Alloc,
                    a: dest_reg,
                    b: size_reg,
                    c: 0,
                });
                self.emit(Instruction {
                    op: OpCode::StorePtr,
                    a: dest_reg,
                    b: r_inner,
                    c: 0,
                });
            }
            IrUnaryOp::Deref => {
                self.emit(Instruction {
                    op: OpCode::LoadPtr,
                    a: dest_reg,
                    b: r_inner,
                    c: 0,
                });
            }
        }
        dest_reg
    }

    fn compile_binary(
        &mut self,
        env: &mut Env,
        left: &IrExpr,
        op: &IrBinaryOp,
        right: &IrExpr,
        _ty: &IrType,
        target: Option<u8>,
    ) -> u8 {
        if *op == IrBinaryOp::And || *op == IrBinaryOp::Or {
            return self.compile_logical_short_circuit(env, left, op, right, target);
        }

        let r_left = self.compile_expr(env, left, None);
        let r_right = self.compile_expr(env, right, None);
        let dest_reg = target.unwrap_or_else(|| env.alloc_reg());
        let is_float = matches!(left.ty, IrType::F32);

        let opcode = match op {
            IrBinaryOp::Add => {
                if is_float {
                    OpCode::AddF
                } else {
                    OpCode::AddI
                }
            }
            IrBinaryOp::Sub => {
                if is_float {
                    OpCode::SubF
                } else {
                    OpCode::SubI
                }
            }
            IrBinaryOp::Mul => {
                if is_float {
                    OpCode::MulF
                } else {
                    OpCode::MulI
                }
            }
            IrBinaryOp::Div => {
                if is_float {
                    OpCode::DivF
                } else {
                    OpCode::DivI
                }
            }
            IrBinaryOp::Mod => {
                if is_float {
                    panic!("% not supported for floats")
                } else {
                    OpCode::ModI
                }
            }
            IrBinaryOp::Eq => {
                if is_float {
                    OpCode::CmpEqF
                } else {
                    OpCode::CmpEqI
                }
            }
            IrBinaryOp::Neq => {
                if is_float {
                    OpCode::CmpNeqF
                } else {
                    OpCode::CmpNeqI
                }
            }
            IrBinaryOp::Lt => {
                if is_float {
                    OpCode::CmpLtF
                } else {
                    OpCode::CmpLtI
                }
            }
            IrBinaryOp::Le => {
                if is_float {
                    OpCode::CmpLeF
                } else {
                    OpCode::CmpLeI
                }
            }
            IrBinaryOp::Gt => {
                if is_float {
                    OpCode::CmpGtF
                } else {
                    OpCode::CmpGtI
                }
            }
            IrBinaryOp::Ge => {
                if is_float {
                    OpCode::CmpGeF
                } else {
                    OpCode::CmpGeI
                }
            }
            _ => unreachable!(),
        };

        self.emit(Instruction {
            op: opcode,
            a: dest_reg,
            b: r_left,
            c: r_right,
        });
        dest_reg
    }

    fn compile_logical_short_circuit(
        &mut self,
        env: &mut Env,
        left: &IrExpr,
        op: &IrBinaryOp,
        right: &IrExpr,
        target: Option<u8>,
    ) -> u8 {
        let dest_reg = target.unwrap_or_else(|| env.alloc_reg());

        self.compile_expr(env, left, Some(dest_reg));

        let jmpz_idx = self.instructions.len();
        self.emit(Instruction {
            op: OpCode::JmpZImm,
            a: dest_reg,
            b: 0,
            c: 0,
        });

        if *op == IrBinaryOp::And {
            self.compile_expr(env, right, Some(dest_reg));
            let end_idx = self.instructions.len();
            self.patch_jump(jmpz_idx, end_idx);
        } else {
            let jmp_end_idx = self.instructions.len();
            self.emit(Instruction {
                op: OpCode::JmpImm,
                a: 0,
                b: 0,
                c: 0,
            });

            let right_eval_idx = self.instructions.len();
            self.patch_jump(jmpz_idx, right_eval_idx);

            self.compile_expr(env, right, Some(dest_reg));

            let end_idx = self.instructions.len();
            self.patch_jump(jmp_end_idx, end_idx);
        }
        dest_reg
    }

    fn compile_call(
        &mut self,
        env: &mut Env,
        func_name: &str,
        args: &[IrExpr],
        target: Option<u8>,
    ) -> u8 {
        if func_name == "print" && args.len() == 1 {
            let arg_reg = self.compile_expr(env, &args[0], None);
            let op = if matches!(args[0].ty, IrType::F32) {
                OpCode::PrintF
            } else {
                OpCode::PrintI
            };
            self.emit(Instruction {
                op,
                a: arg_reg,
                b: 0,
                c: 0,
            });

            let zero_reg = self.emit_load_zero(env);
            if let Some(t) = target {
                if t != zero_reg {
                    self.emit(Instruction {
                        op: OpCode::Move,
                        a: t,
                        b: zero_reg,
                        c: 0,
                    });
                }
                return t;
            }
            return zero_reg;
        }

        let frame_offset = env.next_reg;
        env.next_reg += args.len() as u8;

        for (i, arg) in args.iter().enumerate() {
            let target_reg = frame_offset + (i as u8);
            self.compile_expr(env, arg, Some(target_reg));
        }

        let r_addr = self.compile_var_ref(env, func_name, None);
        let dest_reg = target.unwrap_or_else(|| env.alloc_reg());

        self.emit(Instruction {
            op: OpCode::Call,
            a: dest_reg,
            b: r_addr,
            c: frame_offset,
        });
        dest_reg
    }

    fn compile_native_call(
        &mut self,
        env: &mut Env,
        func_index: usize,
        args: &[IrExpr],
        target: Option<u8>,
    ) -> u8 {
        let arg_start_reg = env.next_reg;
        env.next_reg += args.len() as u8;

        for (i, arg) in args.iter().enumerate() {
            let target_reg = arg_start_reg + (i as u8);
            self.compile_expr(env, arg, Some(target_reg));
        }

        let dest_reg = target.unwrap_or_else(|| env.alloc_reg());
        self.emit(Instruction {
            op: OpCode::CallNative,
            a: dest_reg,
            b: func_index as u8,
            c: arg_start_reg,
        });
        dest_reg
    }
}
