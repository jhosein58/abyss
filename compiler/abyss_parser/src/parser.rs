use abyss_nexus::{nexus::Nexus, storages::tokens::TokenId};
use abyss_token::kind::TokenKind;

pub struct Parser<'db> {
    pub db: &'db mut Nexus,
    pub cursor: u32,
    pub is_headless: bool,
    end: u32,
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

    #[inline(always)]
    pub fn expect(&mut self, kind: TokenKind) {
        if self.peek() != Some(kind) {
            panic!("Error: expected {:?}", kind)
        }
        self.bump();
    }

    pub fn index(&mut self) -> Vec<(u32, u32)> {
        let mut idxs = vec![];

        loop {
            if let Some(TokenKind::Eof) = self.peek() {
                break;
            }

            let start = self.cursor;

            self.expect(TokenKind::Ident);
            self.expect(TokenKind::ColonColon);

            self.parse_expr(0);
            let end = self.cursor - 1;

            idxs.push((start, end));

            if let Some(_) = self.peek() {
                continue;
            }
            break;
        }

        return idxs;
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
