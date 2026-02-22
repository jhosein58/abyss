use crate::{
    ast::{Expr, ExprKind},
    error::ParseError,
    parser::{engine::PrattEngine, precedence::Precedence},
};

pub fn parse_wildcard(eng: &mut PrattEngine) -> Result<Expr, ParseError> {
    eng.advance();
    Ok(eng.new_expr(ExprKind::Wildcard))
}

pub fn parse_ret(eng: &mut PrattEngine) -> Result<Expr, ParseError> {
    eng.advance();

    let next_tk = eng.current_token().preceded_by_newline;
    if next_tk {
        return Ok(eng.new_expr(ExprKind::Ret(None)));
    }

    let val = eng.parse_expression_bp(Precedence::None)?;
    Ok(eng.new_expr(ExprKind::Ret(Some(Box::new(val)))))
}

pub fn parse_break(eng: &mut PrattEngine) -> Result<Expr, ParseError> {
    eng.advance();
    Ok(eng.new_expr(ExprKind::Break))
}

pub fn parse_continue(eng: &mut PrattEngine) -> Result<Expr, ParseError> {
    eng.advance();
    Ok(eng.new_expr(ExprKind::Continue))
}
