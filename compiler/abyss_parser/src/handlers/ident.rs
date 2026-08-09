use abyss_nexus::nexus::{HirId, TokenId};

use crate::parser::Parser;

pub fn handle<const H: bool>(p: &mut Parser<H>) -> HirId {
    if H {
        p.bump();
        return HirId::default();
    }

    p.bump();
    let value = p.db.tokens.text(TokenId(p.cursor - 1));
    p.db.hir.alloc_ident(p.db.interner.intern(value))
}
