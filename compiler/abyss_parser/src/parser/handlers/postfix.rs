use abyss_lexer::token::TokenKind as Tk;

use crate::{
    ast::{Expr, ExprKind},
    parser::{engine::PrattEngine, precedence::Precedence},
};

pub fn parse_index(eng: &mut PrattEngine, left: Expr) -> Result<Expr, ()> {
    let tk = eng.get_and_bump();

    let index = eng.parse_expression_bp(Precedence::None)?;

    let span = tk.span(eng.file_id).merge(eng.current_span());

    eng.expect(Tk::CBracket)?;

    Ok(Expr {
        kind: ExprKind::Index(Box::new(left), Box::new(index)),
        span,
        id: eng.next_id(),
    })
}
