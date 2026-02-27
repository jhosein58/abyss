#![warn(clippy::todo)]

use std::fs;

//use abyss_analyzer::type_checker::engine::TypeChecker;
use abyss_diagnostics::DiagnosticEngine;
use abyss_parser::parser::Parser;

fn main() {
    let code = fs::read_to_string("main.a").unwrap();

    let mut err = DiagnosticEngine::new();
    err.add_source(0, "main.a".to_string(), code.clone());

    let mut parser = Parser::new(&code, &mut err, 0);
    //let mut type_checker = TypeChecker::new();

    let program = parser.parse_program();
    println!("{}", program.to_string());
    println!();
    println!();
    println!("{}", err.render());
    println!();
    // let tast = type_checker.check_expr(&program.body);
    // tast.print_tree();
}
