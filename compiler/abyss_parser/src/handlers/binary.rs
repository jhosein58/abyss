use abyss_hir::hir::HirExprKind as Hir;
use abyss_nexus::nexus::HirId;
use abyss_token::kind::TokenKind as Tk;

use crate::parser::Parser;

impl Parser<'_> {
    pub fn parse_binary(&mut self, op: Tk, lhs: HirId, right_bp: u8) -> HirId {
        let rhs = self.parse_expr(right_bp);

        let lhs_span = self.db.hir_spans.get(lhs);
        let rhs_span = self.db.hir_spans.get(rhs);

        let kind = match op {
            Tk::Plus => Hir::BinaryAdd,
            Tk::Minus => Hir::BinarySub,
            Tk::Star => Hir::BinaryMul,
            Tk::Slash => Hir::BinaryDiv,
            Tk::Percent => Hir::BinaryMod,
            Tk::Eq => Hir::BinaryAssign,

            // logical
            Tk::And => Hir::BinaryAnd,
            Tk::Or => Hir::BinaryOr,

            // comp
            Tk::Lt => Hir::BinaryLt,
            Tk::LtEq => Hir::BinaryLtEq,
            Tk::Gt => Hir::BinaryGt,
            Tk::GtEq => Hir::BinaryGtEq,
            Tk::EqEq => Hir::BinaryEqEq,
            Tk::BangEq => Hir::BinaryNeq,

            Tk::As => Hir::Cast,

            Tk::Dot => Hir::Member,

            _ => return self.db.hir.alloc_error(),
        };

        let id = self.db.hir.alloc_binary(kind, lhs, rhs);
        self.db.hir_spans.set(id, lhs_span.merge(*rhs_span));
        self.db.hir_files.set(id, self.file_id);
        id
    }
}
