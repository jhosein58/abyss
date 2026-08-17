use abyss_hir::hir::HirExprKind;
use abyss_nexus::nexus::HirId;
use abyss_token::kind::TokenKind as Tk;

use crate::parser::Parser;

impl Parser<'_> {
    pub fn parse_binary(&mut self, op: Tk, lhs: HirId, right_bp: u8) -> HirId {
        let rhs = self.parse_expr(right_bp);

        let lhs_span = self.db.hir_spans.get(lhs);
        let rhs_span = self.db.hir_spans.get(rhs);

        let kind = match op {
            Tk::Plus => HirExprKind::BinaryAdd,
            Tk::Minus => HirExprKind::BinarySub,
            Tk::Star => HirExprKind::BinaryMul,
            Tk::Slash => HirExprKind::BinaryDiv,
            Tk::Percent => HirExprKind::BinaryMod,
            Tk::Eq => HirExprKind::BinaryAssign,
            _ => return self.db.hir.alloc_error(),
        };

        let id = self.db.hir.alloc_binary(kind, lhs, rhs);
        self.db.hir_spans.set(id, lhs_span.merge(*rhs_span));
        self.db.hir_files.set(id, self.file_id);
        id
    }
}
