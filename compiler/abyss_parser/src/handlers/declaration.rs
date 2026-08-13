use abyss_nexus::nexus::HirId;
use abyss_token::kind::TokenKind as Tk;

use crate::parser::Parser;

impl<'db, const H: bool> Parser<'db, H> {
    pub fn parse_var_decl(&mut self, lhs: HirId, right_bp: u8) -> HirId {
        if self.optional(Tk::Eq) {
            let value = self.parse_expr(0);
            if H {
                return HirId::default();
            }
            return self.db.hir.alloc_var(lhs, None, Some(value));
        }

        let rhs = self.parse_expr(right_bp);

        if self.optional(Tk::Eq) {
            let value = self.parse_expr(0);
            if H {
                return HirId::default();
            }
            return self.db.hir.alloc_var(lhs, Some(rhs), Some(value));
        }

        if H {
            return HirId::default();
        }
        return self.db.hir.alloc_var(lhs, Some(rhs), None);
    }

    pub fn parse_binding(&mut self, lhs: HirId, right_bp: u8) -> HirId {
        let rhs = self.parse_expr(right_bp);
        if H {
            return HirId::default();
        }
        return self.db.hir.alloc_binding(lhs, None, Some(rhs));
    }
}
