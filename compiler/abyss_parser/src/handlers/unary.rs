use abyss_nexus::nexus::HirId;

use crate::parser::Parser;

impl Parser<'_> {
    #[inline(always)]
    pub fn parse_not(&mut self) -> HirId {
        self.bump();
        let body = self.parse_expr(0);
        self.db.hir.alloc_not(body)
    }

    // IDEA: combine all methods to a one single methode called "parse_unary"

    #[inline(always)]
    pub fn parse_addrof(&mut self) -> HirId {
        self.bump();
        let inner = self.parse_expr(0);
        self.db.hir.alloc_addrof(inner)
    }

    #[inline(always)]
    pub fn parse_deref(&mut self) -> HirId {
        self.bump();
        let inner = self.parse_expr(0);
        self.db.hir.alloc_deref(inner)
    }
}
