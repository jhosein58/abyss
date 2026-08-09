use abyss_nexus::nexus::{HirId, TokenId};

use crate::parser::Parser;

impl<'db, const H: bool> Parser<'db, H> {
    pub fn parse_int(&mut self) -> HirId {
        if H {
            self.bump();
            return HirId::default();
        }

        let value = self.db.tokens.text(TokenId(self.cursor));
        let value = value.parse::<i64>().unwrap();
        self.bump();
        self.db.hir.alloc_int(self.db.ints.alloc(value))
    }
}
