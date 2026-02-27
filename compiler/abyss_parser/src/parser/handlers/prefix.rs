use abyss_lexer::token::TokenKind as Tk;

use crate::{
    ast::{Expr, ExprKind},
    parser::{engine::PrattEngine, precedence::Precedence},
};

pub fn parse_wildcard(eng: &mut PrattEngine) -> Result<Expr, ()> {
    eng.advance();
    Ok(eng.new_expr(ExprKind::Wildcard))
}

pub fn parse_ret(eng: &mut PrattEngine) -> Result<Expr, ()> {
    eng.advance();

    let next_tk = eng.current_token();
    if next_tk.preceded_by_newline || next_tk.kind == Tk::CBrace {
        return Ok(eng.new_expr(ExprKind::Ret(None)));
    }

    let val = eng.parse_expression_bp(Precedence::None)?;
    Ok(eng.new_expr(ExprKind::Ret(Some(Box::new(val)))))
}

pub fn parse_out(eng: &mut PrattEngine) -> Result<Expr, ()> {
    eng.advance();

    let next_tk = eng.current_token();
    if next_tk.preceded_by_newline || next_tk.kind == Tk::CBrace {
        return Ok(eng.new_expr(ExprKind::Out(None)));
    }

    let val = eng.parse_expression_bp(Precedence::None)?;
    Ok(eng.new_expr(ExprKind::Out(Some(Box::new(val)))))
}

pub fn parse_continue(eng: &mut PrattEngine) -> Result<Expr, ()> {
    eng.advance();
    Ok(eng.new_expr(ExprKind::Continue))
}

pub fn parse_if(engine: &mut PrattEngine) -> Result<Expr, ()> {
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

pub fn parse_while(eng: &mut PrattEngine) -> Result<Expr, ()> {
    let span = eng.current_span();
    eng.advance();

    let cond = Box::new(eng.parse_expression_bp(Precedence::None)?);

    let body = Box::new(eng.parse_expression_bp(Precedence::None)?);

    Ok(Expr {
        kind: ExprKind::While(cond, body),
        span,
        id: eng.next_id(),
    })
}

pub fn parse_forever(eng: &mut PrattEngine) -> Result<Expr, ()> {
    let span = eng.current_span();
    eng.advance();

    let body = Box::new(eng.parse_expression_bp(Precedence::None)?);

    Ok(Expr {
        kind: ExprKind::Forever(body),
        span,
        id: eng.next_id(),
    })
}
