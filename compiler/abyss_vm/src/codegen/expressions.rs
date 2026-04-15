use crate::vm::opcode::{Instruction, OpCode};

use super::IrCompiler;
use super::env::Env;
use abyss_ir::ir::{IrBinaryOp, IrExpr, IrExprKind, IrLit, IrType, IrUnaryOp};

impl IrCompiler {
    pub(crate) fn compile_expr(&mut self, env: &mut Env, expr: &IrExpr, target: Option<u8>) -> u8 {
        match &expr.kind {
            IrExprKind::Lit(lit) => self.compile_literal(env, lit, target),
            IrExprKind::VarRef(name) => self.compile_var_ref(env, name, target),
            IrExprKind::Unary(op, inner) => self.compile_unary(env, op, inner, target),
            IrExprKind::Binary(l, op, r) => self.compile_binary(env, l, op, r, &expr.ty, target),
            IrExprKind::Call { func_name, args } => self.compile_call(env, func_name, args, target),

            IrExprKind::ArrayInit(elements) => self.compile_array_init(env, elements, target),
            IrExprKind::ArrayRepeat { val, count } => {
                self.compile_array_repeat(env, val, *count, target)
            }

            IrExprKind::Index(base, index) => self.compile_index(env, base, index, target),

            IrExprKind::StructInit(fields) => self.compile_struct_init(env, fields, target),

            IrExprKind::FieldAccess { base, index } => {
                self.compile_field_access(env, base, *index, target)
            }

            IrExprKind::Cast(source_expr, target_ty) => {
                self.compile_cast(env, source_expr, target_ty, target)
            }

            IrExprKind::GetIndexPtr { base, index } => {
                self.compile_get_index_ptr(env, base, index, target)
            }
            IrExprKind::GetFieldPtr { base, index } => {
                self.compile_get_field_ptr(env, base, *index, target)
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

        if let Some(&global_idx) = self.global_indices.get(name) {
            let dest_reg = target.unwrap_or_else(|| env.alloc_reg());
            self.emit(Instruction {
                op: OpCode::LoadGlobal,
                a: dest_reg,
                b: ((global_idx >> 8) & 0xFF) as u8,
                c: (global_idx & 0xFF) as u8,
            });
            return dest_reg;
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
            IrUnaryOp::BitNot => {
                self.emit(Instruction {
                    op: OpCode::BitNot,
                    a: dest_reg,
                    b: r_inner,
                    c: 0,
                });
            }
            IrUnaryOp::Ref => {
                if let IrExprKind::VarRef(name) = &inner.kind {
                    let var_reg = env.get_var(name);
                    self.emit(Instruction {
                        op: OpCode::RefReg,
                        a: dest_reg,
                        b: var_reg,
                        c: 0,
                    });
                } else {
                    let r_inner = self.compile_expr(env, inner, None);
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
            }
            IrUnaryOp::Deref => {
                let r_inner = self.compile_expr(env, inner, None);
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

        let dest_reg = target.unwrap_or_else(|| env.alloc_reg());
        let is_float = matches!(left.ty, IrType::F32);

        let has_const_op = matches!(
            op,
            IrBinaryOp::Add
                | IrBinaryOp::Sub
                | IrBinaryOp::Mul
                | IrBinaryOp::Div
                | IrBinaryOp::Mod
                | IrBinaryOp::Eq
                | IrBinaryOp::Neq
                | IrBinaryOp::Lt
                | IrBinaryOp::Le
                | IrBinaryOp::Gt
                | IrBinaryOp::Ge
        );

        if has_const_op {
            if let IrExprKind::Lit(lit) = &right.kind {
                let const_val = match lit {
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
                let c_idx = self.add_const(const_val);
                let r_left = self.compile_expr(env, left, None);

                let opcode = match op {
                    IrBinaryOp::Add => {
                        if is_float {
                            OpCode::AddFC
                        } else {
                            OpCode::AddIC
                        }
                    }
                    IrBinaryOp::Sub => {
                        if is_float {
                            OpCode::SubFC
                        } else {
                            OpCode::SubIC
                        }
                    }
                    IrBinaryOp::Mul => {
                        if is_float {
                            OpCode::MulFC
                        } else {
                            OpCode::MulIC
                        }
                    }
                    IrBinaryOp::Div => {
                        if is_float {
                            OpCode::DivFC
                        } else {
                            OpCode::DivIC
                        }
                    }
                    IrBinaryOp::Mod => {
                        if is_float {
                            panic!("% not supported for floats")
                        } else {
                            OpCode::ModIC
                        }
                    }
                    IrBinaryOp::Eq => {
                        if is_float {
                            OpCode::CmpEqFC
                        } else {
                            OpCode::CmpEqIC
                        }
                    }
                    IrBinaryOp::Neq => {
                        if is_float {
                            OpCode::CmpNeqFC
                        } else {
                            OpCode::CmpNeqIC
                        }
                    }
                    IrBinaryOp::Lt => {
                        if is_float {
                            OpCode::CmpLtFC
                        } else {
                            OpCode::CmpLtIC
                        }
                    }
                    IrBinaryOp::Le => {
                        if is_float {
                            OpCode::CmpLeFC
                        } else {
                            OpCode::CmpLeIC
                        }
                    }
                    IrBinaryOp::Gt => {
                        if is_float {
                            OpCode::CmpGtFC
                        } else {
                            OpCode::CmpGtIC
                        }
                    }
                    IrBinaryOp::Ge => {
                        if is_float {
                            OpCode::CmpGeFC
                        } else {
                            OpCode::CmpGeIC
                        }
                    }
                    _ => unreachable!(),
                };

                self.emit(Instruction {
                    op: opcode,
                    a: dest_reg,
                    b: r_left,
                    c: c_idx,
                });
                return dest_reg;
            }

            if let IrExprKind::Lit(lit) = &left.kind {
                let is_commutative = matches!(
                    op,
                    IrBinaryOp::Add | IrBinaryOp::Mul | IrBinaryOp::Eq | IrBinaryOp::Neq
                );
                if is_commutative {
                    let const_val = match lit {
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
                    let c_idx = self.add_const(const_val);
                    let r_right = self.compile_expr(env, right, None);

                    let opcode = match op {
                        IrBinaryOp::Add => {
                            if is_float {
                                OpCode::AddFC
                            } else {
                                OpCode::AddIC
                            }
                        }
                        IrBinaryOp::Mul => {
                            if is_float {
                                OpCode::MulFC
                            } else {
                                OpCode::MulIC
                            }
                        }
                        IrBinaryOp::Eq => {
                            if is_float {
                                OpCode::CmpEqFC
                            } else {
                                OpCode::CmpEqIC
                            }
                        }
                        IrBinaryOp::Neq => {
                            if is_float {
                                OpCode::CmpNeqFC
                            } else {
                                OpCode::CmpNeqIC
                            }
                        }
                        _ => unreachable!(),
                    };

                    self.emit(Instruction {
                        op: opcode,
                        a: dest_reg,
                        b: r_right,
                        c: c_idx,
                    });
                    return dest_reg;
                }
            }
        }

        let r_left = self.compile_expr(env, left, None);
        let r_right = self.compile_expr(env, right, None);

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

            IrBinaryOp::BitAnd => {
                if is_float {
                    panic!("Bitwise AND not supported for floats")
                } else {
                    OpCode::BitAnd
                }
            }
            IrBinaryOp::BitOr => {
                if is_float {
                    panic!("Bitwise OR not supported for floats")
                } else {
                    OpCode::BitOr
                }
            }
            IrBinaryOp::BitXor => {
                if is_float {
                    panic!("Bitwise XOR not supported for floats")
                } else {
                    OpCode::BitXor
                }
            }
            IrBinaryOp::Shl => {
                if is_float {
                    panic!("Shift Left not supported for floats")
                } else {
                    OpCode::Shl
                }
            }
            IrBinaryOp::Shr => {
                if is_float {
                    panic!("Shift Right not supported for floats")
                } else {
                    OpCode::ShrI
                }
            }

            _ => unreachable!("Unhandled opcode in reg-to-reg fallback: {:?}", op),
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
        if let Some(&ext_idx) = self.extern_indices.get(func_name) {
            let arg_start_reg = env.next_reg;
            env.next_reg += args.len() as u8;

            for (i, arg) in args.iter().enumerate() {
                let target_reg = arg_start_reg + (i as u8);
                self.compile_expr(env, arg, Some(target_reg));
            }

            let func_idx_reg = env.alloc_reg();

            let const_idx = self.add_const(ext_idx as u64);

            self.emit(Instruction {
                op: OpCode::LoadConst,
                a: func_idx_reg,
                b: const_idx,
                c: 0,
            });

            let dest_reg = target.unwrap_or_else(|| env.alloc_reg());
            self.emit(Instruction {
                op: OpCode::CallExtern,
                a: dest_reg,
                b: func_idx_reg,
                c: arg_start_reg,
            });
            return dest_reg;
        }

        let frame_offset = env.next_reg;
        env.next_reg += args.len() as u8;

        for (i, arg) in args.iter().enumerate() {
            let target_reg = frame_offset + (i as u8);
            let arg_val_reg = self.compile_expr(env, arg, None);
            let final_arg_reg = self.copy_if_complex(env, arg_val_reg, &arg.ty);

            self.emit(Instruction {
                op: OpCode::Move,
                a: target_reg,
                b: final_arg_reg,
                c: 0,
            });
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

    fn compile_array_init(&mut self, env: &mut Env, elements: &[IrExpr], target: Option<u8>) -> u8 {
        let count = elements.len();
        let size_bytes = (count * 8) as u64;
        let size_idx = self.add_const(size_bytes);
        let size_reg = env.alloc_reg();
        self.emit(Instruction {
            op: OpCode::LoadConst,
            a: size_reg,
            b: size_idx,
            c: 0,
        });

        let arr_ptr = target.unwrap_or_else(|| env.alloc_reg());
        self.emit(Instruction {
            op: OpCode::Alloc,
            a: arr_ptr,
            b: size_reg,
            c: 0,
        });

        for (i, expr) in elements.iter().enumerate() {
            let val_reg = self.compile_expr(env, expr, None);

            let idx_const = self.add_const(i as u64);
            let idx_reg = env.alloc_reg();
            self.emit(Instruction {
                op: OpCode::LoadConst,
                a: idx_reg,
                b: idx_const,
                c: 0,
            });

            self.emit(Instruction {
                op: OpCode::StorePtrOffset,
                a: arr_ptr,
                b: val_reg,
                c: idx_reg,
            });
        }

        arr_ptr
    }

    fn compile_array_repeat(
        &mut self,
        env: &mut Env,
        val_expr: &IrExpr,
        count: usize,
        target: Option<u8>,
    ) -> u8 {
        let size_bytes = (count * 8) as u64;

        let size_idx = self.add_const(size_bytes);
        let size_reg = env.alloc_reg();
        self.emit(Instruction {
            op: OpCode::LoadConst,
            a: size_reg,
            b: size_idx,
            c: 0,
        });

        let arr_ptr = target.unwrap_or_else(|| env.alloc_reg());
        self.emit(Instruction {
            op: OpCode::Alloc,
            a: arr_ptr,
            b: size_reg,
            c: 0,
        });

        if count == 0 {
            return arr_ptr;
        }

        let val_reg = self.compile_expr(env, val_expr, None);

        let i_reg = env.alloc_reg();
        let zero_idx = self.add_const(0);
        self.emit(Instruction {
            op: OpCode::LoadConst,
            a: i_reg,
            b: zero_idx,
            c: 0,
        });

        let count_reg = env.alloc_reg();
        let count_idx = self.add_const(count as u64);
        self.emit(Instruction {
            op: OpCode::LoadConst,
            a: count_reg,
            b: count_idx,
            c: 0,
        });

        let loop_start_idx = self.instructions.len();

        let cmp_reg = env.alloc_reg();
        self.emit(Instruction {
            op: OpCode::CmpLtI,
            a: cmp_reg,
            b: i_reg,
            c: count_reg,
        });

        let jmpz_idx = self.instructions.len();
        self.emit(Instruction {
            op: OpCode::JmpZImm,
            a: cmp_reg,
            b: 0,
            c: 0,
        });

        self.emit(Instruction {
            op: OpCode::StorePtrOffset,
            a: arr_ptr,
            b: val_reg,
            c: i_reg,
        });

        let one_idx = self.add_const(1);
        self.emit(Instruction {
            op: OpCode::AddIC,
            a: i_reg,
            b: i_reg,
            c: one_idx,
        });

        let jmp_back_idx = self.instructions.len();
        self.emit(Instruction {
            op: OpCode::JmpImm,
            a: 0,
            b: 0,
            c: 0,
        });
        self.patch_jump(jmp_back_idx, loop_start_idx);

        let end_idx = self.instructions.len();
        self.patch_jump(jmpz_idx, end_idx);

        arr_ptr
    }

    fn compile_index(
        &mut self,
        env: &mut Env,
        base: &IrExpr,
        index: &IrExpr,
        target: Option<u8>,
    ) -> u8 {
        let base_reg = self.compile_expr(env, base, None);

        let index_reg = self.compile_expr(env, index, None);

        let dest_reg = target.unwrap_or_else(|| env.alloc_reg());

        self.emit(Instruction {
            op: OpCode::LoadPtrOffset,
            a: dest_reg,
            b: base_reg,
            c: index_reg,
        });

        dest_reg
    }

    fn compile_struct_init(&mut self, env: &mut Env, fields: &[IrExpr], target: Option<u8>) -> u8 {
        let count = fields.len();
        let size_bytes = (count * 8) as u64;

        let size_idx = self.add_const(size_bytes);
        let size_reg = env.alloc_reg();
        self.emit(Instruction {
            op: OpCode::LoadConst,
            a: size_reg,
            b: size_idx,
            c: 0,
        });

        let struct_ptr = target.unwrap_or_else(|| env.alloc_reg());
        self.emit(Instruction {
            op: OpCode::Alloc,
            a: struct_ptr,
            b: size_reg,
            c: 0,
        });

        for (i, expr) in fields.iter().enumerate() {
            let val_reg = self.compile_expr(env, expr, None);

            let idx_const = self.add_const(i as u64);
            let idx_reg = env.alloc_reg();
            self.emit(Instruction {
                op: OpCode::LoadConst,
                a: idx_reg,
                b: idx_const,
                c: 0,
            });

            self.emit(Instruction {
                op: OpCode::StorePtrOffset,
                a: struct_ptr,
                b: val_reg,
                c: idx_reg,
            });
        }

        struct_ptr
    }

    fn compile_field_access(
        &mut self,
        env: &mut Env,
        base: &IrExpr,
        index: usize,
        target: Option<u8>,
    ) -> u8 {
        let base_reg = self.compile_expr(env, base, None);

        let idx_const = self.add_const(index as u64);
        let index_reg = env.alloc_reg();
        self.emit(Instruction {
            op: OpCode::LoadConst,
            a: index_reg,
            b: idx_const,
            c: 0,
        });

        let dest_reg = target.unwrap_or_else(|| env.alloc_reg());

        self.emit(Instruction {
            op: OpCode::LoadPtrOffset,
            a: dest_reg,
            b: base_reg,
            c: index_reg,
        });

        dest_reg
    }

    fn compile_cast(
        &mut self,
        env: &mut Env,
        source_expr: &IrExpr,
        target_type: &IrType,
        target: Option<u8>,
    ) -> u8 {
        let source_reg = self.compile_expr(env, source_expr, None);
        let source_type = &source_expr.ty;

        if source_type == target_type {
            if let Some(t) = target {
                if t != source_reg {
                    self.emit(Instruction {
                        op: OpCode::Move,
                        a: t,
                        b: source_reg,
                        c: 0,
                    });
                }
                return t;
            }
            return source_reg;
        }

        let dest_reg = target.unwrap_or_else(|| env.alloc_reg());

        let opcode = match (source_type, target_type) {
            (IrType::I32, IrType::F32) => OpCode::CastI2F,
            (IrType::F32, IrType::I32) => OpCode::CastF2I,
            (IrType::I32, IrType::Bool) => OpCode::CastI2B,
            (IrType::F32, IrType::Bool) => OpCode::CastF2B,

            (IrType::Bool, IrType::I32) => {
                if dest_reg != source_reg {
                    self.emit(Instruction {
                        op: OpCode::Move,
                        a: dest_reg,
                        b: source_reg,
                        c: 0,
                    });
                }
                return dest_reg;
            }

            (IrType::Bool, IrType::F32) => OpCode::CastI2F,

            (src, dst) => {
                panic!(
                    "Compiler Bug: Unhandled or invalid cast from {:?} to {:?} reached CodeGen.",
                    src, dst
                );
            }
        };

        self.emit(Instruction {
            op: opcode,
            a: dest_reg,
            b: source_reg,
            c: 0,
        });

        dest_reg
    }

    fn compile_get_index_ptr(
        &mut self,
        env: &mut Env,
        base: &IrExpr,
        index: &IrExpr,
        target: Option<u8>,
    ) -> u8 {
        let base_reg = self.compile_expr(env, base, None);

        let index_reg = self.compile_expr(env, index, None);

        let dest_reg = target.unwrap_or_else(|| env.alloc_reg());
        let offset_reg = env.alloc_reg();

        let eight_idx = self.add_const(8);
        self.emit(Instruction {
            op: OpCode::MulIC,
            a: offset_reg,
            b: index_reg,
            c: eight_idx,
        });

        self.emit(Instruction {
            op: OpCode::AddI,
            a: dest_reg,
            b: base_reg,
            c: offset_reg,
        });

        dest_reg
    }

    fn compile_get_field_ptr(
        &mut self,
        env: &mut Env,
        base: &IrExpr,
        index: usize,
        target: Option<u8>,
    ) -> u8 {
        let base_reg = self.compile_expr(env, base, None);

        let dest_reg = target.unwrap_or_else(|| env.alloc_reg());

        let offset_bytes = (index * 8) as u64;

        if offset_bytes == 0 {
            if let Some(t) = target {
                if t != base_reg {
                    self.emit(Instruction {
                        op: OpCode::Move,
                        a: t,
                        b: base_reg,
                        c: 0,
                    });
                }
                return t;
            }
            return base_reg;
        }

        let offset_idx = self.add_const(offset_bytes);
        self.emit(Instruction {
            op: OpCode::AddIC,
            a: dest_reg,
            b: base_reg,
            c: offset_idx,
        });

        dest_reg
    }
}
