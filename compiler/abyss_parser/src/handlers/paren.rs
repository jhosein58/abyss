use abyss_nexus::nexus::HirId;
use abyss_token::kind::TokenKind;

use crate::parser::Parser;

const NONE: u32 = u32::MAX;

impl Parser<'_> {
    pub fn parse_paren(&mut self) -> HirId {
        self.bump();

        if self.peek() == Some(TokenKind::CParen) {
            self.bump();
            return self.parse_fn_tail(NONE);
        }

        let first = self.parse_expr(0);

        if self.peek() == Some(TokenKind::CParen) {
            self.bump();
            return first;
        }

        let mut args = Vec::new();
        let mut pending_names = vec![first];

        loop {
            if self.peek() == Some(TokenKind::Comma) {
                self.bump();
            } else if self.peek() != Some(TokenKind::CParen) && self.peek().is_some() {
                let ty = self.parse_expr(0).0;

                for name in pending_names.drain(..) {
                    args.push(self.db.hir.alloc_arg(name, ty).0);
                }

                if self.peek() == Some(TokenKind::Comma) {
                    self.bump();
                }
            }

            if self.peek() == Some(TokenKind::CParen) || self.peek().is_none() {
                break;
            }

            let name = self.parse_expr(0);
            pending_names.push(name);
        }

        for name in pending_names {
            args.push(self.db.hir.alloc_arg(name, NONE).0);
        }

        if self.peek() == Some(TokenKind::CParen) {
            self.bump();
        }

        let params = self.db.add_list_flat(&args);
        self.parse_fn_tail(params)
    }

    fn parse_fn_tail(&mut self, params: u32) -> HirId {
        let ret = if self.peek() == Some(TokenKind::OBrace) {
            NONE
        } else {
            self.parse_expr(0).0
        };

        let body = self.parse_block();

        self.db.hir.alloc_function(params, ret, body.0)
    }
}
