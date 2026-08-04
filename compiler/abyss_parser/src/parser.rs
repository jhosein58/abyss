use abyss_nexus::{nexus::Nexus, storages::tokens::storage::TokenId};
use abyss_token::kind::TokenKind;

use crate::engine;

pub struct Parser<'db> {
    pub db: &'db mut Nexus,
    pub cursor: u32,
    end: u32,
}

impl Parser<'_> {
    #[inline(always)]
    pub fn peek(&self) -> Option<TokenKind> {
        (self.cursor < self.end).then(|| self.db.tokens.kind(TokenId(self.cursor)))
    }

    #[inline(always)]
    pub fn bump(&mut self) -> TokenId {
        let id = TokenId(self.cursor);
        self.cursor += 1;
        id
    }

    #[inline(always)]
    pub fn peek_preceded_by_newline(&self) -> bool {
        self.cursor < self.end && self.db.tokens.preceded_by_newline(TokenId(self.cursor))
    }

    pub fn parse(db: &mut Nexus, cursor: u32, end: u32) {
        let mut parser = Parser { db, cursor, end };

        let mut items = vec![];

        loop {
            if let Some(TokenKind::Eof) = parser.peek() {
                break;
            }

            items.push(engine::parse_expr(&mut parser, 0).0);

            if let Some(_) = parser.peek() {
                continue;
            }
            break;
        }

        let items = parser.db.add_list_flat(&items);
        let root = parser.db.hir.alloc_block(items);

        parser.db.hir.set_root(root);
    }
}
