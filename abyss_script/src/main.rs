use abyss_ir::ir::IrType;
use abyss_script::{ast::AstNode, runtime::Runtime, syntax::build_parser};

use std::rc::Rc;

fn main() {
    let source_code = r#"
        let i = 1
        while i < 10 {
            let j = 0
            while j < i {
                star()
                j = j + 1
            }
            ln()
            i = i + 1
        }
    "#;

    println!("📜 Parsing source code...");
    let mut parser = build_parser(source_code);
    parser.advance();

    match parser.parse_program() {
        Ok(nodes) => {
            println!("✅ Parse success");
            let stmts = nodes
                .into_iter()
                .map(|n: AstNode| n.unwrap_stmt())
                .collect();

            let mut runtime = Runtime::new();

            runtime.register_builtin(
                "print",
                vec![IrType::I64],
                IrType::I64,
                vec![false],
                Rc::new(|args: &[u64], _heap: &mut [u8]| -> u64 {
                    println!("{}", args[0] as i64);
                    0
                }),
            );

            runtime.register_builtin(
                "star",
                vec![],
                IrType::I64,
                vec![],
                Rc::new(|_args: &[u64], _heap: &mut [u8]| -> u64 {
                    print!("*",);
                    0
                }),
            );
            runtime.register_builtin(
                "ln",
                vec![],
                IrType::I64,
                vec![],
                Rc::new(|_args: &[u64], _heap: &mut [u8]| -> u64 {
                    println!("",);
                    0
                }),
            );

            println!("⚙️ Running...");
            runtime.execute(stmts);
            println!("✅ Done");
        }
        Err(e) => {
            println!("❌ Parse Error: {}", e);
        }
    }
}
