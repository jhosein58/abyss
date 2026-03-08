use crate::{Instruction, OpCode};
use abyss_ir::ir::{
    IrBinaryOp, IrExpr, IrExprKind, IrFunction, IrLit, IrProgram, IrStmt, IrType, IrUnaryOp,
};
use std::collections::HashMap;

struct Env {
    vars: HashMap<String, u8>,
    next_reg: u8,
}

impl Env {
    fn new() -> Self {
        Self {
            vars: HashMap::new(),
            next_reg: 0,
        }
    }

    fn alloc_reg(&mut self) -> u8 {
        let r = self.next_reg;
        if r == 255 {
            panic!("Register overflow! A single function used more than 255 registers.");
        }
        self.next_reg += 1;
        r
    }

    fn declare_var(&mut self, name: String) -> u8 {
        let r = self.alloc_reg();
        self.vars.insert(name, r);
        r
    }

    fn get_var(&self, name: &str) -> u8 {
        *self
            .vars
            .get(name)
            .unwrap_or_else(|| panic!("Variable '{}' not found in scope", name))
    }
}

pub struct IrCompiler {
    instructions: Vec<Instruction>,
    constants: Vec<u64>,
    func_const_indices: HashMap<String, u8>,
}

impl IrCompiler {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
            constants: Vec::new(),
            func_const_indices: HashMap::new(),
        }
    }

    fn emit(&mut self, inst: Instruction) {
        self.instructions.push(inst);
    }

    fn patch_jump(&mut self, inst_idx: usize, target_idx: usize) {
        if target_idx > 0xFFFF {
            panic!("Jump target address too large!");
        }
        let inst = &mut self.instructions[inst_idx];
        inst.b = ((target_idx >> 8) & 0xFF) as u8;
        inst.c = (target_idx & 0xFF) as u8;
    }

    fn add_const(&mut self, val: u64) -> u8 {
        let reserved_slots = self.func_const_indices.len();

        if let Some(offset) = self.constants[reserved_slots..]
            .iter()
            .position(|&c| c == val)
        {
            let idx = reserved_slots + offset;
            if idx < 256 {
                return idx as u8;
            }
        }

        let idx = self.constants.len();
        if idx >= 256 {
            panic!("Too many constants! Limit is 256.");
        }
        self.constants.push(val);
        idx as u8
    }

    pub fn compile_comptime_func(mut self, func: &IrFunction) -> (Vec<Instruction>, Vec<u64>) {
        let idx = self.constants.len() as u8;
        self.constants.push(0);
        self.func_const_indices.insert(func.name.clone(), idx);

        let func_ip = self.instructions.len() as u64;
        self.constants[idx as usize] = func_ip;

        let mut env = Env::new();

        for stmt in &func.body {
            self.compile_stmt(&mut env, stmt);
        }

        let r_dummy = env.alloc_reg();
        let zero_idx = self.add_const(0);
        self.emit(Instruction {
            op: OpCode::LoadConst,
            a: r_dummy,
            b: zero_idx,
            c: 0,
        });
        self.emit(Instruction {
            op: OpCode::Ret,
            a: r_dummy,
            b: 0,
            c: 0,
        });

        (self.instructions, self.constants)
    }

    pub fn compile(mut self, program: &IrProgram) -> (Vec<Instruction>, Vec<u64>) {
        for func in &program.functions {
            let idx = self.constants.len() as u8;
            self.constants.push(0);
            self.func_const_indices.insert(func.name.clone(), idx);
        }

        if let Some(main_const_idx) = self.func_const_indices.get("main") {
            let r_addr = 0;
            let r_dest = 1;

            self.emit(Instruction {
                op: OpCode::LoadConst,
                a: r_addr,
                b: *main_const_idx,
                c: 0,
            });
            self.emit(Instruction {
                op: OpCode::Call,
                a: r_dest,
                b: r_addr,
                c: 2,
            });
            self.emit(Instruction {
                op: OpCode::Halt,
                a: 0,
                b: 0,
                c: 0,
            });
        } else {
            panic!("Program must have a 'main' function.");
        }

        for func in &program.functions {
            let func_ip = self.instructions.len() as u64;
            let const_idx = self.func_const_indices[&func.name];
            self.constants[const_idx as usize] = func_ip;

            let mut env = Env::new();

            for (param_name, _) in &func.params {
                let r = env.alloc_reg();
                env.vars.insert(param_name.clone(), r);
            }

            for stmt in &func.body {
                self.compile_stmt(&mut env, stmt);
            }

            let r_dummy = env.alloc_reg();
            let zero_idx = self.add_const(0);
            self.emit(Instruction {
                op: OpCode::LoadConst,
                a: r_dummy,
                b: zero_idx,
                c: 0,
            });
            self.emit(Instruction {
                op: OpCode::Ret,
                a: r_dummy,
                b: 0,
                c: 0,
            });
        }

        (self.instructions, self.constants)
    }

    fn compile_stmt(&mut self, env: &mut Env, stmt: &IrStmt) {
        match stmt {
            IrStmt::VarDec { name, ty: _, init } => {
                let dest_reg = env.declare_var(name.clone());
                if let Some(expr) = init {
                    let val_reg = self.compile_expr(env, expr);
                    self.emit(Instruction {
                        op: OpCode::Move,
                        a: dest_reg,
                        b: val_reg,
                        c: 0,
                    });
                }
            }
            IrStmt::Assign { target, val } => {
                let dest_reg = env.get_var(target);
                let val_reg = self.compile_expr(env, val);
                self.emit(Instruction {
                    op: OpCode::Move,
                    a: dest_reg,
                    b: val_reg,
                    c: 0,
                });
            }
            IrStmt::Expr(expr) => {
                self.compile_expr(env, expr);
            }
            IrStmt::Return(opt_expr) => {
                let ret_reg = match opt_expr {
                    Some(expr) => self.compile_expr(env, expr),
                    None => {
                        let r = env.alloc_reg();
                        let zero = self.add_const(0);
                        self.emit(Instruction {
                            op: OpCode::LoadConst,
                            a: r,
                            b: zero,
                            c: 0,
                        });
                        r
                    }
                };
                self.emit(Instruction {
                    op: OpCode::Ret,
                    a: ret_reg,
                    b: 0,
                    c: 0,
                });
            }
            IrStmt::If(cond, then_branch, else_branch) => {
                let cond_reg = self.compile_expr(env, cond);

                let jmpz_idx = self.instructions.len();
                self.emit(Instruction {
                    op: OpCode::JmpZImm,
                    a: cond_reg,
                    b: 0,
                    c: 0,
                });

                let vars_backup = env.vars.clone();

                for stmt in then_branch {
                    self.compile_stmt(env, stmt);
                }

                let jmp_end_idx = self.instructions.len();
                self.emit(Instruction {
                    op: OpCode::JmpImm,
                    a: 0,
                    b: 0,
                    c: 0,
                });

                let else_start_idx = self.instructions.len();
                self.patch_jump(jmpz_idx, else_start_idx);

                env.vars = vars_backup.clone();

                for stmt in else_branch {
                    self.compile_stmt(env, stmt);
                }

                let end_idx = self.instructions.len();
                self.patch_jump(jmp_end_idx, end_idx);

                env.vars = vars_backup;
            }
        }
    }

    fn compile_expr(&mut self, env: &mut Env, expr: &IrExpr) -> u8 {
        match &expr.kind {
            IrExprKind::Lit(lit) => {
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
                let dest_reg = env.alloc_reg();
                self.emit(Instruction {
                    op: OpCode::LoadConst,
                    a: dest_reg,
                    b: const_idx,
                    c: 0,
                });
                dest_reg
            }

            IrExprKind::VarRef(name) => {
                if let Some(&reg) = env.vars.get(name) {
                    return reg;
                }

                if let Some(&func_const_idx) = self.func_const_indices.get(name) {
                    let dest_reg = env.alloc_reg();
                    self.emit(Instruction {
                        op: OpCode::LoadConst,
                        a: dest_reg,
                        b: func_const_idx,
                        c: 0,
                    });
                    return dest_reg;
                }

                panic!("Variable or function '{}' not found in scope", name);
            }

            IrExprKind::Unary(op, inner_expr) => {
                let r_inner = self.compile_expr(env, inner_expr);
                let r_dest = env.alloc_reg();

                match op {
                    IrUnaryOp::Not => {
                        self.emit(Instruction {
                            op: OpCode::Not,
                            a: r_dest,
                            b: r_inner,
                            c: 0,
                        });
                    }
                    IrUnaryOp::Neg => {
                        let zero_reg = env.alloc_reg();
                        if matches!(inner_expr.ty, IrType::F32) {
                            let zero_idx = self.add_const(0.0f64.to_bits());
                            self.emit(Instruction {
                                op: OpCode::LoadConst,
                                a: zero_reg,
                                b: zero_idx,
                                c: 0,
                            });
                            self.emit(Instruction {
                                op: OpCode::SubF,
                                a: r_dest,
                                b: zero_reg,
                                c: r_inner,
                            });
                        } else {
                            let zero_idx = self.add_const(0);
                            self.emit(Instruction {
                                op: OpCode::LoadConst,
                                a: zero_reg,
                                b: zero_idx,
                                c: 0,
                            });
                            self.emit(Instruction {
                                op: OpCode::SubI,
                                a: r_dest,
                                b: zero_reg,
                                c: r_inner,
                            });
                        }
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
                            a: r_dest,
                            b: size_reg,
                            c: 0,
                        });

                        self.emit(Instruction {
                            op: OpCode::StorePtr,
                            a: r_dest,
                            b: r_inner,
                            c: 0,
                        });
                    }
                    IrUnaryOp::Deref => {
                        self.emit(Instruction {
                            op: OpCode::LoadPtr,
                            a: r_dest,
                            b: r_inner,
                            c: 0,
                        });
                    }
                }
                r_dest
            }

            IrExprKind::Binary(left, op, right) => {
                if *op == IrBinaryOp::And {
                    let r_dest = env.alloc_reg();
                    let r_left = self.compile_expr(env, left);
                    self.emit(Instruction {
                        op: OpCode::Move,
                        a: r_dest,
                        b: r_left,
                        c: 0,
                    });

                    let jmpz_idx = self.instructions.len();
                    self.emit(Instruction {
                        op: OpCode::JmpZImm,
                        a: r_left,
                        b: 0,
                        c: 0,
                    });

                    let r_right = self.compile_expr(env, right);
                    self.emit(Instruction {
                        op: OpCode::Move,
                        a: r_dest,
                        b: r_right,
                        c: 0,
                    });

                    let end_idx = self.instructions.len();
                    self.patch_jump(jmpz_idx, end_idx);

                    return r_dest;
                }

                if *op == IrBinaryOp::Or {
                    let r_dest = env.alloc_reg();
                    let r_left = self.compile_expr(env, left);
                    self.emit(Instruction {
                        op: OpCode::Move,
                        a: r_dest,
                        b: r_left,
                        c: 0,
                    });

                    let jmpz_idx = self.instructions.len();
                    self.emit(Instruction {
                        op: OpCode::JmpZImm,
                        a: r_left,
                        b: 0,
                        c: 0,
                    });

                    let jmp_end_idx = self.instructions.len();
                    self.emit(Instruction {
                        op: OpCode::JmpImm,
                        a: 0,
                        b: 0,
                        c: 0,
                    });

                    let right_eval_idx = self.instructions.len();
                    self.patch_jump(jmpz_idx, right_eval_idx);

                    let r_right = self.compile_expr(env, right);
                    self.emit(Instruction {
                        op: OpCode::Move,
                        a: r_dest,
                        b: r_right,
                        c: 0,
                    });

                    let end_idx = self.instructions.len();
                    self.patch_jump(jmp_end_idx, end_idx);

                    return r_dest;
                }

                let r_left = self.compile_expr(env, left);
                let r_right = self.compile_expr(env, right);
                let r_dest = env.alloc_reg();

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

                    _ => unreachable!("Logical ops already handled"),
                };

                self.emit(Instruction {
                    op: opcode,
                    a: r_dest,
                    b: r_left,
                    c: r_right,
                });
                r_dest
            }

            IrExprKind::Call { func_name, args } => {
                if func_name == "print" && args.len() == 1 {
                    let arg_reg = self.compile_expr(env, &args[0]);

                    let print_op = if matches!(args[0].ty, IrType::F32) {
                        OpCode::PrintF
                    } else {
                        OpCode::PrintI
                    };

                    self.emit(Instruction {
                        op: print_op,
                        a: arg_reg,
                        b: 0,
                        c: 0,
                    });

                    let dummy_reg = env.alloc_reg();
                    let zero_idx = self.add_const(0);
                    self.emit(Instruction {
                        op: OpCode::LoadConst,
                        a: dummy_reg,
                        b: zero_idx,
                        c: 0,
                    });
                    return dummy_reg;
                }

                let mut arg_regs = Vec::new();
                for arg in args {
                    arg_regs.push(self.compile_expr(env, arg));
                }

                let frame_offset = env.next_reg;

                for (i, r_arg) in arg_regs.into_iter().enumerate() {
                    let target_arg_reg = frame_offset + (i as u8);
                    if target_arg_reg >= env.next_reg {
                        env.next_reg = target_arg_reg + 1;
                    }
                    self.emit(Instruction {
                        op: OpCode::Move,
                        a: target_arg_reg,
                        b: r_arg,
                        c: 0,
                    });
                }

                let r_addr = if let Some(&reg) = env.vars.get(func_name) {
                    reg
                } else if let Some(&func_const_idx) = self.func_const_indices.get(func_name) {
                    let reg = env.alloc_reg();
                    self.emit(Instruction {
                        op: OpCode::LoadConst,
                        a: reg,
                        b: func_const_idx,
                        c: 0,
                    });
                    reg
                } else {
                    panic!("Function '{}' not found", func_name);
                };

                let r_dest = env.alloc_reg();
                self.emit(Instruction {
                    op: OpCode::Call,
                    a: r_dest,
                    b: r_addr,
                    c: frame_offset,
                });

                r_dest
            }
        }
    }
}
