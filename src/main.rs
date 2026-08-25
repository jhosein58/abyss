pub use std::{fs, time::Instant};
use std::{fs::File, io::Write, process::Command};

use abyss_engine::engine::Engine;
use abyss_typer::tyck::TyCtx;
use color_eyre::eyre::Ok;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    let t = Instant::now();

    let mut eng = Engine::new();

    let file_id = eng.add_file("main.a", fs::read_to_string("main.a").unwrap());

    let sym_id = eng.get_symbol_id(file_id, "main");

    eng.abyss_main(sym_id);
    eng.type_of(sym_id);

    eng.db.dump_hir();
    eng.print_err();

    eng.compile(sym_id);

    println!("{:?}", t.elapsed());

    let c_code = eng.ccg.finish();
    println!("\n\n{}", c_code);

    let mut f_hanlde = File::create("main.c").unwrap();
    f_hanlde.write_all(c_code.as_bytes()).unwrap();

    let _ = Command::new("gcc")
        .arg("main.c")
        .arg("-o")
        .arg("abyss")
        .status();

    let abyss_out = Command::new("./abyss").output().unwrap();

    let output = String::from_utf8(abyss_out.stdout).unwrap();

    println!("\n-------\n{}", output);

    Ok(())
}
