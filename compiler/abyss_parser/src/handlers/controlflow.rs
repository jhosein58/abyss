use abyss_nexus::nexus::HirId;
use abyss_token::kind::TokenKind as Tk;

use crate::parser::Parser;

impl Parser<'_> {
    pub fn parse_if(&mut self) -> HirId {
        self.bump();

        let cond = self.parse_expr(0);
        let thenb = self.parse_expr(0);

        let mut elseb = None;

        if self.optional(Tk::Else) {
            elseb = Some(self.parse_expr(0))
        }

        self.db.hir.alloc_if(cond, thenb, elseb)
    }
}
