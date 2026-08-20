use abyss_nexus::nexus::HirId;
use abyss_token::kind::TokenKind as Tk;

use crate::parser::Parser;

impl Parser<'_> {
    pub fn parse_call(&mut self, lhs: HirId, rbp: u8) -> HirId {
        self.expect(Tk::CParen);

        self.db.hir.alloc_call(0)
    }
}
