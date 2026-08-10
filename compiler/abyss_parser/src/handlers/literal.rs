use abyss_nexus::nexus::{HirId, TokenId};

use crate::parser::Parser;

impl<'db, const H: bool> Parser<'db, H> {
    pub fn parse_int(&mut self) -> HirId {
        if H {
            self.bump();
            return HirId::default();
        }

        let span = self.span();

        let value = self.db.tokens.text(TokenId(self.cursor));
        let value = value.parse::<i64>().unwrap();
        self.bump();
        let id = self.db.hir.alloc_int(self.db.ints.alloc(value));

        self.db.hir_spans.set(id, span);
        self.db.hir_files.set(id, self.file_id);
        id
    }

    pub fn parse_float(&mut self) -> HirId {
        if H {
            self.bump();
            return HirId::default();
        }

        let span = self.span();

        let value = self.db.tokens.text(TokenId(self.cursor));
        let value = value.parse::<f64>().unwrap();
        self.bump();
        let id = self.db.hir.alloc_float(self.db.floats.alloc(value));

        self.db.hir_spans.set(id, span);
        self.db.hir_files.set(id, self.file_id);
        id
    }
}
