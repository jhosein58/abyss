use abyss_nexus::{
    nexus::{FileId, NameId, Nexus},
    storages::tokens::TokenId,
};
use abyss_token::{kind::TokenKind, stream::TokenRange};

pub struct Parser<'db> {
    pub db: &'db mut Nexus,
    pub cursor: u32,
    pub is_headless: bool,
    end: u32,
    file_id: FileId,
}

impl<'a> Parser<'a> {
    pub fn new_indexer(db: &'a mut Nexus, file_id: FileId) -> Self {
        let range = db.file_token_spans.get_copy(file_id);

        Parser {
            db,
            cursor: range.start,
            end: range.end,
            is_headless: true,
            file_id,
        }
    }

    pub fn new_parser(db: &'a mut Nexus, file_id: FileId, name_id: NameId) -> Self {
        let range = db.symbol_index.get(&(file_id, name_id)).unwrap().clone();

        Parser {
            db,
            cursor: range.start,
            end: range.end,
            is_headless: false,
            file_id,
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

    pub fn index(db: &'a mut Nexus, file_id: FileId) {
        let mut p = Parser::new_indexer(db, file_id);

        loop {
            if let Some(TokenKind::Eof) = p.peek() {
                break;
            }

            let start = p.cursor;

            p.expect(TokenKind::Ident);

            let name = p.db.tokens.text(TokenId(p.cursor - 1));
            let name_id = p.db.interner.intern(name);

            p.expect(TokenKind::ColonColon);

            p.parse_expr(0);
            let end = p.cursor - 1;

            p.db.symbol_index
                .insert((p.file_id, name_id), TokenRange { start, end });

            if let Some(_) = p.peek() {
                continue;
            }
            break;
        }
    }

    pub fn parse(db: &'a mut Nexus, file_id: FileId, name_id: NameId) {
        let mut p = Self::new_parser(db, file_id, name_id);
        p.parse_expr(0);
    }
}
