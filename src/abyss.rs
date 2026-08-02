use std::{println, rc::Rc};

use abyss_analyzer::type_checker::engine::TypeChecker;
use abyss_diagnostics::DiagnosticEngine;
use abyss_ir::builder::IrBuilder;
use abyss_lower::HirLowerer;
use abyss_nexus::nexus::{FileId, Nexus};
use abyss_parser::parser::Parser;
use abyss_typer::collector;
use abyss_utils::idgen::IdGenerator;
use abyss_vm::{codegen::IrCompiler, vm::core::AbyssVm};

#[cfg(feature = "llvm")]
use abyss_codegen::llvm_codegen::{AbyssCompiler, OptLevel};

pub struct ExecutionResult {
    pub diagnostics: String,
    pub stdout: String,
    pub tast_output: String,
    pub asm_output: String,
}

pub type HostFn = Rc<dyn Fn(&[u64], &mut [u8]) -> u64>;

struct RegisteredHostFunction {
    name: String,
    arity: usize,
    is_pointer: Vec<bool>,
    func: HostFn,
}

pub struct Abyss {
    source_code: String,
    filename: String,
    print_tast: bool,
    host_functions: Vec<RegisteredHostFunction>,
}

impl Abyss {
    pub fn new(source_code: impl Into<String>) -> Self {
        Self {
            source_code: source_code.into(),
            filename: "main.a".to_string(),

            print_tast: true,
            host_functions: Vec::new(),
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

    pub fn with_host_function(
        mut self,
        name: &str,
        arity: usize,
        is_pointer: Vec<bool>,
        func: impl Fn(&[u64], &mut [u8]) -> u64 + 'static,
    ) -> Self {
        self.host_functions.push(RegisteredHostFunction {
            name: name.to_string(),
            arity,
            is_pointer,
            func: std::rc::Rc::new(func),
        });
        self
    }

    fn run_core(&mut self) -> ExecutionResult {
        let mut err = DiagnosticEngine::new();
        err.add_source(0, self.filename.clone(), self.source_code.clone());
        let mut idgen = IdGenerator::new();

        let mut parser = Parser::new(&self.source_code, &mut err, &mut idgen, 0);
        let program = parser.parse_program();
        println!("{:?}", program);

        // init database
        let mut nexus = Nexus::new();

        // Lower the program to HIR
        let lowerer = HirLowerer::new(&mut nexus, FileId(0));
        let hir = lowerer.lower_program(&program);
        nexus.set_hir(hir);
        nexus.hir.print_dump(&nexus);

        // collect symbols
        collector::collect(&mut nexus);
        dbg!(nexus.symbols.get_span(0));
        dbg!(nexus.symbols.get_span(1));
        dbg!(nexus.symbols.get_span(2));

        // Typecheck the program
        //let types = tyck::check(&nexus.hir.table);
        //println!("-------------------");
        //println!("{:#?}", types.iter().enumerate().collect::<Vec<_>>());

        let mut type_checker = TypeChecker::new(&mut err, &mut idgen);

        for hf in &self.host_functions {
            type_checker.comptime.vm.register_host_function(
                &hf.name,
                hf.arity,
                hf.is_pointer.clone(),
                hf.func.clone(),
            );
        }

        let tast = type_checker.check_program(&program);

        let tast_output_str = tast.format_tree();
        if self.print_tast {
            println!("{}", tast_output_str)
        }

        let error_output = err.render();
        if !error_output.is_empty() {
            return ExecutionResult {
                diagnostics: error_output,
                stdout: String::new(),
                tast_output: tast_output_str,
                asm_output: String::new(),
            };
        }

        let mut cmp = IrBuilder::new();
        let ir_program = cmp.build_program(tast);

        let compiler = IrCompiler::new();
        let (instructions, constants, imports) = compiler.compile(&ir_program);

        let mut vm = AbyssVm::new(instructions, constants);

        while let Some(hf) = self.host_functions.pop() {
            vm.register_host_function(&hf.name, hf.arity, hf.is_pointer, hf.func);
        }

        vm.load_imports(&imports);
        vm.init_globals(ir_program.globals.len());

        vm.run();

        let asm_output_str = vm.disassemble();

        ExecutionResult {
            diagnostics: String::new(),
            stdout: vm.out.clone(),
            tast_output: tast_output_str,
            asm_output: asm_output_str,
        }
    }

    pub fn run(&mut self) {
        let result = self.run_core();

        if !result.diagnostics.is_empty() {
            println!("{}", result.diagnostics);
            return;
        }
    }

    pub fn run_for_test(&mut self) -> ExecutionResult {
        let result = self.run_core();
        result
    }

    #[cfg(feature = "llvm")]
    pub fn run_llvm_jit(&self) {
        use std::time::Instant;

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

        let native_bindings: &[(&str, usize)] = &[];

        match llvm_compiler.execute_jit("main", native_bindings) {
            Ok(_result) => {}
            Err(e) => eprintln!("\nJIT Error: {}", e),
        }

        println!("Executed via JIT in: {}ms", t_execute.elapsed().as_millis());
    }
}
