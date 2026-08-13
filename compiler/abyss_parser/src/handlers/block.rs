use abyss_nexus::nexus::HirId;
use abyss_token::kind::TokenKind as Tk;

use crate::parser::Parser;

impl Parser<'_> {
    pub fn parse_block(&mut self) -> HirId {
        self.bump();

        let mut items = vec![];

        loop {
            if let Some(Tk::CBrace) = self.peek() {
                self.bump();
                break;
            }

            if self.peek().is_none() {
                break;
            }

            let item = self.parse_expr(0);

            items.push(item.0);

            if let Some(Tk::CBrace) = self.peek() {
                self.bump();
                break;
            }
        }

        let items = self.db.add_list_flat(&items);
        let hir_id = self.db.hir.alloc_block(items);

        hir_id
    }
}
