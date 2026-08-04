use abyss_nexus::storages::{hir::storage::HirId, tokens::TokenId};

use crate::parser::Parser;

pub fn int(p: &mut Parser) -> HirId {
    let value = p.db.tokens.text(TokenId(p.cursor));
    let value = value.parse::<i64>().unwrap();
    p.bump();
    p.db.hir.alloc_int(p.db.literals.intern_int(value))
}
