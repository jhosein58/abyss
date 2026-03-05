#![warn(clippy::todo)]

use std::fs;

use abyss_analyzer::type_checker::{engine::TypeChecker, tast::TypedProgram};
//use abyss_analyzer::type_checker::engine::TypeChecker;
use abyss_diagnostics::DiagnosticEngine;
use abyss_parser::parser::Parser;

use abyss_vm::{AbyssVm, Instruction, OpCode, codegen::Compiler};

fn main() {
    let code = fs::read_to_string("main.a").unwrap();

    let mut err = DiagnosticEngine::new();
    err.add_source(0, "main.a".to_string(), code.clone());

    let mut parser = Parser::new(&code, &mut err, 0);
    let program = parser.parse_program();

    //println!("{:#?}", program);

    let mut type_checker = TypeChecker::new(&mut err);

    let tast = type_checker.check_expr(&program.body);

    let mut cmp = Compiler::new();

    cmp.compile_program(&TypedProgram { body: tast });

    println!();
    println!("{}", err.render());

    let mut vm = AbyssVm::new(cmp.builder.instructions, cmp.builder.constants);
    println!("--- Running VM ---");
    vm.run();
    println!("--- VM Finished ---");

    let return_val = vm.get_register_as_i64(1);
    println!("CTFE Result (Register 2): {}", return_val);
}
