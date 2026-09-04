use abyss_nexus::nexus::HirId;

use crate::parser::Parser;

impl Parser<'_> {
    #[inline(always)]
    pub fn parse_not(&mut self) -> HirId {
        self.bump();
        let body = self.parse_expr(0);
        self.db.hir.alloc_not(body)
    }

    #[inline(always)]
    pub fn parse_amp(&mut self) -> HirId {
        self.bump();
        let body = self.parse_expr(0);
        self.db.hir.alloc_amp(body)
    }
}
