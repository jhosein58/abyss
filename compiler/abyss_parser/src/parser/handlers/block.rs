use abyss_lexer::token::TokenKind as Tk;

use crate::{
    ast::{Expr, ExprKind},
    parser::{engine::PrattEngine, precedence::Precedence},
};

pub fn parse_block(eng: &mut PrattEngine) -> Result<Expr, ()> {
    let obrace_tk = eng.get_and_bump();

    let mut stmts = Vec::new();

    while eng.current_token().kind != Tk::CBrace && !eng.is_eof() {
        match eng.parse_expression_bp(Precedence::None) {
            Ok(expr) => stmts.push(expr),
            Err(_) => {
                eng.synchronize();
            }
        }

        let current = eng.current_token();

        if current.kind == Tk::CBrace {
            break;
        }

        if current.kind == Tk::Comma {
            eng.advance();
        } else if current.preceded_by_newline {
            continue;
        } else {
            let span = eng.current_span();
            eng.report_error(
                span,
                "Expected `,` or newline to separate statements.".to_string(),
            );
            eng.synchronize();
        }
    }

    let cbrace_span = eng.current_span();
    eng.expect(Tk::CBrace)?;

    Ok(Expr {
        kind: ExprKind::Block(stmts),
        span: obrace_tk.span(eng.file_id).merge(cbrace_span),
        id: eng.next_id(),
    })
}
