use crate::{
    ast::{Expr, ExprKind},
    parser::{engine::PrattEngine, precedence::Precedence},
};

pub fn parse_cast(eng: &mut PrattEngine, left: Expr) -> Result<Expr, ()> {
    eng.advance();

    let target_type = eng.parse_expression_bp(Precedence::Cast)?;

    let span = left.span.clone().merge(target_type.span.clone());

    Ok(Expr {
        kind: ExprKind::Cast(Box::new(left), Box::new(target_type)),
        span,
        id: eng.next_id(),
    })
}
