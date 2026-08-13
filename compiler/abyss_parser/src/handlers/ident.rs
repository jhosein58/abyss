use abyss_nexus::nexus::{HirId, TokenId};

use crate::parser::Parser;

impl Parser<'_> {
    pub fn parse_ident(&mut self) -> HirId {
        let span = self.span();

        self.bump();
        let value = self.db.tokens.text(TokenId(self.cursor - 1));
        let id = self.db.hir.alloc_ident(self.db.interner.intern(value));
        self.db.hir_spans.set(id, span);
        id
    }
}
