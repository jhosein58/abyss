pub use std::{fs, time::Instant};

use abyss::abyss::Abyss;
pub use abyss_analyzer::type_checker::engine::TypeChecker;
pub use abyss_diagnostics::DiagnosticEngine;
pub use abyss_ir::builder::IrBuilder;
use abyss_lexer_new::lexer::Lexer;
pub use abyss_parser::parser::Parser;

pub use abyss_utils::idgen::IdGenerator;
pub use abyss_vm::codegen::IrCompiler;

fn main() {
    let code = fs::read_to_string("main.a").unwrap();

    Abyss::new(code.clone())
        .with_filename("main.a")
        .with_host_function("print_f32", 1, vec![false], |args, _heap| {
            let val = f64::from_bits(args[0] as u64);
            println!("{}", val);
            0
        })
        .with_host_function("print_i32", 1, vec![false], |args, _heap| {
            let val = args[0] as i32;
            println!("{}", val);
            0
        })
        .with_host_function("print", 1, vec![false], |args, heap| {
            let mut offset = args[0] as usize;

            while offset + 4 <= heap.len() {
                let mut val: u32 = 0;
                unsafe {
                    std::ptr::copy_nonoverlapping(
                        heap.as_ptr().add(offset),
                        &mut val as *mut u32 as *mut u8,
                        4,
                    );
                }

                let char_val = u32::from_le(val);

                if char_val == 0 {
                    break;
                }

                if let Some(c) = std::char::from_u32(char_val) {
                    print!("{}", c);
                } else {
                    print!("");
                }

                offset += 4;
            }

            0
        })
        .with_host_function("printiln", 1, vec![false], |args, _heap| {
            let val = args[0] as i32;
            println!("{}", val);
            0
        })
        .with_host_function("printfln", 1, vec![false], |args, _heap| {
            let val = f64::from_bits(args[0] as u64);
            println!("{}", val);
            0
        })
        .with_host_function("printbln", 1, vec![false], |args, _heap| {
            let val = args[0] != 0;
            println!("{}", val);
            0
        })
        .run();

    let mut lexer = Lexer::new(&code);
    let tokens = lexer.lex();
    println!("{:#?}", tokens);
}
