use abyss_nexus::{arena::ArenaId, nexus::HirId};
use abyss_token::kind::TokenKind as Tk;

use crate::parser::Parser;

impl Parser<'_> {
    pub fn parse_array(&mut self) -> HirId {
        self.bump(); // [

        let first = self.parse_expr(0);

        // Array Type
        if self.optional(Tk::Semi) {
            let arr_len = self.parse_expr(0);
            self.expect(Tk::CBracket);
            return self.db.hir.alloc_array(first, arr_len);
        }

        HirId::none()
    }
}
