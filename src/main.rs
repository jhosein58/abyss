pub use std::{fs, time::Instant};

use abyss_nexus::nexus::Nexus;
use abyss_parser::parser::Parser;

fn main() {
    let t = Instant::now();

    let mut nexus = Nexus::new();

    let file_id = nexus.add_file("main.a", fs::read_to_string("main.a").unwrap());
    nexus.lex_file(file_id);

    let idx = Parser::new_indexer(&mut nexus, file_id).index();
    dbg!(&idx);

    //Parser::parse(&mut nexus, 0, len as u32);
    //nexus.hir.print_dump(&nexus);

    println!("Time: {:?}", t.elapsed());
}
