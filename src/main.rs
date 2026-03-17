pub use std::{fs, time::Instant};

use abyss::abyss::Abyss;
pub use abyss_analyzer::type_checker::engine::TypeChecker;
pub use abyss_diagnostics::DiagnosticEngine;
pub use abyss_ir::builder::IrBuilder;
pub use abyss_parser::parser::Parser;

pub use abyss_utils::idgen::IdGenerator;
pub use abyss_vm::{AbyssVm, codegen::IrCompiler};

fn native_print_string(vm: &mut AbyssVm, args: &[u64]) -> u64 {
    let ptr = args[0];

    let string_value = vm.read_c_string(ptr);

    print!("{}", string_value);
    vm.out.push_str(&string_value);

    0
}

fn main() {
    let code = fs::read_to_string("main.a").unwrap();

    Abyss::new(code)
        .with_filename("main.a")
        .disable_tast_print()
        .register_native("print_str", 1, native_print_string)
        .run();
}
