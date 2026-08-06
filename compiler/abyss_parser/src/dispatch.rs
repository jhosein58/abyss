use abyss_nexus::nexus::HirId;
use abyss_token::kind::TokenKind as Tk;

use crate::{
    handlers::{
        infix::binary,
        prefix::{block, ident, literal},
    },
    parser::Parser,
};

impl Parser<'_> {
    #[inline]
    pub fn dispatch_prefix(&mut self) -> HirId {
        match self.peek() {
            Some(Tk::IntLit) => literal::int(self),
            Some(Tk::Ident) => ident::handle(self),
            Some(Tk::OBrace) => block::handle(self),

            _ => {
                self.bump();
                HirId(0)
            } // TODO: error node
        }
    }

    #[inline]
    pub fn dispatch_infix(&mut self, op: Tk, lhs: HirId, right_bp: u8) -> HirId {
        let rhs = self.parse_expr(right_bp);
        binary::build(self, op, lhs, rhs)
    }
}
