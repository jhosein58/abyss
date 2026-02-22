use abyss_lexer::token::TokenKind as Tk;

use crate::{
    ast::{Expr, ExprKind},
    error::ParseError,
    parser::{engine::PrattEngine, precedence::Precedence},
};

pub fn parse_sequence(eng: &mut PrattEngine) -> Result<Expr, ParseError> {
    eng.advance();

    if eng.current_token().kind == Tk::CBracket {
        eng.advance();
        return Ok(eng.new_expr(ExprKind::Sequence(vec![], None)));
    }

    let first_expr = eng.parse_expression_bp(Precedence::None)?;

    if eng.current_token().kind == Tk::Semi {
        eng.advance();
        let len_expr = eng.parse_expression_bp(Precedence::None)?;
        eng.expect(Tk::CBracket)?;
        return Ok(eng.new_expr(ExprKind::Sequence(
            vec![first_expr],
            Some(Box::new(len_expr)),
        )));
    }

    let mut items = vec![first_expr];

    while eng.current_token().kind == Tk::Comma {
        eng.advance();
        if eng.current_token().kind == Tk::CBracket {
            break;
        }
        items.push(eng.parse_expression_bp(Precedence::None)?);
    }

    eng.expect(Tk::CBracket)?;
    Ok(eng.new_expr(ExprKind::Sequence(items, None)))
}
