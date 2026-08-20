use std::{collections::HashMap, default};

use abyss_lexer::lexer::Lexer;

use crate::{
    arena::{Arena, SideTable},
    arena_id,
    ranges::{HirRange, TokenRange},
    span::Span,
    storages::{
        consts::ConstStorage, diagnostics::DiagnosticStorage, hir::HirStorage,
        interner::InternerStorage, tokens::TokenStorage, types::TypeStorage, unify::UnifyStorage,
    },
};

arena_id!(RawId);
arena_id!(HirId);
arena_id!(NameId);
arena_id!(FileId);
arena_id!(IntId);
arena_id!(FloatId);
arena_id!(SpanId);
arena_id!(ScopeId);
arena_id!(SymbolId);
arena_id!(TokenId);
arena_id!(TypeId);
arena_id!(DiagnosticId);
arena_id!(SlotId);

#[repr(u8)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum SymbolState {
    #[default]
    Unresolved,
    Resolving,
    Resolved,
}

#[derive(Default)]
pub struct Nexus {
    // Primary Storages
    pub tokens: TokenStorage,
    pub hir: HirStorage,
    pub interner: InternerStorage,
    pub types: TypeStorage,
    pub diagnostics: DiagnosticStorage,
    pub unify: UnifyStorage,
    pub consts: ConstStorage,

    // Primitive Stores
    pub ints: Arena<IntId, u64>, // FIXME: change it to string
    pub floats: Arena<FloatId, f64>,
    pub u32_items: Vec<u32>, // FIXME: use ItemId instead of raw u32

    // File & Source Management
    pub sources: Arena<FileId, String>,
    pub file_paths: SideTable<NameId, FileId>,
    pub file_to_name: SideTable<FileId, NameId>,
    pub file_token_spans: SideTable<FileId, TokenRange>,

    // Symbol & Resolution Lookups
    pub symbols: Arena<SymbolId, HirId>,
    pub symbol_index: HashMap<(FileId, NameId), SymbolId>, // PERF: O(1) lookup by (file, name)
    pub symbol_files: SideTable<SymbolId, FileId>,
    pub symbol_token_range: SideTable<SymbolId, TokenRange>,
    pub symbol_hir_range: SideTable<SymbolId, HirRange>,
    pub hir_to_symbol: SideTable<HirId, SymbolId>,
    pub symbol_to_state: SideTable<SymbolId, SymbolState>,

    // Metadata & Side Tables
    pub hir_spans: SideTable<HirId, Span>,
    pub hir_files: SideTable<HirId, FileId>,
}

impl Nexus {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn reserve_for_tokens(&mut self) {
        let len = self.tokens.count();
        self.hir.reserve(len);
        self.hir_spans.grow_to(len);
        self.hir_files.grow_to(len);
        self.hir_to_symbol.grow_to(len);
        self.unify.grow_to(len);
        self.consts.grow_to(len);

        // symbols
        let sym_len = len / 4;
        self.symbols.reserve(sym_len);
        self.symbol_files.grow_to(sym_len);
        self.symbol_token_range.grow_to(sym_len);
        self.symbol_hir_range.grow_to(sym_len);
        self.symbol_to_state.grow_to(sym_len);
    }

    pub fn add_list_flat(&mut self, items: &[u32]) -> u32 {
        let start = self.u32_items.len() as u32;
        self.u32_items.push(items.len() as u32);
        self.u32_items.extend_from_slice(items);
        start
    }

    pub fn get_list_flat(&self, start: u32) -> &[u32] {
        let start = start as usize;
        let len = self.u32_items[start] as usize;
        &self.u32_items[start + 1..start + 1 + len]
    }

    pub fn add_file(&mut self, path: &str, content: String) -> FileId {
        let file_id = self.sources.alloc(content);
        let name_id = self.interner.intern(path);
        self.file_paths.grow_to(self.interner.len());
        self.file_to_name.grow_to(self.sources.len());
        self.file_token_spans.grow_to(self.sources.len());
        self.file_paths.set(name_id, file_id);
        self.file_to_name.set(file_id, name_id);
        file_id
    }

    pub fn lex_file(&mut self, file_id: FileId) {
        let tokens = {
            let content: &str = self.sources.get(file_id);
            let static_content: &'static str = unsafe { std::mem::transmute(content) };
            Lexer::new(static_content).lex()
        };

        let start = TokenId(self.tokens.count() as u32);
        self.tokens.append(tokens);
        let end = TokenId(self.tokens.count() as u32 - 1);
        self.file_token_spans
            .set(file_id, TokenRange { start, end });
        self.reserve_for_tokens();
    }
}
