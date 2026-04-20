use abyss_ir::ir::{IrExpr, IrFunction, IrProgram, IrType};
use inkwell::OptimizationLevel;
use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::passes::PassBuilderOptions;
use inkwell::targets::{
    CodeModel, FileType, InitializationConfig, RelocMode, Target, TargetMachine,
};
use inkwell::types::BasicMetadataTypeEnum;
use inkwell::types::BasicType;
use inkwell::values::BasicValueEnum;
use inkwell::values::PointerValue;
use std::collections::HashMap;
use std::path::Path;

mod expr;
mod stmt;
mod types;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OptLevel {
    O0,
    O1,
    O2,
    O3,
    Os,
    Oz,
}

impl OptLevel {
    pub fn to_inkwell(self) -> OptimizationLevel {
        match self {
            OptLevel::O0 => OptimizationLevel::None,
            OptLevel::O1 => OptimizationLevel::Less,
            OptLevel::O2 => OptimizationLevel::Default,
            OptLevel::O3 => OptimizationLevel::Aggressive,
            OptLevel::Os | OptLevel::Oz => OptimizationLevel::Default,
        }
    }
}

pub struct AbyssCompiler<'ctx> {
    pub context: &'ctx Context,
    pub module: Module<'ctx>,
    pub builder: Builder<'ctx>,
    variables: HashMap<String, PointerValue<'ctx>>,
    loop_targets: Vec<inkwell::basic_block::BasicBlock<'ctx>>,
    pub opt_level: OptLevel,
}

impl<'ctx> AbyssCompiler<'ctx> {
    pub fn new(context: &'ctx Context, module_name: &str) -> Self {
        Self {
            context,
            module: context.create_module(module_name),
            builder: context.create_builder(),
            variables: HashMap::new(),
            loop_targets: Vec::new(),
            opt_level: OptLevel::O0,
        }
    }

    pub fn set_opt_level(&mut self, level: OptLevel) {
        self.opt_level = level;
    }

    pub fn optimize_module(&self) {
        if self.opt_level == OptLevel::O0 {
            return;
        }

        let pass_string = match self.opt_level {
            OptLevel::O0 => return,
            OptLevel::O1 => "default<O1>",
            OptLevel::O2 => "default<O2>",
            OptLevel::O3 => "default<O3>",
            OptLevel::Os => "default<Os>",
            OptLevel::Oz => "default<Oz>",
        };

        Target::initialize_native(&InitializationConfig::default()).unwrap();
        let target_triple = TargetMachine::get_default_triple();
        let target = Target::from_triple(&target_triple).unwrap();

        let target_machine = target
            .create_target_machine(
                &target_triple,
                "generic",
                "",
                self.opt_level.to_inkwell(),
                RelocMode::Default,
                CodeModel::Default,
            )
            .unwrap();

        let pass_options = PassBuilderOptions::create();

        if let Err(err) = self
            .module
            .run_passes(pass_string, &target_machine, pass_options)
        {
            eprintln!("LLVM Optimization Error: {}", err);
        }
    }

    pub fn run_jit(
        &self,
        execution_engine: &inkwell::execution_engine::ExecutionEngine<'ctx>,
        function_name: &str,
    ) -> Result<i32, String> {
        type MainFunc = unsafe extern "C" fn() -> i32;

        unsafe {
            let jit_function = execution_engine
                .get_function::<MainFunc>(function_name)
                .map_err(|e| format!("Failed to find function '{}': {}", function_name, e))?;

            let result = jit_function.call();
            Ok(result)
        }
    }
    pub fn compile_ir(&mut self, program: &IrProgram) -> Result<(), String> {
        for func in &program.functions {
            self.declare_function(func);
        }

        self.compile_globals(&program.globals);

        for func in &program.functions {
            if func.body.is_some() {
                self.compile_function(func);
            }
        }

        self.module.verify().map_err(|e| e.to_string())
    }

    pub fn execute_jit(
        &self,
        function_name: &str,
        ffi_bindings: &[(&str, usize)],
    ) -> Result<i32, String> {
        Target::initialize_native(&InitializationConfig::default()).unwrap();

        self.optimize_module();

        let execution_engine = self
            .module
            .create_jit_execution_engine(self.opt_level.to_inkwell())
            .map_err(|e| format!("Failed to create JIT engine: {}", e))?;

        for (name, func_ptr) in ffi_bindings {
            if let Some(ext_func) = self.module.get_function(name) {
                execution_engine.add_global_mapping(&ext_func, *func_ptr);
            }
        }

        type MainFunc = unsafe extern "C" fn() -> i32;

        unsafe {
            let jit_function = execution_engine
                .get_function::<MainFunc>(function_name)
                .map_err(|e| format!("Failed to find entry function '{}': {}", function_name, e))?;

            Ok(jit_function.call())
        }
    }

