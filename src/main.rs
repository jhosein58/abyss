pub use std::{fs, time::Instant};

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
    //let add_id = nexus.interner.get_id("add").unwrap();

    let main_symbol_id = Parser::parse_top_level(&mut nexus, file_id, main_id);
    //let add_symbol_id = Parser::parse_top_level(&mut nexus, file_id, add_id);

    let main_range = nexus.symbol_hir_range.get(main_symbol_id).clone();

    tyck::type_check(&mut nexus, main_range);

    nexus.dump_hir();

    let formater = DiagnosticFormatter::new(&nexus);
    let diagnostics = formater.format_all();

    println!("{}", diagnostics);

    println!("Time: {:?}", t.elapsed());
}
