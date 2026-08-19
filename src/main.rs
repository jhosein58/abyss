pub use std::{fs, time::Instant};

use abyss_codegen::cranelift::codegen::CraneliftBackend;
use abyss_diagnostics::DiagnosticFormatter;
use abyss_engine::engine::Engine;
use abyss_typer::tyck;

fn main() {
    let source = fs::read_to_string("main.a").unwrap();

    let tc = Instant::now();

    let mut eng = Engine::new();

    let file_id = eng.add_file("main.a", source);
    let sym_id = eng.parse(file_id, "main");
    eng.type_check(sym_id);

    // Compile
    let mut backend = CraneliftBackend::new();
    let func_id =
        abyss_lower::materialazer::lower_function(&mut nexus, &mut backend, main_symbol_id);
    let code_ptr = backend.compile_and_get_ptr(func_id);
    let main_fn: extern "C" fn() -> i32 = unsafe { std::mem::transmute(code_ptr) };

    let comptime = tc.elapsed();

    nexus.dump_hir();
    let formater = DiagnosticFormatter::new(&nexus);
    let diagnostics = formater.format_all();
    println!("{}", diagnostics);
    println!("Compile Time: {:?}\n", comptime);

    let tr = Instant::now();
    let result = main_fn();
    let runtime = tr.elapsed();

    println!("main says: {}\n", result);
    println!("Run Time: {:?}", runtime);
}
