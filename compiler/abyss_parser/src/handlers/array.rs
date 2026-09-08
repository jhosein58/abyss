use abyss_hir::hir::HirExprKind as Hir;
use abyss_nexus::{
    arena::ArenaId,
    nexus::{HirId, IntId},
};
use abyss_token::kind::TokenKind as Tk;

use crate::parser::Parser;

impl Parser<'_> {
    pub fn parse_array(&mut self) -> HirId {
        self.bump(); // [

        let first = self.parse_expr(0);

        // Array Type
        if self.optional(Tk::Semi) {
            let arr_len_id = self.parse_expr(0);

            if self.db.hir.kind(arr_len_id) != Hir::LitInt {
                panic!()
            }

            let array_int_id = self.db.hir.lhs(arr_len_id).0;

            let array_len = self.db.ints.get_copy(IntId(array_int_id)) as u32;

            self.expect(Tk::CBracket);
            return self.db.hir.alloc_array(first, array_len);
        }

        HirId::none()
    }
}
