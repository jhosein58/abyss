use abyss_nexus::nexus::{HirId, TokenId};

use crate::parser::Parser;

pub fn int<const H: bool>(p: &mut Parser<H>) -> HirId {
    if H {
        p.bump();
        return HirId::default();
    }

    let value = p.db.tokens.text(TokenId(p.cursor));
    let value = value.parse::<i64>().unwrap();
    p.bump();
    p.db.hir.alloc_int(p.db.ints.alloc(value))
}
