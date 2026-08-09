use std::u32;

use abyss_nexus::nexus::HirId;
use abyss_token::kind::TokenKind;

use crate::parser::Parser;

impl<'db, const H: bool> Parser<'db, H> {
    pub fn parse_paren(&mut self) -> HirId {
        self.bump(); // (

        if let Some(TokenKind::CParen) = self.peek() {
            self.bump(); // )

            if let Some(TokenKind::OBrace) = self.peek() {
                self.bump(); // {

                let body = self.parse_block();

                if H {
                    return HirId(0);
                }
                //                                None      None      Id
                return self.db.hir.alloc_function(u32::MAX, u32::MAX, body.0);
            }

            let ret = self.parse_expr(0);
            let body = self.parse_expr(0);

            if H {
                return HirId(0);
            }
            return self.db.hir.alloc_function(u32::MAX, ret.0, body.0);
        }

        let first_arg =

        HirId(0)
    }
}
