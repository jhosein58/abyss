use std::time::Instant;

use abyss_analyzer::type_checker::engine::TypeChecker;
use abyss_diagnostics::DiagnosticEngine;
use abyss_ir::builder::IrBuilder;
use abyss_parser::parser::Parser;
use abyss_std::{interface::AbyssLibrary, stdlib::StandardLib};
use abyss_utils::idgen::IdGenerator;
use abyss_vm::{
    codegen::IrCompiler,
    vm::{core::AbyssVm, types::NativeFunction},
};

pub struct ExecutionResult {
    pub diagnostics: String,
    pub stdout: String,
}

pub struct Abyss {
    source_code: String,
    filename: String,
    natives: Vec<(String, usize, NativeFunction)>,
    print_tast: bool,
}

impl Abyss {
    pub fn new(source_code: impl Into<String>) -> Self {
        let mut abyss = Self {
            source_code: source_code.into(),
            filename: "main.a".to_string(),
            natives: Vec::new(),
            print_tast: true,
        };
        abyss = abyss.register_library::<StandardLib>();
        abyss
    }

    pub fn with_filename(mut self, filename: &str) -> Self {
        self.filename = filename.to_string();
        self
    }

    pub fn disable_tast_print(mut self) -> Self {
        self.print_tast = false;
        self
    }

    pub fn register_native(mut self, name: &str, arity: usize, func: NativeFunction) -> Self {
        self.natives.push((name.to_string(), arity, func));
        self
    }

    pub fn register_library<L: AbyssLibrary>(mut self) -> Self {
        for def in L::get_functions() {
            self = self.register_native(def.name, def.arity, def.func);
        }
        self
    }

    pub fn run(&self) {
        let t_compile = Instant::now();

        let mut err = DiagnosticEngine::new();
        err.add_source(0, self.filename.clone(), self.source_code.clone());
        let mut idgen = IdGenerator::new();

        let mut parser = Parser::new(&self.source_code, &mut err, &mut idgen, 0);
        let program = parser.parse_program();

        let mut type_checker = TypeChecker::new(&mut err, &mut idgen);
        for (index, (name, arity, func)) in self.natives.iter().enumerate() {
            type_checker
                .comptime
                .register_native_with_index(name, index, *arity as u8, *func);
        }

        let tast = type_checker.check_program(&program);

        if self.print_tast {
            tast.body.print_tree();
            println!();
        }

        let error_output = err.render();
        if !error_output.is_empty() {
            println!("{}", error_output);
        }

        let mut cmp = IrBuilder::new();
        for (index, (name, _arity, _func)) in self.natives.iter().enumerate() {
            cmp.register_native(name, index);
        }
        let ir_program = cmp.build_program(tast);

        let compiler = IrCompiler::new();
        let (instructions, constants) = compiler.compile(&ir_program);

        let mut vm = AbyssVm::new(instructions, constants);
        for (_name, arity, func) in self.natives.iter() {
            vm.register_native(*arity as u8, *func);
        }
        vm.init_globals(ir_program.globals.len());

        println!("Compiled in: {}ms\n", t_compile.elapsed().as_millis());

        let t_execute = Instant::now();
        vm.run();

        println!("\nExecuted in: {}ms", t_execute.elapsed().as_millis());
    }

    pub fn run_for_test(&self) -> ExecutionResult {
        let mut err = DiagnosticEngine::new();
        err.add_source(0, self.filename.clone(), self.source_code.clone());
        let mut idgen = IdGenerator::new();

        let mut parser = Parser::new(&self.source_code, &mut err, &mut idgen, 0);
        let program = parser.parse_program();

        let mut type_checker = TypeChecker::new(&mut err, &mut idgen);
        for (index, (name, arity, func)) in self.natives.iter().enumerate() {
            type_checker
                .comptime
                .register_native_with_index(name, index, *arity as u8, *func);
        }

        let tast = type_checker.check_program(&program);
        let error_output = err.render();

        if !error_output.is_empty() {
            return ExecutionResult {
                diagnostics: error_output,
                stdout: String::new(),
            };
        }

        let mut cmp = IrBuilder::new();
        for (index, (name, _arity, _func)) in self.natives.iter().enumerate() {
            cmp.register_native(name, index);
        }
        let ir_program = cmp.build_program(tast);

        let compiler = IrCompiler::new();
        let (instructions, constants) = compiler.compile(&ir_program);

        let mut vm = AbyssVm::new(instructions, constants);
        for (_name, arity, func) in self.natives.iter() {
            vm.register_native(*arity as u8, *func);
        }
        vm.init_globals(ir_program.globals.len());

        vm.run();

        ExecutionResult {
            diagnostics: String::new(),
            stdout: vm.out.clone(),
        }
    }
}
