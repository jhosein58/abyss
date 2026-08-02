use abyss_hir::hir::HirExprKind;
use abyss_nexus::storages::hir::storage::HirId;
use abyss_token::kind::TokenKind;

use crate::parser::Parser;

pub fn build(p: &mut Parser, op: TokenKind, lhs: HirId, rhs: HirId) -> HirId {
    let kind = match op {
        TokenKind::Plus => HirExprKind::BinaryAdd,
        TokenKind::Minus => HirExprKind::BinarySub,
        TokenKind::Star => HirExprKind::BinaryMul,
        _ => unreachable!(),
    };
    p.db.hir.alloc_binary(kind, lhs, rhs)
}
