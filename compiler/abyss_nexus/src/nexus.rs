use std::collections::HashMap;

use abyss_diagnostics::span::Span;
use abyss_hir::hir::HirTable;
use abyss_lexer::lexer::Lexer;
use abyss_token::stream::{TokenRange, TokenStream};

use crate::{
    arena::{Arena, SideTable},
    arena_id,
    storages::{
        hir::storage::HirStorage, interner::InternerStorage, scopes::ScopeStorage,
        symbols::SymbolStorage, tokens::TokenStorage,
    },
};

arena_id!(HirId);
arena_id!(NameId);
arena_id!(FileId);
arena_id!(IntId);
arena_id!(FloatId);
arena_id!(SpanId);
arena_id!(ScopeId);
arena_id!(SymbolId);

#[derive(Default)]
pub struct Nexus {
    // Primary Storages
    pub tokens: TokenStorage,
    pub hir: HirStorage,
    pub interner: InternerStorage,
    pub symbols: SymbolStorage,
    pub scopes: ScopeStorage,

    // Primitive Stores
    pub ints: Arena<IntId, i64>,
    pub floats: Arena<FloatId, f64>,
    pub u32_items: Vec<u32>,

    // File & Source Management
    pub sources: Arena<FileId, String>,
    pub file_paths: SideTable<NameId, FileId>,
    pub file_token_spans: SideTable<FileId, TokenRange>,

    // Symbol & Resolution Lookups
    pub symbol_index: HashMap<(FileId, NameId), TokenRange>,
    pub symbol_to_hir: SideTable<SymbolId, HirId>,

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
        self.scopes.grow_to(len);
    }

    pub fn set_tokens(&mut self, tokens: TokenStream<'static>) {
        self.tokens.stream = tokens;
    }

    pub fn set_hir(&mut self, table: HirTable) {
        self.hir.set(table);
        self.symbols.init(self.hir.len());
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
        self.file_token_spans.grow_to(self.sources.len());
        self.file_paths.set(name_id, file_id);
        file_id
    }

    pub fn lex_file(&mut self, file_id: FileId) {
        let tokens = {
            let content: &str = self.sources.get(file_id);
            let static_content: &'static str = unsafe { std::mem::transmute(content) };
            Lexer::new(static_content).lex()
        };

        let start = self.tokens.count() as u32;
        self.tokens.append(tokens);
        let end = self.tokens.count() as u32 - 1;
        self.file_token_spans
            .set(file_id, TokenRange { start, end });
        self.reserve_for_tokens();
    }
}