    pub fn compile_globals(&mut self, globals: &[(String, IrType, IrExpr)]) {
        for (name, ty, expr) in globals {
            let ll_ty = self.compile_type(ty);
            let global_var = self.module.add_global(ll_ty, None, name);

            let mut initializer: BasicValueEnum = ll_ty.const_zero();

            match &expr.kind {
                abyss_ir::ir::IrExprKind::Lit(lit) => match lit {
                    abyss_ir::ir::IrLit::Int(n) => {
                        if ll_ty.is_int_type() {
                            initializer = ll_ty.into_int_type().const_int(*n as u64, true).into();
                        }
                    }
                    abyss_ir::ir::IrLit::Float(f) => {
                        if ll_ty.is_float_type() {
                            initializer = ll_ty.into_float_type().const_float(*f).into();
                        }
                    }
                    abyss_ir::ir::IrLit::Bool(b) => {
                        if ll_ty.is_int_type() {
                            initializer = ll_ty
                                .into_int_type()
                                .const_int(if *b { 1 } else { 0 }, false)
                                .into();
                        }
                    }
                },

                abyss_ir::ir::IrExprKind::ArrayInit(items) => {
                    if ll_ty.is_array_type() {
                        let array_ty = ll_ty.into_array_type();
                        let element_ty = array_ty.get_element_type();

                        if element_ty.is_int_type() {
                            let int_ty = element_ty.into_int_type();
                            let mut const_vals = Vec::new();

                            for item in items {
                                if let abyss_ir::ir::IrExprKind::Lit(lit) = &item.kind {
                                    match lit {
                                        abyss_ir::ir::IrLit::Int(n) => {
                                            const_vals.push(int_ty.const_int(*n as u64, false));
                                        }
                                        abyss_ir::ir::IrLit::Bool(b) => {
                                            const_vals.push(
                                                int_ty.const_int(if *b { 1 } else { 0 }, false),
                                            );
                                        }
                                        _ => const_vals.push(int_ty.const_zero()),
                                    }
                                } else {
                                    const_vals.push(int_ty.const_zero());
                                }
                            }
                            initializer = int_ty.const_array(&const_vals).into();
                        } else if element_ty.is_float_type() {
                            let float_ty = element_ty.into_float_type();
                            let mut const_vals = Vec::new();

                            for item in items {
                                if let abyss_ir::ir::IrExprKind::Lit(abyss_ir::ir::IrLit::Float(
                                    f,
                                )) = &item.kind
                                {
                                    const_vals.push(float_ty.const_float(*f));
                                } else {
                                    const_vals.push(float_ty.const_zero());
                                }
                            }
                            initializer = float_ty.const_array(&const_vals).into();
                        } else {
                            initializer = array_ty.const_zero().into();
                        }
                    }
                }
                _ => {}
            }

            global_var.set_initializer(&initializer);
        }
    }

    pub fn compile_program(&mut self, program: &IrProgram, output_path: &Path) {
        self.compile_globals(&program.globals);

        for func in &program.functions {
            self.declare_function(func);
        }

        for func in &program.functions {
            if func.body.is_some() {
                self.compile_function(func);
            }
        }

        self.optimize_module();

        Target::initialize_all(&InitializationConfig::default());
        let target_triple = TargetMachine::get_default_triple();
        let target = Target::from_triple(&target_triple).unwrap();

        let target_machine = target
            .create_target_machine(
                &target_triple,
                "generic",
                "",
                self.opt_level.to_inkwell(),
                RelocMode::Default,
                CodeModel::Default,
            )
            .unwrap();

        target_machine
            .write_to_file(&self.module, FileType::Object, output_path)
            .unwrap();
    }

    pub fn declare_function(&mut self, func: &IrFunction) {
        let param_types: Vec<BasicMetadataTypeEnum> = func
            .params
            .iter()
            .map(|(_, ty)| self.compile_type(ty).into())
            .collect();

        let fn_type = if let IrType::Unit = func.return_ty {
            self.context.void_type().fn_type(&param_types, false)
        } else {
            self.compile_type(&func.return_ty)
                .fn_type(&param_types, false)
        };

        self.module.add_function(&func.name, fn_type, None);
    }

    pub fn compile_function(&mut self, func: &IrFunction) {
        let function = self.module.get_function(&func.name).unwrap();
        let basic_block = self.context.append_basic_block(function, "entry");
        self.builder.position_at_end(basic_block);
        self.variables.clear();

        for (i, (name, ty)) in func.params.iter().enumerate() {
            let param_value = function.get_nth_param(i as u32).unwrap();
            let param_type = self.compile_type(ty);
            let alloca = self.builder.build_alloca(param_type, name).unwrap();
            self.builder.build_store(alloca, param_value).unwrap();
            self.variables.insert(name.clone(), alloca);
        }

        if let Some(body) = &func.body {
            for stmt in body {
                self.compile_stmt(stmt, function);
            }
        }

        let current_bb = self.builder.get_insert_block().unwrap();

        if current_bb.get_terminator().is_none() {
            match func.return_ty {
                IrType::Unit => {
                    self.builder.build_return(None).unwrap();
                }
                IrType::I32 => {
                    let zero = self.context.i32_type().const_zero();
                    self.builder.build_return(Some(&zero)).unwrap();
                }
                _ => {}
            }
        }
    }
}
