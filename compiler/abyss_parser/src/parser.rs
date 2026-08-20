use abyss_nexus::{
    arena::ArenaId,
    nexus::{FileId, NameId, Nexus, SymbolId, TokenId},
    ranges::HirRange,
    span::Span,
};
use abyss_token::kind::TokenKind;

use crate::env::ScopeEnv;

pub struct Parser<'db> {
    pub db: &'db mut Nexus,
    pub cursor: u32,
    pub end: u32,
    pub file_id: FileId,
    pub env: ScopeEnv,
    pub toplv_sym: SymbolId,
}

impl<'a> Parser<'a> {
    pub fn new(db: &'a mut Nexus, sym_id: SymbolId) -> Self {
        let range = db.symbol_token_range.get_copy(sym_id);
        let file_id = db.symbol_files.get_copy(sym_id);

        Parser {
            db,
            cursor: range.start.0,
            end: range.end.0,
            file_id,
            env: ScopeEnv::new(),
            toplv_sym: SymbolId::none(),
        }
    }

    #[inline(always)]
    pub fn lookup(&self, name: NameId) -> Option<SymbolId> {
        if let Some(sym) = self.env.lookup_(name) {
            return Some(sym);
        }

        self.db.symbol_index.get(&(self.file_id, name)).cloned() // PERF: hashmaps are slow -_-
    }

    #[inline(always)]
    pub fn tk_id(&self, n: i32) -> TokenId {
        TokenId((self.cursor as i32 + n).max(0) as u32)
    }

    #[inline(always)]
    pub fn peek(&self) -> Option<TokenKind> {
        (self.cursor < self.end).then(|| self.db.tokens.kind(self.tk_id(0)))
    }

    #[inline(always)]
    pub fn peek_n(&self, n: u32) -> Option<TokenKind> {
        let target = self.cursor + n;
        (target < self.end).then(|| self.db.tokens.kind(TokenId(target)))
    }

    pub fn is_eof(&self) -> bool {
        self.cursor >= self.end
    }

    #[inline(always)]
    pub fn prev(&self) -> TokenKind {
        if self.cursor == 0 {
            return TokenKind::Eof;
        }
        self.db.tokens.kind(self.tk_id(-1))
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

    pub fn sync(&mut self) {
        if !self.is_eof() {
            self.bump();
        }

        while !self.is_eof() {
            if self.peek_preceded_by_newline() {
                break;
            }

            let next = self.peek();

            if matches!(next, Some(TokenKind::CBrace) | Some(TokenKind::OBrace)) {
                break;
            }

            if matches!(next, Some(TokenKind::Ident))
                && matches!(self.peek_n(1), Some(TokenKind::ColonColon))
            {
                break;
            }

            self.bump();
        }
    }

    #[inline(always)]
    pub fn optional(&mut self, kind: TokenKind) -> bool {
        if self.peek() == Some(kind) {
            self.bump();
            return true;
        }
        false
    }

    pub fn expect(&mut self, kind: TokenKind) {
        if self.peek() != Some(kind) {
            self.report_unexpected_token(kind);
        } else {
            self.bump();
        }
    }

    #[inline(always)]
    pub fn span(&self) -> Span {
        let start = self.db.tokens.start(TokenId(self.cursor)) as u32;
        let end = start + self.db.tokens.len(TokenId(self.cursor)) as u32;
        Span { start, end }
    }

    #[inline(always)]
    pub fn prev_span(&self) -> Span {
        let start = self.db.tokens.start(TokenId(self.cursor - 1)) as u32;
        let end = start + self.db.tokens.len(TokenId(self.cursor - 1)) as u32;
        Span { start, end }
    } // FIXME: tarkib bayad beshe in dota method ba ham

    pub fn parse_top_level(db: &'a mut Nexus, sym_id: SymbolId) -> SymbolId {
        let mut p = Parser::new(db, sym_id);

        p.toplv_sym = sym_id;

        let main_span = p.span();
        p.expect(TokenKind::Ident);
        let colon_colon_span = p.span();
        p.expect(TokenKind::ColonColon);

        let text = p.db.tokens.text(TokenId(p.cursor - 2));
        let ident_hir_id = p.db.hir.alloc_ident(p.db.interner.intern(text));

        p.db.symbols.data[sym_id.0 as usize] = ident_hir_id; // patch symbol

        p.db.hir_spans.set(ident_hir_id, main_span);

        let body = p.parse_expr(0);

        let end = p.db.hir.alloc_binding(ident_hir_id, None, Some(body));

        p.db.hir_spans.set(end, colon_colon_span);

        p.db.hir_to_symbol.set(ident_hir_id, sym_id);

        p.db.symbol_hir_range.set(
            sym_id,
            HirRange {
                start: ident_hir_id,
                end,
            },
        );

        sym_id
    }
}
