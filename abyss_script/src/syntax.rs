use crate::ast::AstNode;
use crate::core::lexer::{DynamicLexerRules, Scanner};
use crate::core::parser::DynamicPrattParser;
use crate::core::png::{DynamicSyntaxMagic, SyntaxCtx};

use abyss_ir::facade::Ir;
use abyss_ir::ir::{IrExprKind, IrStmt};

use std::rc::Rc;

pub fn build_parser<'a>(source_code: &'a str) -> DynamicPrattParser<'a, AstNode> {
    let mut lexer_rules = DynamicLexerRules::new();

    lexer_rules.add_token("Number", r"\d+");
    lexer_rules.add_token("Ident", r"\a\w*");
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
            Ok(AstNode::Expr(Ir::int(tk.text.parse::<i64>().unwrap())))
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

    parser.define_expr(":l + :r", 20, |ctx: SyntaxCtx<AstNode>| {
        AstNode::Expr(Ir::add(
            ctx.get_node("l").unwrap_expr(),
            ctx.get_node("r").unwrap_expr(),
        ))
    });
    parser.define_expr(":l - :r", 20, |ctx: SyntaxCtx<AstNode>| {
        AstNode::Expr(Ir::sub(
            ctx.get_node("l").unwrap_expr(),
            ctx.get_node("r").unwrap_expr(),
        ))
    });
    parser.define_expr(":l * :r", 30, |ctx: SyntaxCtx<AstNode>| {
        AstNode::Expr(Ir::mul(
            ctx.get_node("l").unwrap_expr(),
            ctx.get_node("r").unwrap_expr(),
        ))
    });
    parser.define_expr(":l / :r", 30, |ctx: SyntaxCtx<AstNode>| {
        AstNode::Expr(Ir::div(
            ctx.get_node("l").unwrap_expr(),
            ctx.get_node("r").unwrap_expr(),
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

    parser.define_expr(":target = :val", 2, |ctx: SyntaxCtx<AstNode>| {
        let target_expr = ctx.get_node("target").unwrap_expr();
        let name = match target_expr.kind {
            IrExprKind::VarRef(n) => n,
            _ => panic!("assignment target must be a variable"),
        };
        AstNode::Stmt(IrStmt::Assign {
            target: name,
            val: ctx.get_node("val").unwrap_expr(),
        })
    });

    parser.define_expr(":func ( $(:args),* )", 40, |ctx: SyntaxCtx<AstNode>| {
        let func_expr = ctx.get_node("func").unwrap_expr();
        let name = match func_expr.kind {
            IrExprKind::VarRef(n) => n,
            _ => panic!("function name must be identifier"),
        };
        let args = ctx
            .get_node_list("args")
            .into_iter()
            .map(|x: AstNode| x.unwrap_expr())
            .collect();
        AstNode::Expr(Ir::call(name, args))
    });

    parser.define_stmt("let @name = :val", |ctx: SyntaxCtx<AstNode>| {
        AstNode::Stmt(Ir::var_dec(
            ctx.get_ident("name"),
            ctx.get_node("val").unwrap_expr(),
        ))
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
