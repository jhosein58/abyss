pub use std::{fs, time::Instant};

use abyss_lexer::lexer::Lexer;
use abyss_nexus::nexus::Nexus;
use abyss_parser::parser::Parser;

fn main() {
    let code: &'static str = Box::leak(fs::read_to_string("main.a").unwrap().into_boxed_str());

    let mut nexus = Nexus::new();
    let mut lexer = Lexer::new(code);
    let tokens = lexer.lex();

    dbg!(&tokens);

    let len = tokens.kinds.len();
    nexus.set_tokens(tokens);

    Parser::parse(&mut nexus, 0, len as u32);
    nexus.hir.print_dump(&nexus);
}
