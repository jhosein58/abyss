use abyss_nexus::nexus::{HirId, TokenId};

use crate::parser::Parser;

impl<'db, const H: bool> Parser<'db, H> {
    pub fn parse_ident(&mut self) -> HirId {
        if H {
            self.bump();
            return HirId::default();
        }

        let span = self.span();

        self.bump();
        let value = self.db.tokens.text(TokenId(self.cursor - 1));
        let id = self.db.hir.alloc_ident(self.db.interner.intern(value));
        self.db.hir_spans.set(id, span);
        id
    }
}
