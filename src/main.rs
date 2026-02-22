#![warn(clippy::todo)]

use std::fs;

use abyss_parser::parser::Parser;

// use abyss::{Abyss, CTarget};

// fn main() {
//     let code = fs::read_to_string("main.a").unwrap();
//     let mut abyss = Abyss::new(&code, "main.a", CTarget::new());
//     //abyss.run();
//     //println!("{}", abyss.emit())
//     println!("{}", abyss.compile());
// }

fn main() {
    let code = fs::read_to_string("main.a").unwrap();
    let mut p = Parser::new(&code);

    let program = p.parse_program().unwrap();
    println!("{}", program.to_string())
}
