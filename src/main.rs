pub use std::{fs, time::Instant};

use abyss_nexus::nexus::Nexus;
use abyss_parser::parser::Parser;
use abyss_typer::tyck;

fn main() {
    let t = Instant::now();

    let mut nexus = Nexus::new();

    let file_id = nexus.add_file("main.a", fs::read_to_string("main.a").unwrap());
    nexus.lex_file(file_id);

    Parser::index(&mut nexus, file_id);

    let main_id = nexus.interner.get_id("main").unwrap();
    //let add_id = nexus.interner.get_id("add").unwrap();

    let main_symbol_id = Parser::parse_top_level(&mut nexus, file_id, main_id);
    //let add_symbol_id = Parser::parse_top_level(&mut nexus, file_id, add_id);

    nexus.hir.print_dump(&nexus);

    let main_range = nexus.symbol_hir_range.get(main_symbol_id).clone();

    tyck::check(&mut nexus, main_range);

    println!("Time: {:?}", t.elapsed());
}
