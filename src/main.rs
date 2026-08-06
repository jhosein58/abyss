pub use std::{fs, time::Instant};

use abyss_lexer::lexer::Lexer;
use abyss_nexus::nexus::Nexus;
use abyss_parser::parser::Parser;

fn main() {
    let code: &'static str = Box::leak(fs::read_to_string("main.a").unwrap().into_boxed_str());

    let t = Instant::now();

    let mut nexus = Nexus::new();

    let mut lexer = Lexer::new(code);
    let tokens = lexer.lex();
    dbg!(&tokens);

    //let len = tokens.kinds.len();
    nexus.set_tokens(tokens);
    nexus.reserve_for_tokens();

    let idx = Parser::new_indexer(&mut nexus).index();
    dbg!(&idx);

    //Parser::parse(&mut nexus, 0, len as u32);
    //nexus.hir.print_dump(&nexus);

    println!("Time: {:?}", t.elapsed());
}
