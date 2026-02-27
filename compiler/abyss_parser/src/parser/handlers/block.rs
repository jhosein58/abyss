use abyss_lexer::token::TokenKind as Tk;

use crate::{
    ast::{Expr, ExprKind},
    parser::{engine::PrattEngine, precedence::Precedence},
};

pub fn parse_block(eng: &mut PrattEngine) -> Result<Expr, ()> {
    eng.advance();

    let mut stmts = Vec::new();

    while eng.current_token().kind != Tk::CBrace && !eng.is_eof() {
        match eng.parse_expression_bp(Precedence::None) {
            Ok(expr) => stmts.push(expr),
            Err(_) => {
                eng.synchronize();
            }
        }
    }

    eng.expect(Tk::CBrace)?;

    Ok(eng.new_expr(ExprKind::Block(stmts)))
}
