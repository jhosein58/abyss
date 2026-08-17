use abyss_hir::hir::HirExprKind;
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
                    if self.db.hir.kind(item) != HirExprKind::Error {
                        items.push(item.0);
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

    // #[inline(always)]
    // pub fn parse_return_stmt(&mut self, id: HirId) -> HirId {}
}
