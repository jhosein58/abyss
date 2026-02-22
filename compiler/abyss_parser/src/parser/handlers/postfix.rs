use abyss_lexer::token::TokenKind as Tk;

use crate::{
    ast::{Expr, ExprKind},
    error::ParseError,
    parser::{engine::PrattEngine, precedence::Precedence},
};

pub fn parse_index(eng: &mut PrattEngine, left: Expr) -> Result<Expr, ParseError> {
    eng.advance();

    let index = eng.parse_expression_bp(Precedence::None)?;

    eng.expect(Tk::CBracket)?;
    Ok(eng.new_expr(ExprKind::Index(Box::new(left), Box::new(index))))
}

pub fn parse_if(engine: &mut PrattEngine) -> Result<Expr, ParseError> {
    let span = engine.current_span();
    engine.advance();
    let cond = Box::new(engine.parse_expression_bp(Precedence::None)?);

    let then_branch = Box::new(engine.parse_expression_bp(Precedence::None)?);

    let mut else_branch = None;
    if engine.match_token(Tk::Else) {
        else_branch = Some(Box::new(engine.parse_expression_bp(Precedence::None)?));
    }

    Ok(Expr {
        kind: ExprKind::If(cond, then_branch, else_branch),
        span,
        id: engine.next_id(),
    })
}
