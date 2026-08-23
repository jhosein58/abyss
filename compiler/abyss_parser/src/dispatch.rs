use abyss_nexus::nexus::HirId;
use abyss_token::kind::TokenKind as Tk;

use crate::parser::Parser;

impl Parser<'_> {
    #[inline]
    pub fn dispatch_prefix(&mut self) -> HirId {
        let Some(tk) = self.peek() else {
            self.report_unexpected_eof(self.span());
            return self.db.hir.alloc_error();
        };

        match tk {
            Tk::IntLit => self.parse_int(),
            Tk::FloatLit => self.parse_float(),
            Tk::Ident => self.parse_ident(),
            Tk::OBrace => self.parse_block(),
            Tk::OParen => self.parse_paren(),
            Tk::Ret => self.parse_return_stmt(),
            Tk::If => self.parse_if(),

            _ => {
                self.report_expected_expression(self.span());
                self.db.hir.alloc_error()
            }
        }
    }

    #[inline]
    pub fn dispatch_infix(&mut self, op: Tk, lhs: HirId, right_bp: u8) -> HirId {
        match op {
            Tk::Colon => self.parse_var_decl(lhs, right_bp),
            Tk::ColonColon => self.parse_binding(lhs, right_bp),
            Tk::OParen => self.parse_call(lhs, right_bp),
            _ => self.parse_binary(op, lhs, right_bp), // FIXME: match ro az dakhel binary biar inja
        }
    }
}
