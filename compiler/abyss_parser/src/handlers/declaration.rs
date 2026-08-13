use abyss_nexus::nexus::HirId;
use abyss_token::kind::TokenKind as Tk;

use crate::{parser::Parser, precedence::Precedence};

impl Parser<'_> {
    pub fn parse_var_decl(&mut self, lhs: HirId, _: u8) -> HirId {
        if self.optional(Tk::Eq) {
            let value = self.parse_expr(0);

            return self.db.hir.alloc_var(lhs, None, Some(value));
        }

        let rhs = self.parse_expr(Precedence::VarDef.value() + 1);

        if self.optional(Tk::Eq) {
            let value = self.parse_expr(0);

            return self.db.hir.alloc_var(lhs, Some(rhs), Some(value));
        } else if self.optional(Tk::Colon) {
            let value = self.parse_expr(0);

            return self.db.hir.alloc_binding(lhs, Some(rhs), Some(value));
        }

        return self.db.hir.alloc_var(lhs, Some(rhs), None);
    }

    pub fn parse_binding(&mut self, lhs: HirId, right_bp: u8) -> HirId {
        let rhs = self.parse_expr(right_bp);

        return self.db.hir.alloc_binding(lhs, None, Some(rhs));
    }
}
