pub use std::{fs, time::Instant};

use abyss_codegen::cranelift::codegen::CraneliftBackend;
use abyss_diagnostics::DiagnosticFormatter;
use abyss_indexer::Indexer;
use abyss_nexus::nexus::Nexus;
use abyss_parser::parser::Parser;
use abyss_typer::tyck;

fn main() {
    let t = Instant::now();

    let mut nexus = Nexus::new();

    let file_id = nexus.add_file("main.a", fs::read_to_string("main.a").unwrap());
    nexus.lex_file(file_id);

    Indexer::index(&mut nexus, file_id);

    let main_id = nexus.interner.get_id("main").unwrap();

    let main_symbol_id = Parser::parse_top_level(&mut nexus, file_id, main_id);

    let main_range = nexus.symbol_hir_range.get(main_symbol_id).clone();

    // Type-Checking
    tyck::type_check(&mut nexus, main_range);

    // Compile
    let mut backend = CraneliftBackend::new();

    let func_id = abyss_lower::materialazer::lower_function(&nexus, &mut backend, main_symbol_id);

    let elapsed = t.elapsed();

    nexus.dump_hir();
    let formater = DiagnosticFormatter::new(&nexus);
    let diagnostics = formater.format_all();
    println!("{}", diagnostics);
    println!("Compile Time: {:?}\n", elapsed);

    let code_ptr = backend.compile_and_get_ptr(func_id);
    let main_fn: extern "C" fn() -> i32 = unsafe { std::mem::transmute(code_ptr) };

    let t = Instant::now();
    let result = main_fn();

    println!("main says: {}\n", result);
    println!("Run Time: {:?}", t.elapsed());
}
