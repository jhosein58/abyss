use abyss_hir::hir::HirExprKind;
use abyss_nexus::nexus::HirId;
use abyss_token::kind::TokenKind as Tk;

use crate::parser::Parser;

pub fn build(p: &mut Parser, op: Tk, lhs: HirId, rhs: HirId) -> HirId {
    let kind = match op {
        Tk::Plus => HirExprKind::BinaryAdd,
        Tk::Minus => HirExprKind::BinarySub,
        Tk::Star => HirExprKind::BinaryMul,
        Tk::Slash => HirExprKind::BinaryDiv,
        Tk::Percent => HirExprKind::BinaryMod,

        Tk::ColonColon => HirExprKind::Binding,
        _ => unreachable!(),
    };
    p.db.hir.alloc_binary(kind, lhs, rhs)
}
