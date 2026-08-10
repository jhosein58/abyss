use abyss_nexus::nexus::HirId;
use abyss_token::kind::TokenKind as Tk;

use crate::parser::Parser;

impl<const H: bool> Parser<'_, H> {
    #[inline]
    pub fn dispatch_prefix(&mut self) -> HirId {
        let Some(tk) = self.peek() else {
            self.bump();
            return HirId(0);
        };

        match tk {
            Tk::IntLit => self.parse_int(),
            Tk::FloatLit => self.parse_float(),
            Tk::Ident => self.parse_ident(),
            Tk::OBrace => self.parse_block(),
            Tk::OParen => self.parse_paren(),

            _ => {
                self.bump();
                HirId(0)
            } // TODO: error node
        }
    }

    #[inline]
    pub fn dispatch_infix(&mut self, op: Tk, lhs: HirId, right_bp: u8) -> HirId {
        let rhs = self.parse_expr(right_bp);
        self.parse_binary(op, lhs, rhs)
    }
}
