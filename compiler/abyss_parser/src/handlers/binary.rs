use abyss_hir::hir::HirExprKind;
use abyss_nexus::nexus::HirId;
use abyss_token::kind::TokenKind as Tk;

use crate::parser::Parser;

impl<'db, const H: bool> Parser<'db, H> {
    pub fn parse_binary(&mut self, op: Tk, lhs: HirId, rhs: HirId) -> HirId {
        if H {
            return HirId::default();
        }

        let kind = match op {
            Tk::Plus => HirExprKind::BinaryAdd,
            Tk::Minus => HirExprKind::BinarySub,
            Tk::Star => HirExprKind::BinaryMul,
            Tk::Slash => HirExprKind::BinaryDiv,
            Tk::Percent => HirExprKind::BinaryMod,

            Tk::ColonColon => HirExprKind::Binding,
            _ => unreachable!(),
        };
        self.db.hir.alloc_binary(kind, lhs, rhs)
    }
}
