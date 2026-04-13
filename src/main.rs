pub use std::{fs, time::Instant};

use abyss::abyss::Abyss;
pub use abyss_analyzer::type_checker::engine::TypeChecker;
pub use abyss_diagnostics::DiagnosticEngine;
pub use abyss_ir::builder::IrBuilder;
pub use abyss_parser::parser::Parser;

pub use abyss_utils::idgen::IdGenerator;
pub use abyss_vm::codegen::IrCompiler;

fn main() {
    let code = fs::read_to_string("main.a").unwrap();

    Abyss::new(code)
        .with_filename("main.a")
        //.disable_tast_print()
        .run_llvm_jit();
}
