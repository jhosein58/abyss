use abyss_nexus::storages::hir::storage::HirId;
use abyss_token::kind::TokenKind;

use crate::{
    engine,
    handlers::{infix::binary, prefix::literal},
    parser::Parser,
};

#[inline]
pub fn prefix(p: &mut Parser) -> HirId {
    match p.peek() {
        Some(TokenKind::IntLit) => literal::int(p),

        _ => {
            p.bump();
            HirId(0)
        } // TODO: error node
    }
}

#[inline]
pub fn infix(p: &mut Parser, op: TokenKind, lhs: HirId, right_bp: u8) -> HirId {
    let rhs = engine::parse_expr(p, right_bp);
    binary::build(p, op, lhs, rhs)
}
