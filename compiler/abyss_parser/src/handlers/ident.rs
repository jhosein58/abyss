use abyss_nexus::nexus::{HirId, TokenId};

use crate::parser::Parser;

impl Parser<'_> {
    pub fn parse_ident(&mut self) -> HirId {
        let span = self.span();

        self.bump();
        let value = self.db.tokens.text(TokenId(self.cursor - 1));
        let name_id = self.db.interner.intern(value);
        let id = self.db.hir.alloc_ident(name_id);

        if let Some(sym_id) = self.env.lookup(name_id) {
            self.db.hir_to_symbol.set(id, sym_id);
        }

        self.db.hir_spans.set(id, span);
        id
    }
}
