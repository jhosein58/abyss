use abyss_nexus::nexus::HirId;
use abyss_token::kind::TokenKind as Tk;

use crate::parser::Parser;

impl Parser<'_> {
    pub fn parse_block(&mut self) -> HirId {
        let start_span = self.span();
        self.bump();

        let mut items = Vec::with_capacity(16);

        // enter scope
        let mark = self.env.mark();

        loop {
            match self.peek() {
                Some(Tk::CBrace) => {
                    self.bump();
                    break;
                }
                None => {
                    self.report_unexpected_token(Tk::CBrace);
                    break;
                }
                _ => {
                    let item = self.parse_expr(0);
                    if item != HirId(0) {
                        items.push(item.0);
                    } else {
                        self.sync();
                    }
                }
            }
        }

        // exit scope
        self.env.reset(mark);

        let items = self.db.add_list_flat(&items);
        let hir_id = self.db.hir.alloc_block(items);

        let end_span = self.prev_span();
        self.db.hir_spans.set(hir_id, start_span.merge(end_span));

        hir_id
    }
}
