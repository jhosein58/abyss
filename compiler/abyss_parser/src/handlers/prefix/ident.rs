use abyss_nexus::{nexus::HirId, storages::tokens::TokenId};

use crate::parser::Parser;

pub fn handle(p: &mut Parser) -> HirId {
    if p.is_headless {
        p.bump();
        return HirId::default();
    }

    p.bump();
    let value = p.db.tokens.text(TokenId(p.cursor - 1));
    p.db.hir.alloc_ident(p.db.interner.intern(value))
}
