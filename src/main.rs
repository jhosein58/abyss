use std::fs;

use abyss::{Abyss, CTarget};

fn main() {
    let code = fs::read_to_string("main.a").unwrap();
    let mut abyss = Abyss::new(&code, "main.a", CTarget::new());
    abyss.run();
    //println!("{}", abyss.emit())
    //println!("{}", abyss.compile());
}
