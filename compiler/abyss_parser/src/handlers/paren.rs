use abyss_hir::hir::HirExprKind;
use abyss_nexus::nexus::HirId;
use abyss_token::kind::TokenKind;

use crate::parser::Parser;

const NONE: u32 = u32::MAX;

impl Parser<'_> {
    // FIXME: tarotamiz taresh kon
    pub fn parse_paren(&mut self) -> HirId {
        let start_span = self.span();
        self.bump();

        if self.optional(TokenKind::CParen) {
            return self.parse_fn_tail(NONE);
        }

        println!("ok");

        let first = self.parse_expr(0);

        // (expr)
        if self.optional(TokenKind::CParen) {
            self.db
                .hir_spans
                .set(first, start_span.merge(self.prev_span()));
            return first;
        }

        let mut args = Vec::new();
        let mut pending_names = vec![first];

        loop {
            if self.optional(TokenKind::Comma) {
            } else if self.peek() != Some(TokenKind::CParen) && self.peek().is_some() {
                let ty = self.parse_expr(0).0;

                for name in pending_names.drain(..) {
                    args.push(self.db.hir.alloc_arg(name, ty).0);
                }

                self.optional(TokenKind::Comma);
            }

            if self.peek() == Some(TokenKind::CParen) || self.peek().is_none() {
                break;
            }

            let name = self.parse_expr(0);
            if name != self.db.hir.alloc_error() {
                pending_names.push(name);
            } else {
                self.sync();
            }
        }

        for name in pending_names {
            args.push(self.db.hir.alloc_arg(name, NONE).0);
        }

        self.expect(TokenKind::CParen);

        let params = self.db.add_list_flat(&args);

        self.parse_fn_tail(params)
    }

    fn parse_fn_tail(&mut self, params: u32) -> HirId {
        let ret = if self.peek() == Some(TokenKind::OBrace) {
            NONE
        } else {
            self.parse_expr(0).0
        };

        // mark
        self.db.hir.alloc(HirExprKind::MarkerFnEnd, 0, 0, 0);

        let body = self.parse_block();

        self.db.hir.alloc_function(params, ret, body.0)
    }
}
