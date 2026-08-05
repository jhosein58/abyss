use abyss_diagnostics::span::Span;
use abyss_hir::hir::HirTable;
use abyss_token::stream::TokenStream;

use crate::{
    arena::{Arena, SideTable},
    arena_id,
    storages::{
        hir::storage::HirStorage, interner::InternerStorage, symbols::SymbolStorage,
        tokens::TokenStorage,
    },
};

arena_id!(HirId);
arena_id!(FileId);
arena_id!(IntId);
arena_id!(FloatId);
arena_id!(SpanId);

#[derive(Default)]
pub struct Nexus {
    // Storages
    pub tokens: TokenStorage,
    pub hir: HirStorage,
    pub interner: InternerStorage,
    pub symbols: SymbolStorage,

    pub ints: Arena<IntId, i64>,
    pub floats: Arena<FloatId, f64>,
    pub file_interner: Arena<FileId, String>,
    pub hir_spans: SideTable<HirId, Span>,
    pub hir_files: SideTable<HirId, FileId>,

    pub u32_items: Vec<u32>,
    pub match_arms: Vec<(u32, u32)>,
    pub ranges: Vec<(u32, u32, u32, u32)>,
}

impl Nexus {
    pub fn new() -> Self {
        Self::default()
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

    pub fn add_match_arm(&mut self, pattern: u32, body: u32) -> u32 {
        let id = self.match_arms.len() as u32;
        self.match_arms.push((pattern, body));
        id
    }

    pub fn add_range(&mut self, start: u32, end: u32, step: u32, inclusive: u32) -> u32 {
        let id = self.ranges.len() as u32;
        self.ranges.push((start, end, step, inclusive));
        id
    }
}
