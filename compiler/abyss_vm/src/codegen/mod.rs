pub mod env;
pub mod expressions;
pub mod statements;

use abyss_ir::ir::{IrProgram, IrType};
use env::Env;
use std::collections::HashMap;

use crate::vm::{
    opcode::{Instruction, OpCode},
    types::ExternDef,
};

pub struct IrCompiler {
    pub instructions: Vec<Instruction>,
    pub constants: Vec<u64>,
    pub func_const_indices: HashMap<String, u8>,
    pub global_indices: HashMap<String, u16>,
    pub break_targets: Vec<Vec<usize>>,
    pub extern_functions: Vec<ExternDef>,
    pub extern_indices: HashMap<String, usize>,
}

impl IrCompiler {
    pub fn new() -> Self {
        Self {
            instructions: Vec::new(),
            constants: Vec::new(),
            func_const_indices: HashMap::new(),
            global_indices: HashMap::new(),
            break_targets: Vec::new(),
            extern_functions: Vec::new(),
            extern_indices: HashMap::new(),
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

    pub fn register_extern(&mut self, name: &str, arg_types: Vec<IrType>, ret_type: IrType) {
        if !self.extern_indices.contains_key(name) {
            let idx = self.extern_functions.len();
            self.extern_functions.push(ExternDef {
                name: name.to_string(),
                arg_types,
                ret_type,
            });
            self.extern_indices.insert(name.to_string(), idx);
        }
    }

    pub fn compile(mut self, program: &IrProgram) -> (Vec<Instruction>, Vec<u64>, Vec<ExternDef>) {
        for (i, (name, _, _)) in program.globals.iter().enumerate() {
            self.global_indices.insert(name.clone(), i as u16);
        }

        for func in &program.functions {
            if func.body.is_none() {
                if !self.extern_indices.contains_key(&func.name) {
                    let idx = self.extern_functions.len();
                    let arg_types: Vec<IrType> =
                        func.params.iter().map(|(_, ty)| ty.clone()).collect();

                    self.extern_functions.push(ExternDef {
                        name: func.name.clone(),
                        arg_types,
                        ret_type: func.return_ty.clone(),
                    });
                    self.extern_indices.insert(func.name.clone(), idx);
                }
            } else {
                if !self.func_const_indices.contains_key(&func.name) {
                    let idx = self.constants.len() as u8;
                    self.constants.push(0);
                    self.func_const_indices.insert(func.name.clone(), idx);
                }
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
            let body = match &func.body {
                Some(b) => b,
                None => continue,
            };

            let func_ip = self.instructions.len() as u64;
            let const_idx = self.func_const_indices[&func.name];
            self.constants[const_idx as usize] = func_ip;

            let mut env = Env::new();

            if func.name == "main" {
                for (name, _, expr) in &program.globals {
                    let global_idx = self.global_indices[name];
                    let val_reg = self.compile_expr(&mut env, expr, None);

                    self.emit(Instruction {
                        op: OpCode::StoreGlobal,
                        a: val_reg,
                        b: ((global_idx >> 8) & 0xFF) as u8,
                        c: (global_idx & 0xFF) as u8,
                    });
                }
            }

            for (param_name, _) in &func.params {
                env.declare_var(param_name.clone());
            }

            for stmt in body {
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

        (self.instructions, self.constants, self.extern_functions)
    }

    pub fn compile_chunk(&mut self, program: &IrProgram, is_thunk: bool) -> usize {
        for (name, _, _) in &program.globals {
            if !self.global_indices.contains_key(name) {
                let idx = self.global_indices.len() as u16;
                self.global_indices.insert(name.clone(), idx);
            }
        }

        for func in &program.functions {
            if func.body.is_none() {
                if !self.extern_indices.contains_key(&func.name) {
                    let idx = self.extern_functions.len();
                    let arg_types: Vec<IrType> =
                        func.params.iter().map(|(_, ty)| ty.clone()).collect();

                    self.extern_functions.push(ExternDef {
                        name: func.name.clone(),
                        arg_types,
                        ret_type: func.return_ty.clone(),
                    });
                    self.extern_indices.insert(func.name.clone(), idx);
                }
            } else if !self.func_const_indices.contains_key(&func.name) {
                let idx = self.constants.len() as u8;
                self.constants.push(0);
                self.func_const_indices.insert(func.name.clone(), idx);
            }
        }
        let mut thunk_start_ip = 0;

        if is_thunk {
            if let Some(main_const_idx) = self.func_const_indices.get("thunk_main") {
                thunk_start_ip = self.instructions.len();
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
            }
        }

        for func in &program.functions {
            let body = match &func.body {
                Some(b) => b,
                None => continue,
            };

            let func_ip = self.instructions.len() as u64;
            let const_idx = self.func_const_indices[&func.name];
            self.constants[const_idx as usize] = func_ip;

            let mut env = Env::new();

            if func.name == "thunk_main" {
                for (name, _, expr) in &program.globals {
                    let global_idx = self.global_indices[name];
                    let val_reg = self.compile_expr(&mut env, expr, None);

                    self.emit(Instruction {
                        op: OpCode::StoreGlobal,
                        a: val_reg,
                        b: ((global_idx >> 8) & 0xFF) as u8,
                        c: (global_idx & 0xFF) as u8,
                    });
                }
            }

            for (param_name, _) in &func.params {
                env.declare_var(param_name.clone());
            }

            for stmt in body {
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

        thunk_start_ip
    }

    pub fn rewind(&mut self, inst_len: usize, const_len: usize) {
        self.instructions.truncate(inst_len);
        self.constants.truncate(const_len);
        self.func_const_indices.remove("thunk_main");
    }

    pub(crate) fn copy_if_complex(&mut self, env: &mut Env, src_reg: u8, ty: &IrType) -> u8 {
        match ty {
            IrType::Struct(_) | IrType::Array(_, _) => {
                let size_in_words = self.get_type_size_in_words(ty);

                let dest_ptr_reg = env.alloc_reg();
                let size_bytes_reg = env.alloc_reg();
                let count_reg = env.alloc_reg();

                let size_bytes = size_in_words * 8;
                let size_bytes_idx = self.add_const(size_bytes);
                self.emit(Instruction {
                    op: OpCode::LoadConst,
                    a: size_bytes_reg,
                    b: size_bytes_idx,
                    c: 0,
                });

                self.emit(Instruction {
                    op: OpCode::Alloc,
                    a: dest_ptr_reg,
                    b: size_bytes_reg,
                    c: 0,
                });

                let count_idx = self.add_const(size_in_words);
                self.emit(Instruction {
                    op: OpCode::LoadConst,
                    a: count_reg,
                    b: count_idx,
                    c: 0,
                });

                self.emit(Instruction {
                    op: OpCode::MemCopy,
                    a: dest_ptr_reg,
                    b: src_reg,
                    c: count_reg,
                });

                dest_ptr_reg
            }
            _ => src_reg,
        }
    }

    pub(crate) fn get_type_size_in_words(&self, ty: &IrType) -> u64 {
        match ty {
            IrType::I1
            | IrType::I8
            | IrType::I16
            | IrType::I32
            | IrType::I64
            | IrType::U8
            | IrType::U16
            | IrType::U32
            | IrType::U64
            | IrType::F32
            | IrType::F64
            | IrType::Bool
            | IrType::Ptr(_) => 1,

            IrType::Unit => 0,

            IrType::Array(inner_type, count) => {
                self.get_type_size_in_words(inner_type) * (*count as u64)
            }

            IrType::Struct(fields) => fields
                .iter()
                .map(|field_ty| self.get_type_size_in_words(field_ty))
                .sum(),
        }
    }
}
