#![warn(clippy::todo)]

use std::{fs, time::Instant};

use abyss_analyzer::type_checker::engine::TypeChecker;
//use abyss_analyzer::type_checker::engine::TypeChecker;
use abyss_diagnostics::DiagnosticEngine;
use abyss_ir::builder::IrBuilder;
use abyss_parser::parser::Parser;

use abyss_vm::{AbyssVm, codegen::IrCompiler};

fn main() {
    let code = fs::read_to_string("main.a").unwrap();

    let t = Instant::now();

    let mut err = DiagnosticEngine::new();
    err.add_source(0, "main.a".to_string(), code.clone());

    let mut parser = Parser::new(&code, &mut err, 0);
    let program = parser.parse_program();

    //println!("{:#?}", program);

    let mut type_checker = TypeChecker::new(&mut err);

    let tast = type_checker.check_program(program);
    tast.body.print_tree();

    println!();
    println!("{}", err.render());

    let mut cmp = IrBuilder::new();

    let ir_program = cmp.build_program(tast);

    let compiler = IrCompiler::new();
    let (instructions, constants) = compiler.compile(&ir_program);

    let mut vm = AbyssVm::new(instructions, constants);
    vm.run();
    let return_val = vm.get_register_as_i64(1);
    println!("CTFE Result (Register 2): {}", return_val);

    println!("\ntime: {}ms", t.elapsed().as_millis());
}
