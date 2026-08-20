use abyss_nexus::nexus::HirId;
use abyss_token::kind::TokenKind as Tk;

use crate::parser::Parser;

impl Parser<'_> {
    pub fn parse_call(&mut self) -> HirId {
        self.expect(Tk::CParen);

        self.db.hir.all
    }
}
