pub use std::{fs, time::Instant};

use abyss_nexus::nexus::Nexus;
use abyss_parser::parser::Parser;

fn main() {
    let t = Instant::now();

    let mut nexus = Nexus::new();

    let file_id = nexus.add_file("main.a", fs::read_to_string("main.a").unwrap());
    nexus.lex_file(file_id);

    Parser::index(&mut nexus, file_id);

    let main_id = nexus.interner.get_id("main").unwrap();

    let symbol_id = Parser::parse_top_level(&mut nexus, file_id, main_id);

    nexus.hir.print_dump(&nexus);

    println!("{:?}", nexus.symbol_hir_range.get(symbol_id));

    println!("Time: {:?}", t.elapsed());
}
