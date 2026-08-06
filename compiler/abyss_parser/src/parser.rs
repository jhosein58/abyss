use abyss_nexus::{nexus::Nexus, storages::tokens::TokenId};
use abyss_token::kind::TokenKind;

pub struct Parser<'db> {
    pub db: &'db mut Nexus,
    pub cursor: u32,
    end: u32,
    is_headless: bool,
}

impl<'a> Parser<'a> {
    pub fn new_indexer(db: &'a mut Nexus) -> Self {
        let end = db.tokens.count() as u32;

        Parser {
            db,
            cursor: 0,
            end,
            is_headless: false,
        }
    }

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

    pub fn index(&mut self) {
        let mut idxs: Vec<(u32, u32)> = vec![];

        loop {
            if let Some(TokenKind::Eof) = self.peek() {
                break;
            }

            let start = self.cursor;
            self.parse_expr(0);
            let end = self.cursor;

            idxs.push((start, end));

            if let Some(_) = self.peek() {
                continue;
            }
            break;
        }
    }

    // pub fn parse(db: &mut Nexus, cursor: u32, end: u32) {
    //     let mut parser = Parser {
    //         db,
    //         cursor,
    //         end,
    //         is_headless: false,
    //     };

    //     let mut items = vec![];

    //     loop {
    //         if let Some(TokenKind::Eof) = parser.peek() {
    //             break;
    //         }

    //         items.push(parser.parse_expr(0).0);

    //         if let Some(_) = parser.peek() {
    //             continue;
    //         }
    //         break;
    //     }

    //     let items = parser.db.add_list_flat(&items);
    //     let root = parser.db.hir.alloc_block(items);

    //     parser.db.hir.set_root(root);
    // }
}
