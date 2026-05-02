use std::cell::RefCell;
use std::rc::Rc;

use abyss_ir::facade::Ir;
use abyss_ir::ir::{IrFunction, IrProgram, IrType};
use abyss_script::{ast::AstNode, runtime::Runtime, syntax::build_parser};

fn main() {
    let source_code = r#"
        let a = 10
        let b = 20.5
        let null_val = nil

        let my_math = fn(x, y) {
            let res = x * y
            print(res)
        }

        my_math(a, b)
        print(2 + 2)
        let abc = 3
        print(abc)

        let test = fn() {
            print(1)
            -- test()
        }
        test()
    "#;

    println!("📜 Parsing source code...");

    let anon_funcs = Rc::new(RefCell::new(Vec::new()));

    let mut parser = build_parser(source_code, anon_funcs.clone());
    parser.advance();

    match parser.parse_program() {
        Ok(nodes) => {
            println!("✅ Parse success");

            let main_stmts = nodes
                .into_iter()
                .map(|n: AstNode| n.unwrap_stmt())
                .collect::<Vec<_>>();

            let mut program_functions = Ir::generate_dynamic_prelude();
            program_functions.extend(anon_funcs.borrow().clone());

            program_functions.push(IrFunction {
                name: "main".into(),
                params: vec![],
                return_ty: IrType::Unit,
                body: Some(main_stmts),
            });

            let ir_program = IrProgram {
                functions: program_functions,
                globals: vec![],
            };

            let mut runtime = Runtime::new();

            runtime.register_builtin(
                "sys_print_num",
                vec![IrType::F64],
                IrType::Unit,
                vec![false],
                Rc::new(|args: &[u64], _heap: &mut [u8]| -> u64 {
                    let val = f64::from_bits(args[0]);
                    println!("{}", val);
                    0
                }),
            );

            println!("⚙️ Running...");

            runtime.execute(ir_program);

            println!("✅ Done");
        }
        Err(e) => {
            println!("❌ Parse Error: {}", e);
        }
    }
}
