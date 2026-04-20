use crate::vm::opcode::{Instruction, OpCode};

use super::IrCompiler;
use super::env::Env;
use abyss_ir::ir::{IrExpr, IrStmt, IrType};

impl IrCompiler {
    pub(crate) fn compile_stmt(&mut self, env: &mut Env, stmt: &IrStmt) {
        match stmt {
            IrStmt::VarDec { name, init, .. } => self.compile_var_dec(env, name, init),
            IrStmt::ConstDef { name, value, .. } => self.compile_const_def(env, name, value),
            IrStmt::Assign { target, val } => self.compile_assign(env, target, val),
            IrStmt::Expr(expr) => {
                self.compile_expr(env, expr, None);
            }
            IrStmt::Return(opt_expr) => self.compile_return(env, opt_expr),
            IrStmt::If(cond, then_b, else_b) => self.compile_if(env, cond, then_b, else_b),
            IrStmt::While { cond, body } => self.compile_while(env, cond, body),
            IrStmt::Break => self.compile_break(),

            IrStmt::WriteIndex { base, index, val } => {
                self.compile_write_index(env, base, index, val)
            }

            IrStmt::WriteField { base, index, val } => {
                self.compile_write_field(env, base, *index, val)
            }

            IrStmt::WritePointer { ptr, val } => self.compile_write_pointer(env, ptr, val),
        }
    }

    fn compile_var_dec(&mut self, env: &mut Env, name: &String, init: &Option<IrExpr>) {
        let dest_reg = env.declare_var(name.clone());
        if let Some(expr) = init {
            let val_reg = self.compile_expr(env, expr, None);
            let final_val_reg = self.copy_if_complex(env, val_reg, &expr.ty);

            self.emit(Instruction {
                op: OpCode::Move,
                a: dest_reg,
                b: final_val_reg,
                c: 0,
            });
        }
    }
    fn compile_const_def(&mut self, env: &mut Env, name: &String, value: &IrExpr) {
        let dest_reg = env.declare_var(name.clone());
        let val_reg = self.compile_expr(env, value, None);
        let final_val_reg = self.copy_if_complex(env, val_reg, &value.ty);

        self.emit(Instruction {
            op: OpCode::Move,
            a: dest_reg,
            b: final_val_reg,
            c: 0,
        });
    }

    fn compile_assign(&mut self, env: &mut Env, target: &str, val: &IrExpr) {
        let val_reg = self.compile_expr(env, val, None);
        let final_val_reg = self.copy_if_complex(env, val_reg, &val.ty);

        if let Some(&local_reg) = env.vars.get(target) {
            self.emit(Instruction {
                op: OpCode::Move,
                a: local_reg,
                b: final_val_reg,
                c: 0,
            });
        } else if let Some(&global_idx) = self.global_indices.get(target) {
            self.emit(Instruction {
                op: OpCode::StoreGlobal,
                a: final_val_reg,
                b: ((global_idx >> 8) & 0xFF) as u8,
                c: (global_idx & 0xFF) as u8,
            });
        } else {
            panic!("Cannot assign to undeclared variable '{}'", target);
        }
    }

    fn compile_return(&mut self, env: &mut Env, opt_expr: &Option<IrExpr>) {
        let ret_reg = match opt_expr {
            Some(expr) => {
                let val_reg = self.compile_expr(env, expr, None);
                self.copy_if_complex(env, val_reg, &expr.ty)
            }
            None => self.emit_load_zero(env),
        };
        self.emit(Instruction {
            op: OpCode::Ret,
            a: ret_reg,
            b: 0,
            c: 0,
        });
    }

