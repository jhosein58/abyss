pub use std::{fs, time::Instant};

use abyss_codegen::cranelift::codegen::CraneliftBackend;
use abyss_diagnostics::DiagnosticFormatter;
use abyss_engine::engine::Engine;

fn main() {
    let mut eng: Engine<CraneliftBackend> = Engine::new();

    let file_id = eng.add_file("main.a", fs::read_to_string("main.a").unwrap());
    let sym_id = eng.parse(file_id, "main");
    eng.type_check(sym_id);

    eng.db.dump_hir();

    let formater = DiagnosticFormatter::new(&eng.db);
    let diagnostics = formater.format_all();
    println!("{}", diagnostics);

    let res = eng.run(sym_id);
    println!("main says: {}", res)
}
