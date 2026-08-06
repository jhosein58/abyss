pub use std::{fs, time::Instant};

use abyss_lexer::lexer::Lexer;
use abyss_nexus::{
    nexus::{HirId, NameId, Nexus},
    storages::scopes::ScopeStorage,
};
use abyss_parser::parser::Parser;

fn main() {
    let code: &'static str = Box::leak(fs::read_to_string("main.a").unwrap().into_boxed_str());

    let t = Instant::now();

    let mut nexus = Nexus::new();

    let mut lexer = Lexer::new(code);
    let tokens = lexer.lex();

    let len = tokens.kinds.len();
    nexus.set_tokens(tokens);
    nexus.reserve_for_tokens();

    Parser::parse(&mut nexus, 0, len as u32);
    nexus.hir.print_dump(&nexus);

    println!("Time: {:?}", t.elapsed());

    let mut s = ScopeStorage::default();
    let parent = s.alloc(None);
    let child = s.alloc(Some(parent));

    s.bind(parent, NameId(0), HirId(10));
    s.bind(child, NameId(0), HirId(20));

    println!("{:?}", s.lookup(child, NameId(0)));
}
