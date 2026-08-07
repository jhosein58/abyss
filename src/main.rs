pub use std::{fs, time::Instant};

use abyss_nexus::nexus::{NameId, Nexus};
use abyss_parser::parser::Parser;

fn main() {
    let t = Instant::now();

    let mut nexus = Nexus::new();

    let file_id = nexus.add_file("main.a", fs::read_to_string("main.a").unwrap());
    nexus.lex_file(file_id);

    Parser::index(&mut nexus, file_id);

    Parser::parse(&mut nexus, file_id, NameId(1));
    Parser::parse(&mut nexus, file_id, NameId(2));

    nexus.hir.print_dump(&nexus);

    println!("Time: {:?}", t.elapsed());
}
