use abyss_lexer::token::TokenKind as Tk;

use crate::{
    ast::{Expr, ExprKind},
    parser::{engine::PrattEngine, precedence::Precedence},
};

pub fn parse_wildcard(eng: &mut PrattEngine) -> Result<Expr, ()> {
    let span = eng.current_span();
    eng.advance();
    Ok(Expr {
        kind: ExprKind::Wildcard,
        span,
        id: eng.next_id(),
    })
}

pub fn parse_ret(eng: &mut PrattEngine) -> Result<Expr, ()> {
    let span = eng.current_span();
    eng.advance();

    let next_tk = eng.current_token();
    if next_tk.preceded_by_newline || next_tk.kind == Tk::CBrace {
        return Ok(Expr {
            kind: ExprKind::Ret(None),
            span,
            id: eng.next_id(),
        });
    }

    let val = eng.parse_expression_bp(Precedence::None)?;
    Ok(Expr {
        kind: ExprKind::Ret(Some(Box::new(val))),
        span,
        id: eng.next_id(),
    })
}

pub fn parse_out(eng: &mut PrattEngine) -> Result<Expr, ()> {
    let span = eng.current_span();
    eng.advance();

    let next_tk = eng.current_token();
    if next_tk.preceded_by_newline || next_tk.kind == Tk::CBrace {
        return Ok(Expr {
            kind: ExprKind::Out(None),
            span,
            id: eng.next_id(),
        });
    }

    let val = eng.parse_expression_bp(Precedence::None)?;
    Ok(Expr {
        kind: ExprKind::Out(Some(Box::new(val))),
        span,
        id: eng.next_id(),
    })
}

pub fn parse_continue(eng: &mut PrattEngine) -> Result<Expr, ()> {
    let span = eng.current_span();
    eng.advance();
    Ok(Expr {
        kind: ExprKind::Continue,
        span,
        id: eng.next_id(),
    })
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

    let mut else_branch = None;
    if eng.match_token(Tk::Else) {
        else_branch = Some(Box::new(eng.parse_expression_bp(Precedence::None)?));
    }

    Ok(Expr {
        kind: ExprKind::While(cond, body, else_branch),
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

pub fn parse_def(eng: &mut PrattEngine) -> Result<Expr, ()> {
    let tk = eng.get_and_bump();

    let name_expr = eng.parse_expression_bp(Precedence::Call)?;
    let value_expr = eng.parse_expression_bp(Precedence::None)?;

    Ok(Expr {
        kind: ExprKind::Def(Box::new(name_expr), Box::new(value_expr)),
        span: tk.span(eng.file_id),
        id: eng.next_id(),
    })
}

pub fn parse_comptime(eng: &mut PrattEngine) -> Result<Expr, ()> {
    let tk = eng.get_and_bump();

    let target_expr = eng.parse_expression_bp(Precedence::None)?;

    Ok(Expr {
        kind: ExprKind::Comptime(Box::new(target_expr)),
        span: tk.span(eng.file_id),
        id: eng.next_id(),
    })
}
