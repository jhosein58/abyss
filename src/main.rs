pub use std::{fs, time::Instant};

use abyss_engine::engine::Engine;
use abyss_typer::tyck::TyCtx;

fn main() {
    let t = Instant::now();

    let mut eng = Engine::new();

    let file_id = eng.add_file("main.a", fs::read_to_string("main.a").unwrap());

    let sym_id = eng.get_symbol_id(file_id, "main");

    eng.type_of(sym_id);

    eng.db.dump_hir();
    eng.print_err();

    eng.compile(sym_id);

    println!("{:?}", t.elapsed());

    println!("main says: {}", eng.ccg.finish());
}
