use std::time::Instant;

use abyss_analyzer::type_checker::engine::TypeChecker;
use abyss_codegen::llvm_codegen::{AbyssCompiler, OptLevel};
use abyss_diagnostics::DiagnosticEngine;
use abyss_ir::builder::IrBuilder;
use abyss_parser::parser::Parser;
use abyss_utils::idgen::IdGenerator;
use abyss_vm::{codegen::IrCompiler, vm::core::AbyssVm};

#[unsafe(no_mangle)]
pub extern "C" fn abyss_jit_print(ptr: i32) {
    println!("{}", ptr);
}

pub struct ExecutionResult {
    pub diagnostics: String,
    pub stdout: String,
}

pub struct Abyss {
    source_code: String,
    filename: String,
    print_tast: bool,
}

impl Abyss {
    pub fn new(source_code: impl Into<String>) -> Self {
        Self {
            source_code: source_code.into(),
            filename: "main.a".to_string(),

            print_tast: true,
        }
    }

    pub fn with_filename(mut self, filename: &str) -> Self {
        self.filename = filename.to_string();
        self
    }

    pub fn disable_tast_print(mut self) -> Self {
        self.print_tast = false;
        self
    }

    fn run_core(&self) -> (ExecutionResult, u128, u128) {
        let t_compile = Instant::now();

        let mut err = DiagnosticEngine::new();
        err.add_source(0, self.filename.clone(), self.source_code.clone());
        let mut idgen = IdGenerator::new();

        let mut parser = Parser::new(&self.source_code, &mut err, &mut idgen, 0);
        let program = parser.parse_program();

        let mut type_checker = TypeChecker::new(&mut err, &mut idgen);

        let tast = type_checker.check_program(&program);

        if self.print_tast {
            tast.print_tree();
            println!();
        }

        let error_output = err.render();
        if !error_output.is_empty() {
            return (
                ExecutionResult {
                    diagnostics: error_output,
                    stdout: String::new(),
                },
                t_compile.elapsed().as_millis(),
                0,
            );
        }

        let mut cmp = IrBuilder::new();
        let ir_program = cmp.build_program(tast);

        let compiler = IrCompiler::new();

        let (instructions, constants, imports) = compiler.compile(&ir_program);

        let mut vm = AbyssVm::new(instructions, constants);

        vm.load_imports(&imports);

        vm.init_globals(ir_program.globals.len());

        let compile_time = t_compile.elapsed().as_millis();

        let t_execute = Instant::now();
        vm.run();
        let execute_time = t_execute.elapsed().as_millis();

        (
            ExecutionResult {
                diagnostics: String::new(),
                stdout: vm.out.clone(),
            },
            compile_time,
            execute_time,
        )
    }

    pub fn run(&self) {
        let (result, _, execute_time) = self.run_core();

        if !result.diagnostics.is_empty() {
            println!("{}", result.diagnostics);
            return;
        }

        println!("\nExecuted in: {}ms", execute_time);
    }

    pub fn run_for_test(&self) -> ExecutionResult {
        let (result, _, _) = self.run_core();
        result
    }

    pub fn run_llvm_jit(&self) {
        let t_compile = Instant::now();

        let mut err = DiagnosticEngine::new();
        err.add_source(0, self.filename.clone(), self.source_code.clone());
        let mut idgen = IdGenerator::new();

        let mut parser = Parser::new(&self.source_code, &mut err, &mut idgen, 0);
        let program = parser.parse_program();

        let mut type_checker = TypeChecker::new(&mut err, &mut idgen);

        let tast = type_checker.check_program(&program);

        if self.print_tast {
            tast.print_tree();
            println!();
        }

        let error_output = err.render();
        if !error_output.is_empty() {
            println!("{}", error_output);
            return;
        }

        let mut cmp = IrBuilder::new();
        let ir_program = cmp.build_program(tast);

        println!(
            "Frontend & IR built in: {}ms\n",
            t_compile.elapsed().as_millis()
        );

        let t_llvm = Instant::now();

        let context = inkwell::context::Context::create();
        let mut llvm_compiler = AbyssCompiler::new(&context, "abyss_jit_module");

        llvm_compiler.set_opt_level(OptLevel::O3);

        if let Err(e) = llvm_compiler.compile_ir(&ir_program) {
            eprintln!("LLVM Verification Error:\n{}", e);
            return;
        }

        println!(
            "LLVM Module compiled in: {}ms",
            t_llvm.elapsed().as_millis()
        );

        let t_execute = Instant::now();

        let native_bindings: &[(&str, usize)] = &[("print", abyss_jit_print as usize)];

        match llvm_compiler.execute_jit("main", native_bindings) {
            Ok(_result) => {}
            Err(e) => eprintln!("\nJIT Error: {}", e),
        }

        println!("Executed via JIT in: {}ms", t_execute.elapsed().as_millis());
    }
}
