use abyss_nexus::{
    nexus::{FileId, NameId, Nexus, SymbolId, TokenId},
    ranges::HirRange,
    span::Span,
};
use abyss_token::kind::TokenKind;

pub struct Parser<'db> {
    pub db: &'db mut Nexus,
    pub cursor: u32,
    pub end: u32,
    pub file_id: FileId,
}

impl<'a> Parser<'a> {
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
    pub fn optional(&mut self, kind: TokenKind) -> bool {
        if self.peek() == Some(kind) {
            self.bump();
            return true;
        }
        false
    }

    #[inline(always)]
    pub fn expect(&mut self, kind: TokenKind) {
        if self.peek() != Some(kind) {
            panic!("Error: expected {:?}, found {:?}", kind, self.peek())
        }
        self.bump();
    }

    #[inline(always)]
    pub fn span(&self) -> Span {
        let start = self.db.tokens.start(TokenId(self.cursor)) as u32;
        let end = start + self.db.tokens.len(TokenId(self.cursor)) as u32;
        Span { start, end }
    }

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

        let main_span = p.span();
        p.expect(TokenKind::Ident);
        let colon_colon_span = p.span();
        p.expect(TokenKind::ColonColon);

        let text = p.db.tokens.text(TokenId(p.cursor - 2));
        let ident_hir_id = p.db.hir.alloc_ident(p.db.interner.intern(text));
        p.db.hir_spans.set(ident_hir_id, main_span);

        let body = p.parse_expr(0);

        let symbol_id = p.db.symbols.alloc(ident_hir_id);

        let end = p.db.hir.alloc_binding(ident_hir_id, None, Some(body));

        p.db.hir_spans.set(end, colon_colon_span);

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
