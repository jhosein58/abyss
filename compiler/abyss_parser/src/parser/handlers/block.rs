use abyss_lexer::token::TokenKind as Tk;

use crate::{
    ast::{Expr, ExprKind},
    error::ParseError,
    parser::{engine::PrattEngine, precedence::Precedence},
};

pub fn parse_block(eng: &mut PrattEngine) -> Result<Expr, ParseError> {
    eng.advance();

    let mut stmts = Vec::new();

    while eng.current_token().kind != Tk::CBrace {
        let expr = eng.parse_expression_bp(Precedence::None)?;
        stmts.push(expr);
    }

    eng.expect(Tk::CBrace)?;

    Ok(eng.new_expr(ExprKind::Block(stmts)))
}