    fn compile_if(&mut self, env: &mut Env, cond: &IrExpr, then_b: &[IrStmt], else_b: &[IrStmt]) {
        let cond_reg = self.compile_expr(env, cond, None);
        let jmpz_idx = self.instructions.len();
        self.emit(Instruction {
            op: OpCode::JmpZImm,
            a: cond_reg,
            b: 0,
            c: 0,
        });

        let vars_backup = env.vars.clone();
        for stmt in then_b {
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
        for stmt in else_b {
            self.compile_stmt(env, stmt);
        }

        let end_idx = self.instructions.len();
        self.patch_jump(jmp_end_idx, end_idx);
        env.vars = vars_backup;
    }

    fn compile_while(&mut self, env: &mut Env, cond: &IrExpr, body: &[IrStmt]) {
        let loop_start_idx = self.instructions.len();
        let cond_reg = self.compile_expr(env, cond, None);

        let jmpz_idx = self.instructions.len();
        self.emit(Instruction {
            op: OpCode::JmpZImm,
            a: cond_reg,
            b: 0,
            c: 0,
        });

        self.break_targets.push(Vec::new());
        let vars_backup = env.vars.clone();

        for stmt in body {
            self.compile_stmt(env, stmt);
        }

        let jmp_back_idx = self.instructions.len();
        self.emit(Instruction {
            op: OpCode::JmpImm,
            a: 0,
            b: 0,
            c: 0,
        });
        self.patch_jump(jmp_back_idx, loop_start_idx);

        let loop_end_idx = self.instructions.len();
        self.patch_jump(jmpz_idx, loop_end_idx);

        let breaks = self.break_targets.pop().unwrap();
        for break_idx in breaks {
            self.patch_jump(break_idx, loop_end_idx);
        }
        env.vars = vars_backup;
    }

    fn compile_break(&mut self) {
        let break_idx = self.instructions.len();
        self.emit(Instruction {
            op: OpCode::JmpImm,
            a: 0,
            b: 0,
            c: 0,
        });

        if let Some(current_loop_breaks) = self.break_targets.last_mut() {
            current_loop_breaks.push(break_idx);
        } else {
            panic!("Compiler Error: 'Break' outside of a loop!");
        }
    }

    fn compile_write_index(&mut self, env: &mut Env, base: &IrExpr, index: &IrExpr, val: &IrExpr) {
        let base_reg = self.compile_expr(env, base, None);
        let val_reg = self.compile_expr(env, val, None);

        let final_val_reg = self.copy_if_complex(env, val_reg, &val.ty);
        let index_reg = self.compile_expr(env, index, None);

        let elem_ty = match &base.ty {
            IrType::Array(inner, _) => &**inner,
            _ => &val.ty,
        };
        let (_, _, _, _, store_offset_op) = self.get_type_info(elem_ty);

        self.emit(Instruction {
            op: store_offset_op,
            a: base_reg,
            b: final_val_reg,
            c: index_reg,
        });
    }

    fn compile_write_field(&mut self, env: &mut Env, base: &IrExpr, index: usize, val: &IrExpr) {
        let base_reg = self.compile_expr(env, base, None);
        let val_reg = self.compile_expr(env, val, None);

        let final_val_reg = self.copy_if_complex(env, val_reg, &val.ty);

        let idx_const = self.add_const(index as u64);
        let idx_reg = env.alloc_reg();
        self.emit(Instruction {
            op: OpCode::LoadConst,
            a: idx_reg,
            b: idx_const,
            c: 0,
        });

        self.emit(Instruction {
            op: OpCode::StorePtrOffset,
            a: base_reg,
            b: final_val_reg,
            c: idx_reg,
        });
    }

    fn compile_write_pointer(&mut self, env: &mut Env, ptr: &IrExpr, val: &IrExpr) {
        let ptr_reg = self.compile_expr(env, ptr, None);
        let val_reg = self.compile_expr(env, val, None);

        let final_val_reg = self.copy_if_complex(env, val_reg, &val.ty);
        let zero_reg = self.emit_load_zero(env);

        self.emit(Instruction {
            op: OpCode::StorePtrOffset,
            a: ptr_reg,
            b: final_val_reg,
            c: zero_reg,
        });
    }
}
