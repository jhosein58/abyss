use abyss_hir::hir::HirExprKind;
use abyss_nexus::{
    nexus::{FileId, NameId, Nexus, SymbolId, TokenId},
    ranges::{HirRange, TokenRange},
};
use abyss_token::kind::TokenKind;

pub struct Parser<'db, const HEADLESS: bool> {
    pub db: &'db mut Nexus,
    pub cursor: u32,
    end: u32,
    file_id: FileId,
}

impl<'a> Parser<'a, true> {
    pub fn indexer(db: &'a mut Nexus, file_id: FileId) -> Self {
        let range = db.file_token_spans.get_copy(file_id);

        Parser {
            db,
            cursor: range.start.0,
            end: range.end.0,
            file_id,
        }
    }

    pub fn index(db: &'a mut Nexus, file_id: FileId) {
        let mut p = Parser::indexer(db, file_id);

        loop {
            if let Some(TokenKind::Eof) = p.peek() {
                break;
            }

            let start = TokenId(p.cursor);

            p.expect(TokenKind::Ident);

            let name = p.db.tokens.text(TokenId(p.cursor - 1));
            let name_id = p.db.interner.intern(name);

            p.expect(TokenKind::ColonColon);

            p.parse_expr(0);
            let end = TokenId(p.cursor - 1);

            p.db.symbol_index
                .insert((p.file_id, name_id), TokenRange { start, end });

            if let Some(_) = p.peek() {
                continue;
            }
            break;
        }
    }
}

impl<'a> Parser<'a, false> {
    pub fn new(db: &'a mut Nexus, file_id: FileId, name_id: NameId) -> Self {
        let range = db.symbol_index.get(&(file_id, name_id)).unwrap().clone();

        Parser {
            db,
            cursor: range.start.0,
            end: range.end.0,
            file_id,
        }
    }

    pub fn parse_top_level(db: &'a mut Nexus, file_id: FileId, name_id: NameId) -> SymbolId {
        let mut p = Parser::new(db, file_id, name_id);

        p.expect(TokenKind::Ident);
        p.expect(TokenKind::ColonColon);

        let text = p.db.tokens.text(TokenId(p.cursor - 2));
        let ident_hir_id = p.db.hir.alloc_ident(p.db.interner.intern(text));

        let body = p.parse_expr(0);

        let symbol_id = p.db.symbols.alloc(ident_hir_id);
        let end =
            p.db.hir
                .alloc_binary(HirExprKind::Binding, ident_hir_id, body);

        p.db.symbol_hir_range.grow_to(p.db.symbols.len());

        p.db.hir_to_symbol.set(ident_hir_id, symbol_id);

        p.db.symbol_hir_range.set(
            symbol_id,
            HirRange {
                start: ident_hir_id,
                end,
            },
        );

        symbol_id
    }
}

impl<'a, const HEADLESS: bool> Parser<'a, HEADLESS> {
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
            panic!("Error: expected {:?}, found {:?}", kind, self.peek())
        }
        self.bump();
    }
}
