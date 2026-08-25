use abyss_nexus::{arena::ArenaId, nexus::HirId};
use abyss_token::kind::TokenKind::{self as Tk, CBrace};

use crate::parser::Parser;

impl Parser<'_> {
    #[inline]
    pub fn parse_struct(&mut self) -> HirId {
        self.bump();
        self.expect(Tk::OBrace);

        loop {
            if self.peek() == Some(CBrace) {
                break;
            }
        }

        self.expect(Tk::CBrace);

        HirId::none()
    }
}
