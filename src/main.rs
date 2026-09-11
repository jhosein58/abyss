pub use std::{fs, time::Instant};
use std::{fs::File, io::Write, process::Command};

use abyss_engine::engine::Engine;
use abyss_nexus::{
    arena::ArenaId,
    nexus::{SlotId, TypeId},
};
use abyss_typer::tyck::TyCtx;
use color_eyre::eyre::Ok;

fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;

    // let t = Instant::now();

    // let mut eng = Engine::new();

    // let file_id = eng.add_file("main.a", fs::read_to_string("main.a").unwrap());

    // let sym_id = eng.get_symbol_id(file_id, "main");

    // eng.abyss_main(sym_id);

    // eng.type_of(sym_id);

    // eng.db.dump_hir();
    // eng.print_err();

    // eng.compile(sym_id);

    // println!("{:?}", t.elapsed());

    // let c_code = eng.ccg.finish();
    // println!("\n\n{}", c_code);

    // let mut f_hanlde = File::create("main.c").unwrap();
    // f_hanlde.write_all(c_code.as_bytes()).unwrap();

    // let _ = Command::new("gcc")
    //     .arg("main.c")
    //     .arg("-o")
    //     .arg("abyss")
    //     .status();

    // let abyss_out = Command::new("./abyss").output().unwrap();

    // let output = String::from_utf8(abyss_out.stdout).unwrap();

    // println!("\n-------\n{}", output);

    // -------------------------------------------------------------------

    let mut eng = Engine::new();

    let s1 = eng.db.unify.tmp_slot();

    let infer_ty = eng.db.types.alloc_infer(s1.value());

    let array_ty = eng.db.types.alloc_array(infer_ty, 3);

    let s2 = eng.db.unify.tmp_slot();

    eng.db
        .unify
        .bind_type(&mut eng.db.types, s2, array_ty)
        .unwrap();

    let res_arr_ty = eng.db.unify.resolve_type(s2);

    let inner_ty = eng.db.types.get_array_type(res_arr_ty);

    let inner_slot = SlotId(eng.db.types.payload(inner_ty));

    let i32ty = eng.db.types.alloc_int(32);

    eng.db
        .unify
        .bind_type(&mut eng.db.types, inner_slot, i32ty)
        .unwrap();

    let res_arr_ty = eng.db.unify.resolve_type_deep(&mut eng.db.types, s2);

    println!("{}", eng.db.types.name(res_arr_ty));

    Ok(())
}
