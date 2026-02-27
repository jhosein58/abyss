use abyss_lexer::token::TokenKind as Tk;

use crate::{
    ast::{Expr, ExprKind, UnaryOp},
    parser::{engine::PrattEngine, precedence::Precedence},
};

pub fn parse_unary(eng: &mut PrattEngine) -> Result<Expr, ()> {
    let span = eng.current_span();
    let tk = eng.current_token();
    eng.advance();

    let right = Box::new(eng.parse_expression_bp(Precedence::Unary)?);

    let kind = match tk.kind {
        Tk::Minus => ExprKind::Unary(UnaryOp::Neg, right),
        Tk::Not => ExprKind::Unary(UnaryOp::Not, right),
        Tk::Tilde => ExprKind::Unary(UnaryOp::BitNot, right),
        Tk::Star => ExprKind::Unary(UnaryOp::Deref, right),
        Tk::Amp => ExprKind::Unary(UnaryOp::AddrOf, right),
        _ => unreachable!("Invalid unary operator"),
    };

    Ok(Expr { kind, span, id: 0 })
}
