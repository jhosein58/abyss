use abyss_lexer::token::TokenKind as Tk;

use crate::{
    ast::{Expr, ExprKind},
    parser::{engine::PrattEngine, precedence::Precedence},
};

pub fn parse_index(eng: &mut PrattEngine, left: Expr) -> Result<Expr, ()> {
    eng.advance();

    let index = eng.parse_expression_bp(Precedence::None)?;

    eng.expect(Tk::CBracket)?;
    Ok(eng.new_expr(ExprKind::Index(Box::new(left), Box::new(index))))
}
