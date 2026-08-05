use abyss_nexus::{nexus::HirId, storages::tokens::TokenId};

use crate::parser::Parser;

pub fn int(p: &mut Parser) -> HirId {
    let value = p.db.tokens.text(TokenId(p.cursor));
    let value = value.parse::<i64>().unwrap();
    p.bump();
    p.db.hir.alloc_int(p.db.ints.alloc(value))
}
