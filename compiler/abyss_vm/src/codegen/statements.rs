use super::IrCompiler;
use super::env::Env;
use crate::{Instruction, OpCode};
use abyss_ir::ir::{IrExpr, IrStmt};

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
                let base_reg = self.compile_expr(env, base, None);

                let val_reg = self.compile_expr(env, val, None);

                let index_reg = self.compile_expr(env, index, None);

                self.emit(Instruction {
                    op: OpCode::StorePtrOffset,
                    a: base_reg,
                    b: val_reg,
                    c: index_reg,
                });
            }
        }
    }

    fn compile_var_dec(&mut self, env: &mut Env, name: &String, init: &Option<IrExpr>) {
        let dest_reg = env.declare_var(name.clone());
        if let Some(expr) = init {
            self.compile_expr(env, expr, Some(dest_reg));
        }
    }
    fn compile_const_def(&mut self, env: &mut Env, name: &String, value: &IrExpr) {
        let dest_reg = env.declare_var(name.clone());
        self.compile_expr(env, value, Some(dest_reg));
    }

    fn compile_assign(&mut self, env: &mut Env, target: &str, val: &IrExpr) {
        let dest_reg = env.get_var(target);
        self.compile_expr(env, val, Some(dest_reg));
    }

    fn compile_return(&mut self, env: &mut Env, opt_expr: &Option<IrExpr>) {
        let ret_reg = match opt_expr {
            Some(expr) => self.compile_expr(env, expr, None),
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
}
