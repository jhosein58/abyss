pub mod env;
pub mod expressions;
pub mod statements;

use crate::{Instruction, OpCode};
use abyss_ir::ir::IrProgram;
use env::Env;
use std::collections::HashMap;

pub struct IrCompiler {
    pub instructions: Vec<Instruction>,
    pub constants: Vec<u64>,
    pub func_const_indices: HashMap<String, u8>,
    pub break_targets: Vec<Vec<usize>>,
}

impl IrCompiler {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
            constants: Vec::new(),
            func_const_indices: HashMap::new(),
            break_targets: Vec::new(),
        }
    }

    pub(crate) fn emit(&mut self, inst: Instruction) {
        self.instructions.push(inst);
    }

    pub(crate) fn patch_jump(&mut self, inst_idx: usize, target_idx: usize) {
        if target_idx > 0xFFFF {
            panic!("Jump target address too large!");
        }
        let inst = &mut self.instructions[inst_idx];
        inst.b = ((target_idx >> 8) & 0xFF) as u8;
        inst.c = (target_idx & 0xFF) as u8;
    }

    pub(crate) fn add_const(&mut self, val: u64) -> u8 {
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

    pub(crate) fn emit_load_zero(&mut self, env: &mut Env) -> u8 {
        let r = env.alloc_reg();
        let zero_idx = self.add_const(0);
        self.emit(Instruction {
            op: OpCode::LoadConst,
            a: r,
            b: zero_idx,
            c: 0,
        });
        r
    }

    pub fn compile(mut self, program: &IrProgram) -> (Vec<Instruction>, Vec<u64>) {
        for func in &program.functions {
            if !func.is_native {
                let idx = self.constants.len() as u8;
                self.constants.push(0);
                self.func_const_indices.insert(func.name.clone(), idx);
            }
        }

        if let Some(main_const_idx) = self.func_const_indices.get("main") {
            self.emit(Instruction {
                op: OpCode::LoadConst,
                a: 0,
                b: *main_const_idx,
                c: 0,
            });
            self.emit(Instruction {
                op: OpCode::Call,
                a: 1,
                b: 0,
                c: 2,
            });
            self.emit(Instruction {
                op: OpCode::Ret,
                a: 1,
                b: 0,
                c: 0,
            });
        } else {
            panic!("Program must have a 'main' function.");
        }

        for func in &program.functions {
            if func.is_native {
                continue;
            }

            let func_ip = self.instructions.len() as u64;
            let const_idx = self.func_const_indices[&func.name];
            self.constants[const_idx as usize] = func_ip;

            let mut env = Env::new();
            for (param_name, _) in &func.params {
                env.declare_var(param_name.clone());
            }

            for stmt in &func.body {
                self.compile_stmt(&mut env, stmt);
            }

            let r_dummy = self.emit_load_zero(&mut env);
            self.emit(Instruction {
                op: OpCode::Ret,
                a: r_dummy,
                b: 0,
                c: 0,
            });
        }

        (self.instructions, self.constants)
    }
}
