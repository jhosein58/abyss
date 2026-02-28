use abyss_lexer::token::TokenKind as Tk;

use crate::{
    ast::{Expr, ExprKind},
    parser::{engine::PrattEngine, precedence::Precedence},
};

pub fn parse_sequence(eng: &mut PrattEngine) -> Result<Expr, ()> {
    let start_span = eng.current_span();
    eng.advance();

    if eng.current_token().kind == Tk::CBracket {
        let end_span = eng.current_span();
        eng.advance();
        return Ok(Expr {
            kind: ExprKind::Sequence(vec![], None),
            span: start_span.merge(end_span),
            id: eng.next_id(),
        });
    }

    let first_expr = eng.parse_expression_bp(Precedence::None)?;

    if eng.current_token().kind == Tk::Semi {
        eng.advance();
        let len_expr = eng.parse_expression_bp(Precedence::None)?;
        let end_span = eng.current_span();
        eng.expect(Tk::CBracket)?;
        return Ok(Expr {
            kind: ExprKind::Sequence(vec![first_expr], Some(Box::new(len_expr))),
            span: start_span.merge(end_span),
            id: eng.next_id(),
        });
    }

    let mut items = vec![first_expr];

    while eng.current_token().kind == Tk::Comma {
        eng.advance();
        if eng.current_token().kind == Tk::CBracket {
            break;
        }
        items.push(eng.parse_expression_bp(Precedence::None)?);
    }

    let end_span = eng.current_span();
    eng.expect(Tk::CBracket)?;
    Ok(Expr {
        kind: ExprKind::Sequence(items, None),
        span: start_span.merge(end_span),
        id: eng.next_id(),
    })
}
