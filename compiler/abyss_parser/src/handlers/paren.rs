use abyss_hir::hir::HirExprKind;
use abyss_nexus::{
    arena::ArenaId,
    nexus::{HirId, SymbolId, TokenId},
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

    fn is_function_header(&self) -> bool {
        let mut depth = 1;
        let mut i = self.cursor;
        let mut has_comma = false;

        while i < self.end {
            let tk = self.db.tokens.kind(TokenId(i));
            match tk {
                TokenKind::OParen => depth += 1,
                TokenKind::CParen => {
                    depth -= 1;
                    if depth == 0 {
                        break;
                    }
                }
                TokenKind::Comma if depth == 1 => {
                    has_comma = true;
                }
                _ => {}
            }
            i += 1;
        }

        if depth != 0 {
            return false;
        }

        if has_comma {
            return true;
        }

        let after_cp = if i + 1 < self.end {
            Some(self.db.tokens.kind(TokenId(i + 1)))
        } else {
            None
        };

        match after_cp {
            Some(TokenKind::OBrace) => true,

            Some(TokenKind::Ident) => !self.db.tokens.preceded_by_newline(TokenId(i + 1)),

            Some(TokenKind::Amp) | Some(TokenKind::Star) | Some(TokenKind::OBracket) => {
                self.toplv_sym.is_some()
            }

            _ => false,
        }
    }

    pub fn parse_paren(&mut self) -> HirId {
        let start_span = self.span();
        self.bump();
        if self.optional(TokenKind::CParen) {
            let mark = self.env.mark();
            let id = self.parse_fn_tail(NONE);
            self.env.reset(mark);
            return id;
        }

        if self.is_function_header() {
            let mark = self.env.mark();
            let mut args = Vec::new();
            let mut pending_names = Vec::new();

            while self.peek() != Some(TokenKind::CParen) && !self.is_eof() {
                let name = self.parse_ident();
                self.define_param(name);
                pending_names.push(name);

                if self.optional(TokenKind::Comma) {
                    continue;
                }

                if self.peek() != Some(TokenKind::CParen) && self.peek().is_some() {
                    let ty = self.parse_expr(0).0;

                    for name in pending_names.drain(..) {
                        args.push(self.db.hir.alloc_arg(name, ty).0);
                    }

                    self.optional(TokenKind::Comma);
                }
            }

            for name in pending_names {
                args.push(self.db.hir.alloc_arg(name, NONE).0);
            }

            self.expect(TokenKind::CParen);

            let params = self.db.add_list_flat(&args);
            let id = self.parse_fn_tail(params);
            self.env.reset(mark);
            return id;
        }

        let first = self.parse_expr(0);
        self.expect(TokenKind::CParen);
        self.db
            .hir_spans
            .set(first, start_span.merge(self.prev_span()));

        first
    }

    fn parse_fn_tail(&mut self, params: u32) -> HirId {
        let ret = if self.peek() == Some(TokenKind::OBrace) {
            NONE
        } else {
            self.parse_expr(0).0
        };

        let sym_id = self.toplv_sym;
        self.toplv_sym = SymbolId::none();

        let mark = self
            .db
            .hir
            .alloc(HirExprKind::MarkerFnStart, 0, sym_id.0, 0);

        let body = if self.peek() == Some(TokenKind::OBrace) {
            self.parse_block()
        } else {
            HirId::none()
        };

        let id = self.db.hir.alloc_function(params, ret, body.0);

        self.db.hir.table.lhs[mark.0 as usize] = id.0;

        id
    }
}
