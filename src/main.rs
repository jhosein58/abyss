use abyss::{Abyss, CTarget};

fn main() {
    let code = include_str!("../main.a");
    let mut abyss = Abyss::new(code, CTarget::new());
    //abyss.run();
    println!("Code:\n{}", abyss.compile());
}
