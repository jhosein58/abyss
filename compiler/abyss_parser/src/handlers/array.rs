use abyss_hir::hir::HirExprKind as Hir;
use abyss_nexus::nexus::{HirId, IntId};
use abyss_token::kind::TokenKind as Tk;

use crate::parser::Parser;

impl Parser<'_> {
    pub fn parse_array(&mut self) -> HirId {
        self.bump(); // [

        if self.optional(Tk::CBracket) {
            let empty_list = self.db.add_list_flat(&[]);
            return self.db.hir.alloc_array_init(empty_list);
        }

        let first = self.parse_expr(0);

        if self.optional(Tk::Semi) {
            let arr_len_id = self.parse_expr(0);

            if self.db.hir.kind(arr_len_id) != Hir::LitInt {
                panic!();
            }

            let array_int_id = self.db.hir.lhs(arr_len_id).0;
            let array_len = self.db.ints.get_copy(IntId(array_int_id)) as u32;

            self.expect(Tk::CBracket);
            return self.db.hir.alloc_array(first, array_len);
        }

        let mut list = Vec::with_capacity(8);
        list.push(first.0);

        while self.optional(Tk::Comma) {
            if self.peek() == Some(Tk::CBracket) {
                break;
            }
            let node = self.parse_expr(0);
            list.push(node.0);
        }

        self.expect(Tk::CBracket);

        let list_id = self.db.add_list_flat(&list);
        self.db.hir.alloc_array_init(list_id)
    }

    pub fn parse_index(&mut self, lhs: HirId, _: u8) -> HirId {
        let expr = self.parse_expr(0);
        self.expect(Tk::CBracket);
        self.db.hir.alloc_index(lhs, expr)
    }
}
