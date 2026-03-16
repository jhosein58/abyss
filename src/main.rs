pub use std::{fs, time::Instant};

pub use abyss_analyzer::type_checker::engine::TypeChecker;
pub use abyss_diagnostics::DiagnosticEngine;
pub use abyss_ir::builder::IrBuilder;
pub use abyss_parser::parser::Parser;

use abyss_utils::idgen::IdGenerator;
pub use abyss_vm::{AbyssVm, codegen::IrCompiler};

fn main() {
    let code = fs::read_to_string("main.a").unwrap();

    let t = Instant::now();

    let mut err = DiagnosticEngine::new();
    err.add_source(0, "main.a".to_string(), code.clone());

    let mut idgen = IdGenerator::new();

    let mut parser = Parser::new(&code, &mut err, &mut idgen, 0);
    let program = parser.parse_program();

    println!("\n{}\n\n", program);

    let mut type_checker = TypeChecker::new(&mut err, &mut idgen);

    let tast = type_checker.check_program(&program);
    tast.body.print_tree();

    println!();
    println!("{}", err.render());

    let mut cmp = IrBuilder::new();

    let ir_program = cmp.build_program(tast);

    let compiler = IrCompiler::new();
    let (instructions, constants) = compiler.compile(&ir_program);

    //println!("{:#?}", instructions);
    let mut vm = AbyssVm::new(instructions, constants);
    println!("\nCompiled in: {}ms", t.elapsed().as_millis());

    let t = Instant::now();
    vm.run();

    println!("\nExecuted in: {}ms", t.elapsed().as_millis());
}
