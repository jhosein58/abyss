use std::env;
use std::path::PathBuf;

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let root = PathBuf::from(manifest_dir);

    let tcc_path = root.join("third_party/tcc");

    println!("cargo:rustc-link-search=native={}", tcc_path.display());

    println!("cargo:rustc-link-lib=static=tcc");

    if env::var("CARGO_CFG_TARGET_OS").unwrap() != "windows" {
        println!("cargo:rustc-link-arg=-ldl");
        println!("cargo:rustc-link-arg=-lm");
        println!("cargo:rustc-link-arg=-lpthread");
    }

    println!("cargo:rerun-if-changed=third_party/tcc/libtcc.a");
}
