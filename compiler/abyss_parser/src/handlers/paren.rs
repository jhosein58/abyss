use abyss_nexus::nexus::HirId;
use abyss_token::kind::TokenKind;

use crate::parser::Parser;

const NONE: u32 = u32::MAX;

impl<'db, const H: bool> Parser<'db, H> {
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

        let mut args = vec![self.parse_param(first)];

        while self.peek() != Some(TokenKind::CParen) {
            let name = self.parse_expr(0);
            args.push(self.parse_param(name));
        }
        self.bump();

        if H {
            return self.parse_fn_tail(NONE);
        }

        let params = self.db.add_list_flat(&args);
        self.parse_fn_tail(params)
    }

    fn parse_param(&mut self, name: HirId) -> u32 {
        let ty = if self.peek() == Some(TokenKind::Comma) {
            NONE
        } else {
            self.parse_expr(0).0
        };

        if self.peek() == Some(TokenKind::Comma) {
            self.bump();
        }

        if H {
            return NONE;
        }
        self.db.hir.alloc_arg(name, ty).0
    }

    fn parse_fn_tail(&mut self, params: u32) -> HirId {
        let ret = if self.peek() == Some(TokenKind::OBrace) {
            NONE
        } else {
            self.parse_expr(0).0
        };

        let body = self.parse_block();

        if H {
            return HirId(0);
        }

        self.db.hir.alloc_function(params, ret, body.0)
    }
}
