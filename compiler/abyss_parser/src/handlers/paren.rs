use abyss_hir::hir::HirExprKind;
use abyss_nexus::{
    arena::ArenaId,
    nexus::{HirId, SymbolId},
};
use abyss_token::kind::TokenKind;

use crate::parser::Parser;

const NONE: u32 = u32::MAX;

impl Parser<'_> {
    #[inline(always)]
    fn define_param(&mut self, name_hir: HirId) {
        if self.db.hir.kind(name_hir) == HirExprKind::Ident {
            let symbol_id = self.db.symbols.alloc(name_hir);
            self.db.hir_to_symbol.set(name_hir, symbol_id);

            let name_id = self.db.hir.ident_name(name_hir);
            self.env.define(name_id, symbol_id);
        } else {
            self.report_invalid_binding_target(self.db.hir_spans.get_copy(name_hir));
        }
    }

    // FIXME: tarotamiz taresh kon
    pub fn parse_paren(&mut self) -> HirId {
        let start_span = self.span();
        self.bump();

        if self.optional(TokenKind::CParen) {
            let mark = self.env.mark(); // new scope
            let id = self.parse_fn_tail(NONE);
            self.env.reset(mark); // end of function scope
            return id;
        }

        let first = self.parse_expr(0);

        // group
        if self.optional(TokenKind::CParen) {
            self.db
                .hir_spans
                .set(first, start_span.merge(self.prev_span()));
            return first;
        }

        let mark = self.env.mark(); // enter scope
        self.define_param(first);

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

            self.define_param(name);
            pending_names.push(name);
        }

        for name in pending_names {
            args.push(self.db.hir.alloc_arg(name, NONE).0);
        }

        self.expect(TokenKind::CParen);

        let params = self.db.add_list_flat(&args);

        let id = self.parse_fn_tail(params);

        self.env.reset(mark);

        id
    }

    fn parse_fn_tail(&mut self, params: u32) -> HirId {
        let ret = if self.peek() == Some(TokenKind::OBrace) {
            NONE
        } else {
            self.parse_expr(0).0
        };

        let sym_id = self.toplv_sym;
        self.toplv_sym = SymbolId::none();

        // mark
        let mark = self
            .db
            .hir
            .alloc(HirExprKind::MarkerFnStart, 0, sym_id.0, 0);

        let body = self.parse_block();

        let id = self.db.hir.alloc_function(params, ret, body.0);

        // patch mark
        self.db.hir.table.lhs[mark.0 as usize] = id.0;

        id
    }
}
