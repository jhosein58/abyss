use crate::ast::AstNode;
use crate::core::lexer::{DynamicLexerRules, Scanner};
use crate::core::parser::DynamicPrattParser;
use crate::core::png::{DynamicSyntaxMagic, SyntaxCtx};

use abyss_ir::facade::Ir;
use abyss_ir::ir::{IrExpr, IrExprKind, IrFunction, IrStmt, IrType};

use std::cell::RefCell;
use std::rc::Rc;
pub fn build_parser<'a>(
    source_code: &'a str,
    anon_funcs: Rc<RefCell<Vec<IrFunction>>>,
) -> DynamicPrattParser<'a, AstNode> {
    let mut lexer_rules = DynamicLexerRules::new();

    lexer_rules.add_token("Float", r"\d+\.\d+");
    lexer_rules.add_token("Number", r"\d+");
    lexer_rules.add_token("Ident", r"\a\w*");
    lexer_rules.add_token("Space", r"\s+");
    lexer_rules.add_token("Comment", "--~\n*");

    let scanner = Scanner::new(source_code, lexer_rules);
    let mut parser: DynamicPrattParser<AstNode> = DynamicPrattParser::new(scanner, 0);

    parser.ignore_token("Space");
    parser.ignore_token("Comment");

    parser.define_expr("nil", 0, |_| AstNode::Expr(Ir::call("rt_make_nil", vec![])));

    parser.register_rule(
        "Number",
        0,
        Some(Rc::new(|_, tk| {
            let val = tk.text.parse::<f64>().unwrap();
            Ok(AstNode::Expr(Ir::call(
                "rt_make_number",
                vec![Ir::float(val)],
            )))
        })),
        None,
    );

    parser.register_rule(
        "Float",
        0,
        Some(Rc::new(|_, tk| {
            let val = tk.text.parse::<f64>().unwrap();
            Ok(AstNode::Expr(Ir::call(
                "rt_make_number",
                vec![Ir::float(val)],
            )))
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
    parser.define_expr("( :expr )", 100, |ctx: SyntaxCtx<AstNode>| {
        ctx.get_node("expr")
    });

    parser.define_expr("print ( :val )", 100, |ctx| {
        AstNode::Expr(Ir::call("print", vec![ctx.get_node("val").unwrap_expr()]))
    });

    parser.define_expr(":l + :r", 20, |ctx| {
        AstNode::Expr(Ir::call(
            "rt_add",
            vec![
                ctx.get_node("l").unwrap_expr(),
                ctx.get_node("r").unwrap_expr(),
            ],
        ))
    });
    parser.define_expr(":l - :r", 20, |ctx| {
        AstNode::Expr(Ir::call(
            "rt_sub",
            vec![
                ctx.get_node("l").unwrap_expr(),
                ctx.get_node("r").unwrap_expr(),
            ],
        ))
    });
    parser.define_expr(":l * :r", 30, |ctx| {
        AstNode::Expr(Ir::call(
            "rt_mul",
            vec![
                ctx.get_node("l").unwrap_expr(),
                ctx.get_node("r").unwrap_expr(),
            ],
        ))
    });
    parser.define_expr(":l / :r", 30, |ctx| {
        AstNode::Expr(Ir::call(
            "rt_div",
            vec![
                ctx.get_node("l").unwrap_expr(),
                ctx.get_node("r").unwrap_expr(),
            ],
        ))
    });

    parser.define_expr(":l < :r", 10, |ctx: SyntaxCtx<AstNode>| {
        AstNode::Expr(Ir::lt(
            ctx.get_node("l").unwrap_expr(),
            ctx.get_node("r").unwrap_expr(),
        ))
    });
    parser.define_expr(":l > :r", 10, |ctx: SyntaxCtx<AstNode>| {
        AstNode::Expr(Ir::gt(
            ctx.get_node("l").unwrap_expr(),
            ctx.get_node("r").unwrap_expr(),
        ))
    });
    parser.define_expr(":l == :r", 9, |ctx: SyntaxCtx<AstNode>| {
        AstNode::Expr(Ir::eq(
            ctx.get_node("l").unwrap_expr(),
            ctx.get_node("r").unwrap_expr(),
        ))
    });
    parser.define_expr(":l != :r", 9, |ctx: SyntaxCtx<AstNode>| {
        AstNode::Expr(Ir::neq(
            ctx.get_node("l").unwrap_expr(),
            ctx.get_node("r").unwrap_expr(),
        ))
    });

    parser.define_expr(":target = :val", 2, |ctx| {
        let target = match ctx.get_node("target").unwrap_expr().kind {
            IrExprKind::VarRef(n) => n,
            _ => panic!("assignment target must be a variable"),
        };
        AstNode::Stmt(IrStmt::Assign {
            target,
            val: ctx.get_node("val").unwrap_expr(),
        })
    });

    let funcs_ref = anon_funcs.clone();
    parser.define_expr("fn ( $(:args),* ) { $(:body)* }", 100, move |ctx| {
        let mut funcs = funcs_ref.borrow_mut();
        let func_name = format!("anon_fn_{}", funcs.len());

        let arg_names: Vec<String> = ctx
            .get_node_list("args")
            .into_iter()
            .map(|n| match n.unwrap_expr().kind {
                IrExprKind::VarRef(name) => name,
                _ => panic!("Arguments must be identifiers"),
            })
            .collect();

        let body_stmts_raw = ctx.get_node_list("body");

        let mut body_stmts: Vec<IrStmt> = body_stmts_raw
            .into_iter()
            .map(|n| n.unwrap_stmt())
            .collect();
        body_stmts.push(IrStmt::Return(Some(Ir::call("rt_make_nil", vec![]))));

        let params = arg_names
            .into_iter()
            .map(|n| (n, Ir::value_ty()))
            .collect::<Vec<_>>();

        funcs.push(IrFunction {
            name: func_name.clone(),
            params: params.clone(),
            return_ty: Ir::value_ty(),
            body: Some(body_stmts),
        });

        let func_ptr_ty = IrType::FuncPtr {
            params: params.into_iter().map(|_| Ir::value_ty()).collect(),
            ret: Box::new(Ir::value_ty()),
        };

        let func_addr = Ir::expr(IrExprKind::FuncAddr(func_name), func_ptr_ty.clone());
        let opaque_ptr = Ir::expr(
            IrExprKind::Cast(Box::new(func_addr), IrType::Ptr(Box::new(IrType::Unit))),
            IrType::Ptr(Box::new(IrType::Unit)),
        );

        AstNode::Expr(Ir::call("rt_make_func", vec![opaque_ptr]))
    });
    parser.define_expr(":func ( $(:args),* )", 40, |ctx| {
        let func_val = ctx.get_node("func").unwrap_expr();

        let args: Vec<IrExpr> = ctx
            .get_node_list("args")
            .into_iter()
            .map(|x| x.unwrap_expr())
            .collect();

        let union_ptr = Ir::expr(
            IrExprKind::GetFieldPtr {
                base: Box::new(func_val),
                index: 1,
            },
            IrType::Ptr(Box::new(IrType::Union(vec![
                IrType::Unit,
                IrType::F64,
                IrType::Ptr(Box::new(IrType::Unit)),
            ]))),
        );
        let opaque_ptr = Ir::expr(
            IrExprKind::FieldAccess {
                base: Box::new(union_ptr),
                index: 2,
            },
            IrType::Ptr(Box::new(IrType::Unit)),
        );

        let func_ptr_ty = IrType::FuncPtr {
            params: vec![Ir::value_ty(); args.len()],
            ret: Box::new(Ir::value_ty()),
        };
        let real_ptr = Ir::expr(
            IrExprKind::Cast(Box::new(opaque_ptr), func_ptr_ty.clone()),
            func_ptr_ty,
        );

        AstNode::Expr(Ir::expr(
            IrExprKind::CallIndirect {
                ptr: Box::new(real_ptr),
                args,
            },
            Ir::value_ty(),
        ))
    });

    parser.define_stmt("let @name = :val", |ctx| {
        AstNode::Stmt(IrStmt::VarDec {
            name: ctx.get_ident("name"),
            ty: Ir::value_ty(),
            init: Some(ctx.get_node("val").unwrap_expr()),
        })
    });

    parser.define_stmt(
        "if :cond { $(:then_body)* } $( else $< { $(:else_body)* } | :elif > )?",
        |ctx: SyntaxCtx<AstNode>| {
            let cond = ctx.get_node("cond").unwrap_expr();
            let then_body = ctx
                .get_node_list("then_body")
                .into_iter()
                .map(|n: AstNode| n.unwrap_stmt())
                .collect();
            let mut elif_nodes = ctx.get_node_list("elif");

            let else_body = if !elif_nodes.is_empty() {
                vec![elif_nodes.remove(0).unwrap_stmt()]
            } else {
                ctx.get_node_list("else_body")
                    .into_iter()
                    .map(|n: AstNode| n.unwrap_stmt())
                    .collect()
            };

            AstNode::Stmt(IrStmt::If(cond, then_body, else_body))
        },
    );

    parser.define_stmt("while :cond { $(:body)* }", |ctx: SyntaxCtx<AstNode>| {
        AstNode::Stmt(IrStmt::While {
            cond: ctx.get_node("cond").unwrap_expr(),
            body: ctx
                .get_node_list("body")
                .into_iter()
                .map(|n: AstNode| n.unwrap_stmt())
                .collect(),
        })
    });

    parser
}
