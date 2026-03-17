use abyss_lexer::token::TokenKind as Tk;

use crate::{
    ast::{Expr, ExprKind},
    parser::{engine::PrattEngine, precedence::Precedence},
};

pub fn parse_call(eng: &mut PrattEngine, left: Expr) -> Result<Expr, ()> {
    let oparen_tk = eng.get_and_bump();

    let mut args = Vec::new();

    while eng.current_token().kind != Tk::CParen {
        args.push(eng.parse_expression_bp(Precedence::None)?);

        if eng.current_token().kind == Tk::Comma {
            eng.advance();
        } else {
            break;
        }
    }

    let cparen_span = eng.current_span();
    eng.expect(Tk::CParen)?;

    Ok(Expr {
        kind: ExprKind::Call(Box::new(left), args),
        span: oparen_tk.span(eng.file_id).merge(cparen_span),
        id: eng.next_id(),
    })
}

pub fn parse_member(eng: &mut PrattEngine, left: Expr) -> Result<Expr, ()> {
    let dot_tk = eng.get_and_bump();

    let token = eng.current_token();
    if !matches!(token.kind, Tk::Ident | Tk::IntLit) {
        eng.report_error(
            token.span(eng.file_id),
            format!("expected identifier after '.', found {:?}", token.kind),
        );
        return Err(());
    }

    let member_name = token.text.to_string();
    eng.advance();

    Ok(Expr {
        kind: ExprKind::Member(Box::new(left), member_name),
        span: dot_tk.span(eng.file_id).merge(token.span(eng.file_id)),
        id: eng.next_id(),
    })
}
