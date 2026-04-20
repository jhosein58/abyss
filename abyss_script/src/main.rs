use abyss_ir::{
    facade::Ir,
    ir::{IrExpr, IrStmt, IrType},
};
use abyss_script::{
    lexer::{DynamicLexerRules, Scanner},
    parser::DynamicPrattParser,
    png::DynamicSyntaxMagic,
};
use abyss_vm::{codegen::IrCompiler, vm::core::AbyssVm};
use std::rc::Rc;

#[derive(Clone)]
pub enum AstNode {
    Expr(IrExpr),
    Stmt(IrStmt),
}

impl AstNode {
    fn unwrap_expr(self) -> IrExpr {
        match self {
            AstNode::Expr(e) => e,
            _ => panic!("Expected an Expression, but found a Statement!"),
        }
    }
    fn unwrap_stmt(self) -> IrStmt {
        match self {
            AstNode::Stmt(s) => s,
            AstNode::Expr(e) => IrStmt::Expr(e),
        }
    }
}

fn main() {
    let source_code = "
        let limit = 5;
        let counter = 0;

        while (counter < limit) {
            counter = counter + 1;

            if (counter == 3) {
                print(999);
            } else {
                print(counter);
            }
        }
    ";

    println!("📜 Parsing source code...");

    let mut lexer_rules = DynamicLexerRules::new();
    lexer_rules.add_token("Number", r"\d+");
    
    // Keywords must come before Ident
    lexer_rules.add_token("Let", r"let");
    lexer_rules.add_token("If", r"if");
    lexer_rules.add_token("Else", r"else");
    lexer_rules.add_token("While", r"while");
    lexer_rules.add_token("True", r"true");
    lexer_rules.add_token("False", r"false");
    
    lexer_rules.add_token("Ident", r"\a\w*");
    lexer_rules.add_token("LParen", r"\(");
    lexer_rules.add_token("RParen", r"\)");
    lexer_rules.add_token("LBrace", r"\{");
    lexer_rules.add_token("RBrace", r"\}");
    lexer_rules.add_token("Semicolon", r";");
    lexer_rules.add_token("Comma", r",");
    lexer_rules.add_token("Equals", r"=");
    lexer_rules.add_token("Plus", r"\+");
    lexer_rules.add_token("Minus", r"-");
    lexer_rules.add_token("Star", r"\*");
    lexer_rules.add_token("Slash", r"/");
    lexer_rules.add_token("Lt", r"<");
    lexer_rules.add_token("Gt", r">");
    lexer_rules.add_token("Space", r"\s+");
    lexer_rules.add_token("Comment", "//~\n*");

    let scanner = Scanner::new(source_code, lexer_rules);

    let mut parser: DynamicPrattParser<AstNode> = DynamicPrattParser::new(scanner, 0);
    parser.ignore_token("Space");
    parser.ignore_token("Comment");

    parser.register_rule(
        "Number",
        0,
        Some(Rc::new(|_, tk| {
            Ok(AstNode::Expr(Ir::int(tk.text.parse().unwrap())))
        })),
        None,
    );
    parser.register_rule(
        "Ident",
        0,
        Some(Rc::new(|_, tk| {
            Ok(AstNode::Expr(Ir::var(tk.text.to_string())))
        })),
        None,
    );

    parser.define_expr("true", 0, |_| AstNode::Expr(Ir::bool(true)));
    parser.define_expr("false", 0, |_| AstNode::Expr(Ir::bool(false)));

    parser.define_expr(":l + :r", 10, |ctx| {
        AstNode::Expr(Ir::add(
            ctx.get_node("l").unwrap_expr(),
            ctx.get_node("r").unwrap_expr(),
        ))
    });
    parser.define_expr(":l - :r", 10, |ctx| {
        AstNode::Expr(Ir::sub(
            ctx.get_node("l").unwrap_expr(),
            ctx.get_node("r").unwrap_expr(),
        ))
    });
    parser.define_expr(":l * :r", 20, |ctx| {
        AstNode::Expr(Ir::mul(
            ctx.get_node("l").unwrap_expr(),
            ctx.get_node("r").unwrap_expr(),
        ))
    });
    parser.define_expr(":l / :r", 20, |ctx| {
        AstNode::Expr(Ir::div(
            ctx.get_node("l").unwrap_expr(),
            ctx.get_node("r").unwrap_expr(),
        ))
    });

    parser.define_expr(":l < :r", 5, |ctx| {
        AstNode::Expr(Ir::lt(
            ctx.get_node("l").unwrap_expr(),
            ctx.get_node("r").unwrap_expr(),
        ))
    });
    parser.define_expr(":l > :r", 5, |ctx| {
        AstNode::Expr(Ir::gt(
            ctx.get_node("l").unwrap_expr(),
            ctx.get_node("r").unwrap_expr(),
        ))
    });
    parser.define_expr(":l == :r", 5, |ctx| {
        AstNode::Expr(Ir::eq(
            ctx.get_node("l").unwrap_expr(),
            ctx.get_node("r").unwrap_expr(),
        ))
    });
    parser.define_expr(":l != :r", 5, |ctx| {
        AstNode::Expr(Ir::neq(
            ctx.get_node("l").unwrap_expr(),
            ctx.get_node("r").unwrap_expr(),
        ))
    });

    parser.define_expr(":target = :val", 2, |ctx| {
        let t = ctx.get_node("target").unwrap_expr();
        let name = match t.kind {
            abyss_ir::ir::IrExprKind::VarRef(n) => n,
            _ => panic!("Target of assignment must be a variable name"),
        };
        AstNode::Stmt(IrStmt::Assign {
            target: name,
            val: ctx.get_node("val").unwrap_expr(),
        })
    });

    parser.define_expr(":func ( $(:args),* )", 30, |ctx| {
        let f = ctx.get_node("func").unwrap_expr();
        let name = match f.kind {
            abyss_ir::ir::IrExprKind::VarRef(n) => n,
            _ => panic!("Function name must be an identifier"),
        };
        let args = ctx
            .get_node_list("args")
            .into_iter()
            .map(|n| n.unwrap_expr())
            .collect();
        AstNode::Expr(Ir::call(name, args))
    });

    parser.define_stmt("let @name = :val ;", |ctx| {
        AstNode::Stmt(Ir::var_dec(
            ctx.get_ident("name"),
            ctx.get_node("val").unwrap_expr(),
        ))
    });

    parser.define_stmt(
        "if ( :cond ) { $(:then_body);* } else { $(:else_body);* }",
        |ctx| {
            AstNode::Stmt(IrStmt::If(
                ctx.get_node("cond").unwrap_expr(),
                ctx.get_node_list("then_body")
                    .into_iter()
                    .map(|n| n.unwrap_stmt())
                    .collect(),
                ctx.get_node_list("else_body")
                    .into_iter()
                    .map(|n| n.unwrap_stmt())
                    .collect(),
            ))
        },
    );

    parser.define_stmt("while ( :cond ) { $(:body);* }", |ctx| {
        AstNode::Stmt(IrStmt::While {
            cond: ctx.get_node("cond").unwrap_expr(),
            body: ctx
                .get_node_list("body")
                .into_iter()
                .map(|n| n.unwrap_stmt())
                .collect(),
        })
    });

    parser.advance();

    match parser.parse_program() {
        Ok(nodes) => {
            println!("✅ Code parsed successfully!\n");

            let stmts: Vec<IrStmt> = nodes.into_iter().map(|n| n.unwrap_stmt()).collect();
            let ir_program = Ir::program(stmts);

            let mut compiler = IrCompiler::new();

            compiler.register_extern("print", vec![IrType::I64], IrType::I64);

            let (instructions, constants, extern_defs) = compiler.compile(&ir_program);
            let mut vm = AbyssVm::new(instructions, constants);

            let print_function = Rc::new(|args: &[u64], _heap: &mut [u8]| -> u64 {
                println!("🖨️  Print from Abyss: {}", args[0] as i64);
                0
            });

            vm.register_host_function("print", 1, vec![false], print_function);

            vm.load_imports(&extern_defs);
            vm.init_globals(ir_program.globals.len());

            println!("⚙️ Running VM...\n");
            vm.run();
            println!("\n✅ Execution finished!");
        }
        Err(e) => println!("❌ Parse Error: {}", e),
    }
}
