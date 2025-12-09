use abyss_codegen::{
    director::Director,
    jit::CraneliftTarget,
    target::{Target, Type},
};
use abyss_parser::ast::{BinaryOp, Expr, Function, Lit, Program, Stmt};

pub unsafe extern "C" fn print_num(n: i64) {
    print!("\rCounter: {}", n);
    use std::io::Write;
    std::io::stdout().flush().unwrap();

    std::thread::sleep(std::time::Duration::from_millis(50));
}

fn main() {
    let symbols = [("print_num", print_num as *const u8)];
    let mut target = CraneliftTarget::new(&symbols);
    target.declare_extern_function("print_num", &[("n".to_string(), Type::Int)], Type::Void);

    let main_body = vec![
        // counter = 0
        Stmt::Let("counter".to_string(), Expr::Lit(Lit::Int(0))),
        // while (true)
        Stmt::While(
            Expr::Lit(Lit::Bool(true)),
            vec![
                // counter = counter + 1
                Stmt::Assign(
                    "counter".to_string(),
                    Expr::Binary(
                        Box::new(Expr::Ident("counter".to_string())),
                        BinaryOp::Add,
                        Box::new(Expr::Lit(Lit::Int(1))),
                    ),
                ),
                // print_num(counter)
                Stmt::Expr(Expr::Call(
                    "print_num".to_string(),
                    vec![Expr::Ident("counter".to_string())],
                )),
            ],
        ),
    ];

    let program = Program {
        functions: vec![Function {
            name: "main".to_string(),
            params: vec![],
            return_type: Some("void".to_string()),
            body: main_body,
        }],
    };

    let mut director = Director::new(&mut target);

    println!("Compiling...");
    director.process_program(&program);

    println!("Running...\n");
    let _ = target.run_fn("main");
}
