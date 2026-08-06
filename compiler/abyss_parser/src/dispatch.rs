use abyss_nexus::nexus::{HirId, ScopeId};
use abyss_token::kind::TokenKind as Tk;

use crate::{
    engine,
    handlers::{
        infix::binary,
        prefix::{block, ident, literal},
    },
    parser::Parser,
};

#[inline]
pub fn prefix(p: &mut Parser, parent: ScopeId) -> HirId {
    match p.peek() {
        Some(Tk::IntLit) => literal::int(p),
        Some(Tk::Ident) => ident::handle(p),
        Some(Tk::OBrace) => block::handle(p, parent),

        _ => {
            p.bump();
            HirId(0)
        } // TODO: error node
    }
}

#[inline]
pub fn infix(p: &mut Parser, parent: ScopeId, op: Tk, lhs: HirId, right_bp: u8) -> HirId {
    let rhs = engine::parse_expr(p, right_bp, parent);
    binary::build(p, op, lhs, rhs)
}
